mod health;
mod model;
mod persistence;
mod v1;

extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use actix_web::{web, App, HttpServer};
use persistence::postgres_repository::PostgresRepository;
use std::env;
use std::sync::{atomic::AtomicU16, atomic::Ordering, Arc};
use tracing::{self as log};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let tracing = tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    if cfg!(debug_assertions) {
        tracing.pretty().init();
    } else {
        tracing.json().init();
    }

    let port = env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}", port);

    log::info!(%address, "Starting server at {}", address);

    let thread_counter = Arc::new(AtomicU16::new(1));
    let repo = PostgresRepository::from_env()
        .await
        .expect("Repository initialization error");
    let repo = web::Data::new(repo);
    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        log::trace!("Starting thread {}", thread_index);
        App::new()
            .data(thread_index)
            .app_data(repo.clone())
            .configure(v1::service::<PostgresRepository>)
            .configure(health::service)
    })
    .bind(address)?
    .run()
    .await
}
