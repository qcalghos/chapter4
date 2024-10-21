use chapter4::configuration::get_configuration;
use chapter4::startup::run;
use chapter4::telemetry;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
// use env_logger::Env;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = telemetry::get_subscriber("chapter4".into(), "info".into(),std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    //连接数据库
    let connection_pool = PgPool::connect(&connection_string.expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind  random port");
    run(listener, connection_pool)?.await
}
