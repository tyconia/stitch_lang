use stitch_lang::prelude::*;
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    // Set up the subscriber to capture logs at the `DEBUG` level and higher
    tracing_subscriber::fmt()
        .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
        .init();

    let (rt, tx) = Runtime::new();

    let rt_task = tokio::spawn(async move {
        let _ = rt.run().await;
    });

    let tx_clone = tx.clone();
    let startup_task = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        let _ = tx_clone.send(RuntimeReception::Startup).await;
    });

    // TODO: sigterm. refer to https://docs.rs/tokio/latest/tokio/signal/unix/struct.Signal.html
    let sig_int = tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Err(e) => {
                log::error!("Error handling Ctrl+C: {}", e);
                return;
            }
            Ok(()) => {
                tx.send(RuntimeReception::Shutdown)
                    .await
                    .expect("mpsc channel missing");
            }
        }
    });

    tokio::join!(rt_task, sig_int, startup_task).0
}
