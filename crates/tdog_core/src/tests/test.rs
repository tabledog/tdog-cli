
fn init_log_output() {
    // let mut builder = Builder::init_from();
    // builder.target(env_logger::Target::Stdout); // Stderr by default?
    // builder.init();
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info")
    );
}


#[cfg(test)]
mod test {

}
