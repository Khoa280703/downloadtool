//! Configuration module for API server.
//!
//! Loads configuration from environment variables.

use std::env;
use std::process::Command;

use crate::limit_profiles::backend_limit_profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuxArtifactBackend {
    LocalFs,
    Minio,
    R2,
}

impl MuxArtifactBackend {
    fn from_env(value: Option<String>) -> anyhow::Result<Self> {
        match value
            .unwrap_or_else(|| "localfs".to_string())
            .trim()
            .to_ascii_lowercase()
            .as_str()
        {
            "localfs" => Ok(Self::LocalFs),
            "minio" => Ok(Self::Minio),
            "r2" => Ok(Self::R2),
            other => Err(anyhow::anyhow!(
                "MUX_ARTIFACT_BACKEND must be 'localfs', 'minio', or 'r2', got '{other}'"
            )),
        }
    }

    pub fn storage_backend_name(self) -> &'static str {
        match self {
            Self::LocalFs => "localfs",
            Self::Minio | Self::R2 => "s3",
        }
    }
}

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Port to listen on (default: 3068)
    pub port: u16,
    /// Directory containing TypeScript extractor scripts
    pub extractor_dir: String,
    /// PostgreSQL connection string
    pub database_url: String,
    /// Better Auth shared secret (JWT verify + session signing key)
    pub jwt_secret: String,
    /// Whop webhook HMAC secret
    pub whop_webhook_secret: String,
    /// Toggle rate limiter for /api/extract (default: true)
    pub extract_rate_limit_enabled: bool,
    pub mux_artifact_backend: MuxArtifactBackend,
    pub mux_direct_download: bool,
    pub redis_url: String,
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

    fn env_or_default(name: &str, default: &str) -> String {
        Self::optional_env(name).unwrap_or_else(|| default.to_string())
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
        ps_filter.args(["ps", "-q", "--filter", &format!("name={fallback_container_name}")]);

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

    fn resolve_local_database_url(raw_url: &str) -> String {
        let uses_localhost = raw_url.contains("@127.0.0.1:") || raw_url.contains("@localhost:");
        if !uses_localhost {
            return raw_url.to_string();
        }

        let Some(ip) = Self::resolve_compose_service_ip("postgres", "downloadtool-postgres") else {
            return raw_url.to_string();
        };

        Self::replace_local_loopback_host(raw_url, &ip)
    }

    fn resolve_local_redis_url(raw_url: &str) -> String {
        let uses_localhost = raw_url.contains("//127.0.0.1:") || raw_url.contains("//localhost:");
        if !uses_localhost {
            return raw_url.to_string();
        }

        let Some(ip) = Self::resolve_compose_service_ip("redis", "downloadtool-redis") else {
            return raw_url.to_string();
        };

        Self::replace_local_loopback_host(raw_url, &ip)
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
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL env var is required"))?;
        let database_url = Self::resolve_local_database_url(&database_url);
        let jwt_secret = env::var("BETTER_AUTH_SECRET")
            .map_err(|_| anyhow::anyhow!("BETTER_AUTH_SECRET env var is required"))?;
        let whop_webhook_secret = env::var("WHOP_WEBHOOK_SECRET")
            .map_err(|_| anyhow::anyhow!("WHOP_WEBHOOK_SECRET env var is required"))?;
        let extract_rate_limit_enabled = backend_limit_profile().extract_rate_limit_enabled_value();
        let mux_artifact_backend =
            MuxArtifactBackend::from_env(Self::optional_env("MUX_ARTIFACT_BACKEND"))?;
        let mux_direct_download = env::var("MUX_DIRECT_DOWNLOAD")
            .ok()
            .map(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes"
                )
            })
            .unwrap_or(false);
        let redis_url =
            Self::resolve_local_redis_url(&Self::env_or_default("REDIS_URL", "redis://127.0.0.1:6379"));
        let mux_queue_stream = Self::env_or_default("MUX_QUEUE_STREAM", "mux_jobs");
        let mux_job_max_attempts = env::var("MUX_JOB_MAX_ATTEMPTS")
            .ok()
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
            jwt_secret,
            whop_webhook_secret,
            extract_rate_limit_enabled,
            mux_artifact_backend,
            mux_direct_download,
            redis_url,
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
            jwt_secret: "secret".to_string(),
            whop_webhook_secret: "whop_secret".to_string(),
            extract_rate_limit_enabled: true,
            mux_artifact_backend: MuxArtifactBackend::LocalFs,
            mux_direct_download: false,
            redis_url: "redis://127.0.0.1:6379".to_string(),
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
        assert_eq!(config.jwt_secret, "secret");
        assert_eq!(config.whop_webhook_secret, "whop_secret");
        assert!(config.extract_rate_limit_enabled);
        assert_eq!(config.mux_artifact_backend, MuxArtifactBackend::LocalFs);
        assert!(!config.mux_direct_download);
        assert_eq!(config.redis_url, "redis://127.0.0.1:6379");
        assert_eq!(config.mux_queue_stream, "mux_jobs");
        assert_eq!(config.mux_job_max_attempts, 3);
        assert_eq!(config.mux_file_ticket_ttl_secs, 900);
    }
}
