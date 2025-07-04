use std::env;

#[tokio::main]
async fn main() {
    let tty_path = env::var("TTY_PATH").expect("env var TTY_PATH should be set");
    let db_path = env::var("DB_PATH").expect("env var DB_PATH should be set");

    ths::app::run(tty_path, db_path).await;
}
