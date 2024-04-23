use sqlx::PgPool;
mod errors;
mod models;
mod router;
mod routes;
mod templates;

#[cyndra_runtime::main]
async fn main(#[cyndra_shared_db::Postgres] pool: PgPool) -> cyndra_axum::CyndraAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let router = router::init_router(pool);

    Ok(router.into())
}
