pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::init();
}

#[derive(Debug, Default, Clone, Hash)]
pub struct UniqueSymbol {
    pub exchange: String,
    pub symbol: String,
}
