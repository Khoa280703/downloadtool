//! Configuration module for API server.
//!
//! Loads configuration from environment variables.

use std::env;
use std::process::Command;

use crate::limit_profiles::backend_limit_profile;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Port to listen on (default: 3068)
    pub port: u16,
    /// Directory containing TypeScript extractor scripts
    pub extractor_dir: String,
    /// PostgreSQL connection string
    pub database_url: String,
    /// Shared PostgreSQL connection string for proxy inventory/health.
    pub proxy_database_url: String,
    /// Better Auth shared secret (JWT verify + session signing key)
    pub jwt_secret: String,
    /// Whop webhook HMAC secret
    pub whop_webhook_secret: String,
    /// Toggle rate limiter for /api/extract (default: true)
    pub extract_rate_limit_enabled: bool,
    pub redis_url: String,
    pub proxy_redis_url: String,
    pub proxy_quarantine_ttl_secs: u64,
    pub mux_queue_stream: String,
    pub mux_job_max_attempts: i32,
    pub mux_file_ticket_ttl_secs: u64,
    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
    pub s3_endpoint: Option<String>,
    pub s3_access_key_id: Option<String>,
    pub s3_secret_access_key: Option<String>,
    pub s3_force_path_style: bool,
}

impl Config {
    fn first_present_env(names: &[&str]) -> Option<String> {
        names.iter().find_map(|name| Self::optional_env(name))
    }

