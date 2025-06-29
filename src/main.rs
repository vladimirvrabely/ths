use std::env;

#[tokio::main]
async fn main() {
    let tty_path = env::var("TTY_PATH").expect("env var TTY_PATH should be set");
    let measurement_file =
        env::var("MEASUREMENT_FILE").expect("env var MEASUREMENT_FILE should be set");

    ths::app::run(tty_path, measurement_file).await;
}
