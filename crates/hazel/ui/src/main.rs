#[macro_use]
extern crate lazy_static;

#[cfg(test)]
pub use tracing_test;

pub mod ui;

use hazel_engine::uci;

// NOTE: No need to mutation test the main wrapper.
#[tokio::main]
async fn main() {
    tracing::info!("Welcome to Hazel.");

    // console_subscriber::init();

    // TODO: actually parse arguments
    let headless : bool = false;

    if headless {
        // Log to STDERR
        let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stderr());
        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .init();
        let _ = uci::run().await;
    } else {
        // // Log to a file
        // let (non_blocking, _guard) = tracing_appender::non_blocking(std::fs::File::create("hazel.log").unwrap());
        // let subscriber = fmt::Subscriber::builder()
        //     .with_env_filter(EnvFilter::from_default_env())
        //     .finish();
        let _ = ui::run().await;
    };
}



// initialize log to file at in /var/log/hazel.log or eventually some configurable spot
