// src/database/queries.rs - Adicionar esta função

pub async fn find_process_share(
    pool: &SqlitePool,
    process_id: &str,
    supplier_public_key: &str,
) -> Result<Option<ProcessShare>, sqlx::Error> {
    let share = sqlx::query_as!(
        ProcessShare,
        r#"
        SELECT id, process_id, supplier_public_key, stellar_transaction_hash, shared_at
        FROM process_shares 
        WHERE process_id = ? AND supplier_public_key = ?
        "#,
        process_id,
        supplier_public_key
    )
    .fetch_optional(pool)
    .await?;

    Ok(share)
}