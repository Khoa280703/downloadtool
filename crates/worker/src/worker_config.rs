use std::env;
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone, Debug)]
pub struct WorkerConfig {
    pub database_url: String,
    pub redis_url: String,
    pub queue_stream: String,
    pub queue_group: String,
    pub worker_id: String,
    pub lease_secs: i64,
    pub reclaim_limit: i64,
    pub max_attempts: i32,
    pub artifact_backend: String,
    pub artifact_dir: PathBuf,
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

    pub fn from_env() -> anyhow::Result<Self> {
        let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "worker".to_string());
        let pid = std::process::id();
        let database_url = Self::resolve_local_database_url(&env::var("DATABASE_URL")?);
        let redis_url = Self::resolve_local_redis_url(&Self::env_or_default("REDIS_URL", || {
            "redis://127.0.0.1:6379".to_string()
        }));
        Ok(Self {
            database_url,
            redis_url,
            queue_stream: Self::env_or_default("MUX_QUEUE_STREAM", || "mux_jobs".to_string()),
            queue_group: Self::env_or_default("MUX_QUEUE_GROUP", || "mux-workers".to_string()),
            worker_id: Self::env_or_default("MUX_WORKER_ID", || format!("{host}-{pid}")),
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
            artifact_backend: env::var("MUX_ARTIFACT_BACKEND")
                .unwrap_or_else(|_| "localfs".to_string()),
            artifact_dir: PathBuf::from(Self::env_or_default("MUX_JOB_OUTPUT_DIR", || {
                "/tmp/downloadtool-worker-artifacts".to_string()
            })),
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
