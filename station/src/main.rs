use std::env;

#[tokio::main]
async fn main() {
    let tty_path = env::var("TTY_PATH").expect("env var TTY_PATH should be set");
    let db_path = env::var("DB_PATH").expect("env var DB_PATH should be set");
    let csv_path = env::var("CSV_PATH").expect("env var CSV_PATH should be set");
    let measurement_periods_seconds = env::var("MEASUREMENT_PERIOD_SECONDS")
        .expect("env var MEASUREMENT_PERIOD_SECONDS should be set")
        .parse::<u64>()
        .expect("env var MEASUREMENT_PERIOD_SECONDS should be positive integer");

    ths_station::service::run(tty_path, db_path, csv_path, measurement_periods_seconds).await;
}
