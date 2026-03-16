use anyhow::Context;
use sqlx::PgPool;

const CREATE_PROXY_INVENTORY_SQL: &str =
    include_str!("../migrations/0001_create_proxy_inventory.sql");
const ADD_PROXY_QUALITY_SCORING_SQL: &str =
    include_str!("../migrations/0002_add_proxy_quality_scoring.sql");

pub async fn ensure_proxy_schema(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::raw_sql(CREATE_PROXY_INVENTORY_SQL)
        .execute(pool)
        .await
        .context("failed to create proxy inventory schema")?;

    sqlx::raw_sql(ADD_PROXY_QUALITY_SCORING_SQL)
        .execute(pool)
        .await
        .context("failed to add proxy quality scoring schema")?;

    Ok(())
}
