use tracing_subscriber::{fmt::time::ChronoUtc, prelude::*, util::SubscriberInitExt};

pub fn init_tracing() {
    let appender = tracing_appender::rolling::daily("./logs", "stargazer.log");

    let timer = ChronoUtc::new("%F %T".to_string());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_timer(timer.clone())
                .with_writer(appender),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
                .with_timer(timer.clone()),
        )
        .init();
}
