use std::io;

use clap::Parser;
#[cfg(not(feature = "log"))]
use fire_alarm_service::Args as Cli;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    #[cfg(feature = "log")]
    let args = {
        env_logger::Builder::new()
            .filter_level(args.verbosity.log_level_filter())
            .init();
        args.args
    };

    let incidents: Vec<_> = serde_json::from_reader(io::BufReader::new(io::stdin()))
        .expect("Failed to parse incidents");

    fire_alarm_service::run(
        args.timestamp,
        sea_orm::Database::connect(args.database),
        incidents,
        &args.index,
        args.username,
        args.address,
        args.password,
        &args.relay,
    )
    .await
    .expect("Failed to run Fire-Alarm Service")
}

#[cfg(feature = "log")]
#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    args: fire_alarm_service::Args,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

#[cfg(test)]
mod test {
    use std::env;

    use tokio::io;

    async fn fetch_incidents(
        path: impl AsRef<std::path::Path>,
    ) -> io::Result<Vec<fire_alarm_service::Incident>> {
        use io::AsyncReadExt;

        let file = tokio::fs::File::open(path);
        let mut dst = String::new();
        file.await?.read_to_string(&mut dst).await?;
        Ok(serde_json::from_str(&dst)?)
    }

    #[tokio::test]
    async fn test_fetch_incidents() {
        let path = env::var("INCIDENTS").unwrap_or_else(|_| String::from("incidents.json"));
        fetch_incidents(path).await.unwrap();
    }

    #[tokio::test]
    async fn test_connection() {
        let username = env::var("ADDRESS").unwrap();
        let password = env::var("PASSWORD").unwrap();
        let relay = env::var("RELAY").unwrap();
        assert!(
            fire_alarm_service::test_connection(username, password, &relay)
                .await
                .unwrap()
        )
    }

    #[tokio::test]
    async fn test_main() {
        let path = env::var("INCIDENTS").unwrap_or_else(|_| String::from("incidents.json"));
        let incidents = fetch_incidents(path).await.unwrap();

        let timestamp = env::var("TIMESTAMP").unwrap_or_else(|_| String::from("timestamp.txt"));
        let database = match env::var("DATABASE") {
            Ok(opt) => sea_orm::Database::connect(opt).await,
            Err(_) => {
                let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
                fire_alarm_service::setup_db(&db, true).await.unwrap();
                Ok(db)
            }
        };
        let address = lettre::Address::new("obiwan.konobi", "jedi.com").unwrap();

        fire_alarm_service::test_run(
            timestamp,
            std::future::ready(database),
            incidents,
            "index.html",
            address,
        )
        .await
        .unwrap();
    }

    #[cfg(feature = "file-transport")]
    #[tokio::test]
    async fn test_file_transport() {
        let path = env::var("INCIDENTS").unwrap_or_else(|_| String::from("incidents.json"));
        let incidents = fetch_incidents(path).await.unwrap();

        let timestamp = env::var("TIMESTAMP").unwrap_or_else(|_| String::from("timestamp.txt"));
        let database = match env::var("DATABASE") {
            Ok(opt) => sea_orm::Database::connect(opt).await,
            Err(_) => {
                let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
                fire_alarm_service::setup_db(&db, true).await.unwrap();
                Ok(db)
            }
        };
        let address = lettre::Address::new("obiwan.konobi", "jedi.com").unwrap();

        fire_alarm_service::file_run(
            timestamp,
            std::future::ready(database),
            incidents,
            "index.html",
            address,
        )
        .await
        .unwrap();
    }
}
