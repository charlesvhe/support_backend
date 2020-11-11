use actix_web::{App, HttpServer};
use anyhow::Result;

use support_backend::Pool;

#[actix_web::main]
async fn main() -> Result<()> {
    let pool = Pool::connect("sqlite:support.db").await?;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(support_backend::config)
    })
        .bind("0.0.0.0:8080")?
        .run().await?;

    Ok(())
}