    fn optional_env(name: &str) -> Option<String> {
        env::var(name).ok().and_then(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
    }

    fn first_env_or_default(names: &[&str], default: &str) -> String {
        Self::first_present_env(names).unwrap_or_else(|| default.to_string())
    }

    fn command_stdout_trimmed(command: &mut Command) -> Option<String> {
        let output = command.output().ok()?;
        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if stdout.is_empty() {
            return None;
        }

        Some(stdout)
    }

    fn inspect_container_ip(container_ref: &str) -> Option<String> {
        let mut inspect = Command::new("docker");
        inspect.args([
            "inspect",
            "-f",
            "{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}",
            container_ref,
        ]);
        Self::command_stdout_trimmed(&mut inspect)
    }

    fn resolve_compose_service_ip(service: &str, fallback_container_name: &str) -> Option<String> {
        // Preferred path: ask docker compose for postgres service container id.
        let mut compose_ps = Command::new("docker");
        compose_ps.args([
            "compose",
            "--project-directory",
            ".",
            "--env-file",
            ".env",
            "-f",
            "docker/docker-compose.server.yml",
            "ps",
            "-q",
            service,
        ]);

        if let Some(container_id) = Self::command_stdout_trimmed(&mut compose_ps)
            .and_then(|stdout| stdout.lines().next().map(str::trim).map(str::to_string))
            .filter(|id| !id.is_empty())
        {
            if let Some(ip) = Self::inspect_container_ip(&container_id) {
                return Some(ip);
            }
        }

        // Fallback for environments where container names are prefixed by project id.
        let mut ps_filter = Command::new("docker");
        ps_filter.args([
            "ps",
            "-q",
            "--filter",
            &format!("name={fallback_container_name}"),
        ]);

        if let Some(container_id) = Self::command_stdout_trimmed(&mut ps_filter)
            .and_then(|stdout| stdout.lines().next().map(str::trim).map(str::to_string))
            .filter(|id| !id.is_empty())
        {
            if let Some(ip) = Self::inspect_container_ip(&container_id) {
                return Some(ip);
            }
        }

        // Last fallback for exact container name.
        Self::inspect_container_ip(fallback_container_name)
    }

    fn replace_local_loopback_host(raw_url: &str, ip: &str) -> String {
        if raw_url.contains("@127.0.0.1:") {
            return raw_url.replacen("@127.0.0.1:", &format!("@{}:", ip), 1);
        }
        if raw_url.contains("@localhost:") {
            return raw_url.replacen("@localhost:", &format!("@{}:", ip), 1);
        }
        if raw_url.contains("//127.0.0.1:") {
            return raw_url.replacen("//127.0.0.1:", &format!("//{}:", ip), 1);
        }

        raw_url.replacen("//localhost:", &format!("//{}:", ip), 1)
    }

    fn replace_local_service_host_and_port(raw_url: &str, ip: &str, port: u16) -> String {
        let Some(scheme_end) = raw_url.find("//").map(|idx| idx + 2) else {
            return Self::replace_local_loopback_host(raw_url, ip);
        };
        let authority_end = raw_url[scheme_end..]
            .find(['/', '?', '#'])
            .map(|idx| scheme_end + idx)
            .unwrap_or(raw_url.len());
        let authority = &raw_url[scheme_end..authority_end];
        let (userinfo, host_port) = authority
            .rsplit_once('@')
            .map(|(userinfo, host_port)| (Some(userinfo), host_port))
            .unwrap_or((None, authority));

        let is_local_loopback =
            host_port.starts_with("127.0.0.1") || host_port.starts_with("localhost");
        if !is_local_loopback {
            return raw_url.to_string();
        }

        let rewritten_authority = match userinfo {
            Some(userinfo) => format!("{userinfo}@{ip}:{port}"),
            None => format!("{ip}:{port}"),
        };

        format!(
            "{}{}{}",
            &raw_url[..scheme_end],
            rewritten_authority,
            &raw_url[authority_end..]
        )
    }

    fn resolve_local_database_url(raw_url: &str) -> String {
        let uses_localhost = raw_url.contains("@127.0.0.1:") || raw_url.contains("@localhost:");
        if !uses_localhost {
            return raw_url.to_string();
        }

        let (service, fallback_container_name) =
            if raw_url.contains("@127.0.0.1:15432") || raw_url.contains("@localhost:15432") {
                ("shared-proxy-postgres", "shared-proxy-postgres")
            } else {
                ("postgres", "downloadtool-postgres")
            };

        let Some(ip) = Self::resolve_compose_service_ip(service, fallback_container_name) else {
            return raw_url.to_string();
        };

        Self::replace_local_service_host_and_port(raw_url, &ip, 5432)
    }

    fn resolve_local_redis_url(raw_url: &str) -> String {
        let uses_localhost = raw_url.contains("//127.0.0.1:") || raw_url.contains("//localhost:");
        if !uses_localhost {
            return raw_url.to_string();
        }

        let (service, fallback_container_name) =
            if raw_url.contains("//127.0.0.1:6381") || raw_url.contains("//localhost:6381") {
                ("shared-proxy-redis", "shared-proxy-redis")
            } else {
                ("redis", "downloadtool-redis")
            };

        let Some(ip) = Self::resolve_compose_service_ip(service, fallback_container_name) else {
            return raw_url.to_string();
        };

        Self::replace_local_service_host_and_port(raw_url, &ip, 6379)
    }

    /// Load configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `PORT` - Server port (default: 3068)
    /// - `EXTRACTOR_DIR` - Path to extractor scripts (default: "./extractors")
    /// - `DATABASE_URL` - PostgreSQL connection string
    /// - `BETTER_AUTH_SECRET` - Shared Better Auth secret
    /// - `WHOP_WEBHOOK_SECRET` - Whop webhook signing secret
    /// - `config/runtime-limit-profiles.json` - Backend limits profile (local/production)
    pub fn from_env() -> anyhow::Result<Self> {
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3068);

        let extractor_dir =
            env::var("EXTRACTOR_DIR").unwrap_or_else(|_| "./extractors".to_string());
        let database_url = Self::first_present_env(&["INTERNAL_DATABASE_URL", "DATABASE_URL"])
            .ok_or_else(|| anyhow::anyhow!("DATABASE_URL env var is required"))?;
        let database_url = Self::resolve_local_database_url(&database_url);
        let proxy_database_url =
            Self::first_present_env(&["INTERNAL_PROXY_DATABASE_URL", "PROXY_DATABASE_URL"])
                .map(|value| Self::resolve_local_database_url(&value))
                .unwrap_or_else(|| database_url.clone());
        let jwt_secret = env::var("BETTER_AUTH_SECRET")
            .map_err(|_| anyhow::anyhow!("BETTER_AUTH_SECRET env var is required"))?;
        let whop_webhook_secret = env::var("WHOP_WEBHOOK_SECRET")
            .map_err(|_| anyhow::anyhow!("WHOP_WEBHOOK_SECRET env var is required"))?;
        let extract_rate_limit_enabled = backend_limit_profile().extract_rate_limit_enabled_value();
        let redis_url = Self::resolve_local_redis_url(&Self::first_env_or_default(
            &["INTERNAL_REDIS_URL", "REDIS_URL"],
            "redis://127.0.0.1:6379",
        ));
        let proxy_redis_url =
            Self::first_present_env(&["INTERNAL_PROXY_REDIS_URL", "PROXY_REDIS_URL"])
                .map(|value| Self::resolve_local_redis_url(&value))
                .unwrap_or_else(|| redis_url.clone());
        let proxy_quarantine_ttl_secs = env::var("PROXY_QUARANTINE_TTL_SECS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(172_800);
        let mux_queue_stream = Self::first_env_or_default(
            &["INTERNAL_MUX_QUEUE_STREAM", "MUX_QUEUE_STREAM"],
            "mux_jobs",
        );
        let mux_job_max_attempts =
            Self::first_present_env(&["INTERNAL_MUX_JOB_MAX_ATTEMPTS", "MUX_JOB_MAX_ATTEMPTS"])
                .and_then(|value| value.parse().ok())
                .unwrap_or(3);
        let mux_file_ticket_ttl_secs = env::var("MUX_FILE_TICKET_TTL_SECS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(900);
        let s3_bucket = Self::optional_env("S3_BUCKET_NAME");
        let s3_region = Self::optional_env("S3_REGION");
        let s3_endpoint = Self::optional_env("S3_ENDPOINT");
        let s3_access_key_id = Self::optional_env("S3_ACCESS_KEY_ID");
        let s3_secret_access_key = Self::optional_env("S3_SECRET_ACCESS_KEY");
        let s3_force_path_style = env::var("S3_FORCE_PATH_STYLE")
            .ok()
            .map(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes"
                )
            })
            .unwrap_or(false);

        Ok(Self {
            port,
            extractor_dir,
            database_url,
            proxy_database_url,
            jwt_secret,
            whop_webhook_secret,
            extract_rate_limit_enabled,
            redis_url,
            proxy_redis_url,
            proxy_quarantine_ttl_secs,
            mux_queue_stream,
            mux_job_max_attempts,
            mux_file_ticket_ttl_secs,
            s3_bucket,
            s3_region,
            s3_endpoint,
            s3_access_key_id,
            s3_secret_access_key,
            s3_force_path_style,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_fields() {
        let config = Config {
            port: 3068,
            extractor_dir: "./extractors".to_string(),
            database_url: "postgres://user:pass@localhost:5432/db".to_string(),
            proxy_database_url: "postgres://user:pass@localhost:5432/db".to_string(),
            jwt_secret: "secret".to_string(),
            whop_webhook_secret: "whop_secret".to_string(),
            extract_rate_limit_enabled: true,
            redis_url: "redis://127.0.0.1:6379".to_string(),
            proxy_redis_url: "redis://127.0.0.1:6379".to_string(),
            proxy_quarantine_ttl_secs: 172_800,
            mux_queue_stream: "mux_jobs".to_string(),
            mux_job_max_attempts: 3,
            mux_file_ticket_ttl_secs: 900,
            s3_bucket: None,
            s3_region: None,
            s3_endpoint: None,
            s3_access_key_id: None,
            s3_secret_access_key: None,
            s3_force_path_style: false,
        };

        assert_eq!(config.port, 3068);
        assert_eq!(config.extractor_dir, "./extractors");
        assert!(config.database_url.starts_with("postgres://"));
        assert!(config.proxy_database_url.starts_with("postgres://"));
        assert_eq!(config.jwt_secret, "secret");
        assert_eq!(config.whop_webhook_secret, "whop_secret");
        assert!(config.extract_rate_limit_enabled);
        assert_eq!(config.redis_url, "redis://127.0.0.1:6379");
        assert_eq!(config.proxy_redis_url, "redis://127.0.0.1:6379");
        assert_eq!(config.proxy_quarantine_ttl_secs, 172_800);
        assert_eq!(config.mux_queue_stream, "mux_jobs");
        assert_eq!(config.mux_job_max_attempts, 3);
        assert_eq!(config.mux_file_ticket_ttl_secs, 900);
    }

    #[test]
    fn rewrites_local_redis_port_to_compose_default() {
        let rewritten = Config::replace_local_service_host_and_port(
            "redis://127.0.0.1:6380",
            "172.18.0.5",
            6379,
        );

        assert_eq!(rewritten, "redis://172.18.0.5:6379");
    }

    #[test]
    fn rewrites_local_database_port_to_compose_default() {
        let rewritten = Config::replace_local_service_host_and_port(
            "postgres://user:pass@127.0.0.1:15432/downloadtool",
            "172.18.0.10",
            5432,
        );

        assert_eq!(
            rewritten,
            "postgres://user:pass@172.18.0.10:5432/downloadtool"
        );
    }
}
