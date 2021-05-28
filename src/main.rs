fn main() {
    init_tracing();
    <wasmedge_paratime::Runtime as oasis_runtime_sdk::Runtime>::start();
}

fn init_tracing() {
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

    let base_subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true);
    if cfg!(not(debug_assertions)) {
        base_subscriber.json().with_ansi(false).init();
    } else {
        let subscriber = base_subscriber
            .with_span_events(FmtSpan::CLOSE)
            .without_time()
            .pretty()
            .with_test_writer();
        if std::env::var("VERBOSE_TRACING").unwrap_or_default() != "0" {
            subscriber.try_init().ok();
        } else {
            subscriber.compact().try_init().ok();
        }
    }
}
