fn main() {
    log::info!("message with info level");
    log::error!("message with error level");
    log::debug!("message with debug level");
}

// Wont produce anything with just cargo run, to display we need a logger,
