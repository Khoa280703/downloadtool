//! Configuration module for API server.
//!
//! Loads configuration from environment variables.

use std::env;
use std::process::Command;

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
}

impl Config {
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

    fn resolve_postgres_container_ip() -> Option<String> {
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
            "postgres",
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
        ps_filter.args(["ps", "-q", "--filter", "name=downloadtool-postgres"]);

        if let Some(container_id) = Self::command_stdout_trimmed(&mut ps_filter)
            .and_then(|stdout| stdout.lines().next().map(str::trim).map(str::to_string))
            .filter(|id| !id.is_empty())
        {
            if let Some(ip) = Self::inspect_container_ip(&container_id) {
                return Some(ip);
            }
        }

        // Last fallback for exact container name.
        Self::inspect_container_ip("downloadtool-postgres")
    }

    fn resolve_local_database_url(raw_url: &str) -> String {
        let uses_localhost =
            raw_url.contains("@127.0.0.1:") || raw_url.contains("@localhost:");
        if !uses_localhost {
            return raw_url.to_string();
        }

        let Some(ip) = Self::resolve_postgres_container_ip() else {
            return raw_url.to_string();
        };

        if raw_url.contains("@127.0.0.1:") {
            return raw_url.replacen("@127.0.0.1:", &format!("@{}:", ip), 1);
        }

        raw_url.replacen("@localhost:", &format!("@{}:", ip), 1)
    }

    /// Load configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `PORT` - Server port (default: 3068)
    /// - `EXTRACTOR_DIR` - Path to extractor scripts (default: "./extractors")
    /// - `DATABASE_URL` - PostgreSQL connection string
    /// - `BETTER_AUTH_SECRET` - Shared Better Auth secret
    /// - `WHOP_WEBHOOK_SECRET` - Whop webhook signing secret
    pub fn from_env() -> anyhow::Result<Self> {
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3068);

        let extractor_dir = env::var("EXTRACTOR_DIR").unwrap_or_else(|_| "./extractors".to_string());
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL env var is required"))?;
        let database_url = Self::resolve_local_database_url(&database_url);
        let jwt_secret = env::var("BETTER_AUTH_SECRET")
            .map_err(|_| anyhow::anyhow!("BETTER_AUTH_SECRET env var is required"))?;
        let whop_webhook_secret = env::var("WHOP_WEBHOOK_SECRET")
            .map_err(|_| anyhow::anyhow!("WHOP_WEBHOOK_SECRET env var is required"))?;

        Ok(Self {
            port,
            extractor_dir,
            database_url,
            jwt_secret,
            whop_webhook_secret,
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
        };

        assert_eq!(config.port, 3068);
        assert_eq!(config.extractor_dir, "./extractors");
        assert!(config.database_url.starts_with("postgres://"));
        assert_eq!(config.jwt_secret, "secret");
        assert_eq!(config.whop_webhook_secret, "whop_secret");
    }
}
