use somanyfeeds_server::{app, env::load_env_num, worker::{Worker, WorkerSettings}};

#[tokio::main]
async fn main() {
    let worker_interval_seconds = load_env_num("WORKER_INTERVAL_SECONDS", 30);
    let worker_settings = WorkerSettings::new(worker_interval_seconds);

    Worker::new(worker_settings).start();

    let port: u16 = load_env_num("PORT", 3000);
    let server_addr = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(&server_addr)
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}
