use std::env;
use std::process::Command;

#[derive(Clone, Debug)]
pub struct WorkerConfig {
    pub database_url: String,
    pub proxy_database_url: String,
    pub redis_url: String,
    pub proxy_redis_url: String,
    pub proxy_quarantine_ttl_secs: u64,
    pub queue_stream: String,
    pub queue_group: String,
    pub worker_id: String,
    pub concurrency: usize,
    pub lease_secs: i64,
    pub reclaim_limit: i64,
    pub max_attempts: i32,
    pub artifact_ttl_secs: i64,
    pub cleanup_interval_secs: i64,
    pub cleanup_batch_limit: i64,
    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
    pub s3_endpoint: Option<String>,
    pub s3_access_key_id: Option<String>,
    pub s3_secret_access_key: Option<String>,
    pub s3_force_path_style: bool,
    pub extractor_dir: String,
}

impl WorkerConfig {
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

    fn env_or_default(name: &str, default: impl FnOnce() -> String) -> String {
        Self::optional_env(name).unwrap_or_else(default)
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

    pub fn from_env() -> anyhow::Result<Self> {
        let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "worker".to_string());
        let pid = std::process::id();
        let database_url = Self::resolve_local_database_url(&env::var("DATABASE_URL")?);
        let proxy_database_url = Self::optional_env("PROXY_DATABASE_URL")
            .map(|value| Self::resolve_local_database_url(&value))
            .unwrap_or_else(|| database_url.clone());
        let redis_url = Self::resolve_local_redis_url(&Self::env_or_default("REDIS_URL", || {
            "redis://127.0.0.1:6379".to_string()
        }));
        let proxy_redis_url = Self::optional_env("PROXY_REDIS_URL")
            .map(|value| Self::resolve_local_redis_url(&value))
            .unwrap_or_else(|| redis_url.clone());
        let proxy_quarantine_ttl_secs = env::var("PROXY_QUARANTINE_TTL_SECS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(172_800);
        Ok(Self {
            database_url,
            proxy_database_url,
            redis_url,
            proxy_redis_url,
            proxy_quarantine_ttl_secs,
            queue_stream: Self::env_or_default("MUX_QUEUE_STREAM", || "mux_jobs".to_string()),
            queue_group: Self::env_or_default("MUX_QUEUE_GROUP", || "mux-workers".to_string()),
            worker_id: Self::env_or_default("MUX_WORKER_ID", || format!("{host}-{pid}")),
            concurrency: env::var("MUX_WORKER_CONCURRENCY")
                .ok()
                .and_then(|v| v.parse().ok())
                .filter(|value: &usize| *value > 0)
                .unwrap_or(16),
            lease_secs: env::var("MUX_WORKER_LEASE_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(90),
            reclaim_limit: env::var("MUX_WORKER_RECLAIM_LIMIT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            max_attempts: env::var("MUX_JOB_MAX_ATTEMPTS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
            artifact_ttl_secs: env::var("MUX_ARTIFACT_TTL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(24 * 3600),
            cleanup_interval_secs: env::var("MUX_CLEANUP_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3600),
            cleanup_batch_limit: env::var("MUX_CLEANUP_BATCH_LIMIT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50),
            s3_bucket: Self::optional_env("S3_BUCKET_NAME"),
            s3_region: Self::optional_env("S3_REGION"),
            s3_endpoint: Self::optional_env("S3_ENDPOINT"),
            s3_access_key_id: Self::optional_env("S3_ACCESS_KEY_ID"),
            s3_secret_access_key: Self::optional_env("S3_SECRET_ACCESS_KEY"),
            s3_force_path_style: env::var("S3_FORCE_PATH_STYLE")
                .ok()
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            extractor_dir: env::var("EXTRACTOR_DIR").unwrap_or_else(|_| "./extractors".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::WorkerConfig;

    #[test]
    fn rewrites_local_redis_port_to_compose_default() {
        let rewritten = WorkerConfig::replace_local_service_host_and_port(
            "redis://127.0.0.1:6380",
            "172.18.0.5",
            6379,
        );

        assert_eq!(rewritten, "redis://172.18.0.5:6379");
    }

    #[test]
    fn rewrites_local_database_port_to_compose_default() {
        let rewritten = WorkerConfig::replace_local_service_host_and_port(
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
