use sqlx::PgPool;
mod errors;
mod models;
mod router;
mod routes;
mod templates;

#[cyndra_runtime::main]
async fn main(#[cyndra_shared_db::Postgres] db: PgPool) -> cyndra_axum::CyndraAxum {
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Looks like something went wrong with migrations :(");

    let router = router::init_router(db);

    Ok(router.into())
}
