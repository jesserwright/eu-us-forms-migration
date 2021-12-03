use dotenv::dotenv;
use mongodb::{
    bson::Document,
    options::{ClientOptions, Tls, TlsOptions},
    Client,
};
use s37::FormTemplate;
use futures::stream::TryStreamExt;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let connection_string = dotenv::var("CONNECTION_STRING")?;
    let db_name = dotenv::var("DB_NAME")?;
    let db_connection = build_connection(&connection_string)
        .await?
        .database(&db_name);
    let collection =
        db_connection.collection::<Document>("trial-07646c8a-86a0-49ef-a342-0704ef4b3c2e").;
    for doc in collection.try_collect() {
        dbg!(doc);
    }

    Ok(())
}

async fn build_connection(connection_string: &str) -> Result<Client, Box<dyn std::error::Error>> {
    let mut client_options = ClientOptions::parse(connection_string).await?;
    let path = std::path::PathBuf::from("./cert.pem");
    let mut opts = TlsOptions::default();
    opts.ca_file_path = Some(path);
    client_options.tls = Some(Tls::Enabled(opts));
    let client = Client::with_options(client_options)?;
    return Ok(client);
}
