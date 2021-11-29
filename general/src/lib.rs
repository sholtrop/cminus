pub mod logging {
    use env_logger;
    use log::LevelFilter;
    pub fn init_logger(level: LevelFilter) {
        env_logger::builder()
            .format_timestamp(None)
            .filter_level(level)
            .init();
    }
}
