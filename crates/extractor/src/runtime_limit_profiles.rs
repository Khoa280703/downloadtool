use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct RuntimeLimitProfiles {
    local: RuntimeLimitProfileEntry,
    production: RuntimeLimitProfileEntry,
}

#[derive(Debug, Deserialize)]
struct RuntimeLimitProfileEntry {
    backend: BackendExtractorLimitProfile,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BackendExtractorLimitProfile {
    pub extract_cache_ttl_secs: Option<u64>,
    pub stream_proxy_cache_ttl_secs: Option<u64>,
}

static LIMIT_PROFILES: OnceLock<RuntimeLimitProfiles> = OnceLock::new();
static BACKEND_ACTIVE_PROFILE: OnceLock<BackendExtractorLimitProfile> = OnceLock::new();

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

pub fn extractor_limit_profile() -> &'static BackendExtractorLimitProfile {
    BACKEND_ACTIVE_PROFILE.get_or_init(|| {
        let profiles = LIMIT_PROFILES.get_or_init(parse_profiles);
        match current_profile_name() {
            "production" => profiles.production.backend.clone(),
            _ => profiles.local.backend.clone(),
        }
    })
}

impl BackendExtractorLimitProfile {
    pub fn extract_cache_ttl_secs_value(&self, default: u64) -> u64 {
        self.extract_cache_ttl_secs
            .filter(|value| *value > 0)
            .unwrap_or(default)
    }

    pub fn stream_proxy_cache_ttl_secs_value(&self, default: u64) -> u64 {
        self.stream_proxy_cache_ttl_secs
            .filter(|value| *value > 0)
            .unwrap_or(default)
    }
}
