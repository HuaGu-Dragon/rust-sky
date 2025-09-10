mod app;
mod config;
mod controller;
mod database;
mod logger;
mod server;

#[tokio::main]
async fn main() {
    app::run(controller::create_router()).await;
}
