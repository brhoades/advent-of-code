use log::LevelFilter;
use std::sync::Once;

static INIT: Once = Once::new();

/// Setup function that is only run once, even if called multiple times.
pub fn init() {
    INIT.call_once(|| {
        pretty_env_logger::formatted_timed_builder()
            .filter_level(LevelFilter::Trace)
            .is_test(cfg!(test))
            .init();
    });
}
