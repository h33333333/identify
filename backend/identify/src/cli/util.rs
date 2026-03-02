use tokio::signal;

pub async fn cancellation_signal() {
    let _ = signal::ctrl_c().await;
}
