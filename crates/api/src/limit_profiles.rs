use serde::Deserialize;
use std::sync::OnceLock;

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
    pub stream_url_refresh_max_attempts: Option<usize>,
    pub playlist_mux_idle_timeout_secs: Option<u64>,
    pub playlist_job_max_concurrent_items: Option<usize>,
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

    pub fn stream_url_refresh_max_attempts_value(&self) -> Option<usize> {
        self.stream_url_refresh_max_attempts
            .filter(|value| *value > 0)
    }

    pub fn playlist_mux_idle_timeout_secs_value(&self) -> u64 {
        self.playlist_mux_idle_timeout_secs
            .filter(|value| *value > 0)
            .unwrap_or(300)
    }

    pub fn playlist_job_max_concurrent_items_value(&self) -> usize {
        self.playlist_job_max_concurrent_items
            .filter(|value| *value > 0)
            .unwrap_or(1)
    }
}
