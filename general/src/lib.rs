pub mod logging {
    use env_logger;
    pub fn init_logger() {
        env_logger::builder().format_timestamp(None).init();
    }
}
