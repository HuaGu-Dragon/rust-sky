mod app;
mod config;
mod controller;
mod database;
mod logger;
mod redis;
mod server;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    app::run(controller::create_router()).await;
}
