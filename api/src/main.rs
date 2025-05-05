use tokio::sync::mpsc::{self, Receiver, Sender};

use api::{
    services::transcribe::{TranscribeTask, transcribe_worker_loop},
    web,
};
use tokio::task;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let (tx, rx): (Sender<TranscribeTask>, Receiver<TranscribeTask>) = mpsc::channel(100);
    task::spawn(transcribe_worker_loop(rx));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    web::app::serve(tx).await
}
