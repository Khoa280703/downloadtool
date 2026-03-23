use std::sync::{Arc, OnceLock};
use std::time::Duration;

use anyhow::Context;
use sqlx::PgPool;
use tracing::{info, warn};

use crate::proxy_health_store::ProxyHealthStore;
use crate::proxy_inventory_store::ProxyInventoryStore;
use crate::proxy_pool::ProxyPool;
use crate::proxy_quarantine::{load_quarantined_proxies, proxy_quarantine_file_from_env};

const PROXY_SYNC_INTERVAL_SECS: u64 = 5;

static GLOBAL_PROXY_POOL: OnceLock<Arc<ProxyPool>> = OnceLock::new();

pub async fn init_global_proxy_pool(
    db_pool: PgPool,
    redis_url: &str,
    quarantine_ttl_secs: u64,
) -> anyhow::Result<()> {
    if GLOBAL_PROXY_POOL.get().is_some() {
        return Ok(());
    }

    let inventory_store = ProxyInventoryStore::new(db_pool.clone());
    let redis_store = ProxyHealthStore::new(redis_url)
        .context("failed to initialize shared redis proxy health store")?;
    let proxy_count_before_seed = inventory_store.count_all().await.unwrap_or_default();
    info!("Initializing proxy runtime from database inventory");

    let quarantine_file = proxy_quarantine_file_from_env();
    if proxy_count_before_seed == 0 {
        let file_quarantined = quarantine_file
            .as_ref()
            .map(|path| load_quarantined_proxies(path))
            .unwrap_or_default();
        if !file_quarantined.is_empty() {
            let migrated = inventory_store
                .mark_urls_quarantined(
                    &file_quarantined.into_iter().collect::<Vec<_>>(),
                    "migrated-file-quarantine",
                )
                .await?;
            if migrated > 0 {
                info!(
                    migrated,
                    "Migrated proxy quarantine file into postgres inventory"
                );
            }
        }
    }

    let released_proxies = inventory_store
        .release_expired_quarantined(quarantine_ttl_secs)
        .await?;
    clear_released_runtime_health(&redis_store, &released_proxies).await;

    let records = inventory_store.list_runtime_records().await?;
    let pool = Arc::new(ProxyPool::new_with_runtime(
        records,
        quarantine_file,
        Some(db_pool),
        Some(redis_store.clone()),
    ));

    pool.refresh_from_runtime().await?;

    if GLOBAL_PROXY_POOL.set(Arc::clone(&pool)).is_ok() {
        spawn_background_sync(pool, quarantine_ttl_secs);
    }

    Ok(())
}

pub fn global_proxy_pool() -> Option<Arc<ProxyPool>> {
    GLOBAL_PROXY_POOL.get().cloned()
}

fn spawn_background_sync(pool: Arc<ProxyPool>, quarantine_ttl_secs: u64) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(PROXY_SYNC_INTERVAL_SECS));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let inventory_store = pool.inventory_pool().map(ProxyInventoryStore::new);
        let health_store = pool.health_store();

        loop {
            interval.tick().await;
            if let Some(store) = inventory_store.as_ref() {
                match store.release_expired_quarantined(quarantine_ttl_secs).await {
                    Ok(released_proxies) => {
                        if let Some(health_store) = health_store.as_ref() {
                            clear_released_runtime_health(health_store, &released_proxies).await;
                        }
                    }
                    Err(error) => {
                        warn!(err = %error, "Failed to release expired quarantined proxies");
                    }
                }
            }
            if let Err(error) = pool.refresh_from_runtime().await {
                warn!(err = %error, "Failed to sync proxy runtime state");
            }
        }
    });
}

async fn clear_released_runtime_health(health_store: &ProxyHealthStore, proxy_urls: &[String]) {
    for proxy_url in proxy_urls {
        if let Err(error) = health_store.clear_runtime_health(proxy_url).await {
            warn!(
                err = %error,
                proxy = %proxy_url,
                "Failed to clear runtime health for released proxy"
            );
        }
    }
}
