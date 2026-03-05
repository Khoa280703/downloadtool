use serde::Deserialize;
use std::sync::OnceLock;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct RuntimeLimitProfiles {
    local: RuntimeLimitProfileEntry,
    production: RuntimeLimitProfileEntry,
}

#[derive(Debug, Deserialize)]
struct RuntimeLimitProfileEntry {
    backend: BackendLimitProfile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BackendLimitProfile {
    pub extract_rate_limit_enabled: Option<bool>,
    pub stream_max_concurrent: Option<usize>,
    pub mux_max_concurrent: Option<usize>,
    pub mux_preflight_timeout_secs: Option<u64>,
    pub stream_url_refresh_max_attempts: Option<usize>,
    pub mux_url_refresh_max_attempts: Option<usize>,
    pub mux_job_max_workers: Option<usize>,
    pub mux_job_queue_capacity: Option<usize>,
    pub mux_job_estimated_runtime_secs: Option<u64>,
    pub mux_job_max_estimated_wait_secs: Option<u64>,
    pub mux_job_timeout_secs: Option<u64>,
    pub mux_job_ttl_secs: Option<u64>,
    pub mux_job_temp_file_ttl_secs: Option<u64>,
    pub mux_job_cleanup_interval_secs: Option<u64>,
    pub mux_job_output_dir: Option<String>,
}

static LIMIT_PROFILES: OnceLock<RuntimeLimitProfiles> = OnceLock::new();
static BACKEND_ACTIVE_PROFILE: OnceLock<BackendLimitProfile> = OnceLock::new();

fn parse_profiles() -> RuntimeLimitProfiles {
    let raw = include_str!("../../../config/runtime-limit-profiles.json");
    json5::from_str(raw)
        .expect("config/runtime-limit-profiles.json must be valid JSON5 (supports // comments)")
}

fn current_profile_name() -> &'static str {
    if cfg!(debug_assertions) {
        "local"
    } else {
        "production"
    }
}

pub fn backend_limit_profile() -> &'static BackendLimitProfile {
    BACKEND_ACTIVE_PROFILE.get_or_init(|| {
        let profiles = LIMIT_PROFILES.get_or_init(parse_profiles);
        match current_profile_name() {
            "production" => profiles.production.backend.clone(),
            _ => profiles.local.backend.clone(),
        }
    })
}

impl BackendLimitProfile {
    pub fn extract_rate_limit_enabled_value(&self) -> bool {
        self.extract_rate_limit_enabled.unwrap_or(false)
    }

    pub fn stream_max_concurrent_value(&self) -> Option<usize> {
        self.stream_max_concurrent.filter(|value| *value > 0)
    }

    pub fn mux_max_concurrent_value(&self) -> Option<usize> {
        self.mux_max_concurrent.filter(|value| *value > 0)
    }

    pub fn mux_preflight_timeout_value(&self) -> Option<Duration> {
        self.mux_preflight_timeout_secs
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
    }

    pub fn stream_url_refresh_max_attempts_value(&self) -> Option<usize> {
        self.stream_url_refresh_max_attempts
            .filter(|value| *value > 0)
    }

    pub fn mux_url_refresh_max_attempts_value(&self) -> Option<usize> {
        self.mux_url_refresh_max_attempts.filter(|value| *value > 0)
    }

    pub fn mux_job_max_workers_value(&self) -> usize {
        self.mux_job_max_workers
            .filter(|value| *value > 0)
            .unwrap_or_else(|| {
                std::thread::available_parallelism()
                    .map(|value| value.get().saturating_mul(8))
                    .unwrap_or(64)
                    .max(1)
            })
    }

    pub fn mux_job_queue_capacity_value(&self) -> Option<usize> {
        self.mux_job_queue_capacity.filter(|value| *value > 0)
    }

    pub fn mux_job_estimated_runtime_secs_value(&self) -> u64 {
        self.mux_job_estimated_runtime_secs
            .filter(|value| *value > 0)
            .unwrap_or(210)
    }

    pub fn mux_job_max_estimated_wait_secs_value(&self) -> Option<u64> {
        self.mux_job_max_estimated_wait_secs
            .filter(|value| *value > 0)
    }

    pub fn mux_job_timeout_value(&self) -> Option<Duration> {
        self.mux_job_timeout_secs
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
    }

    pub fn mux_job_ttl_value(&self) -> Option<Duration> {
        self.mux_job_ttl_secs
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
    }

    pub fn mux_job_temp_file_ttl_value(&self) -> Option<Duration> {
        self.mux_job_temp_file_ttl_secs
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
    }

    pub fn mux_job_cleanup_interval_value(&self) -> Option<Duration> {
        self.mux_job_cleanup_interval_secs
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
    }

    pub fn mux_job_output_dir_value(&self) -> String {
        self.mux_job_output_dir
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("/tmp/downloadtool-mux-jobs")
            .to_string()
    }
}
