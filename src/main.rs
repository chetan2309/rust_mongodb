use mongodb::{options::ClientOptions, Client, bson::doc};
use tokio::runtime::Runtime;

fn main() -> mongodb::error::Result<()> {
    let rt = Runtime::new().unwrap();
    rt.block_on(perform_insert())
}

async fn perform_insert() -> mongodb::error::Result<()> {
    let connection_string = "YOUR_CONNECTION_STRING";
    // Configure the MongoDB connection options
    let client_options = ClientOptions::parse(&connection_string)
        .await
        .expect("Failed to parse MongoDB connection string");

    // Enable retryable writes for Cosmos DB
    // client_options.retry_writes = Some(true);

    // Create the MongoDB client
    let client = Client::with_options(client_options)?;

    // Select the database and collection
    let db = client.database("rust-cosmos-demo");
    let collection = db.collection("tasks");

    // Create a document to insert
    let document = doc! {
        "name": "John Doe",
        "age": 30,
        "email": "johndoe@example.com"
    };

    // Insert the document
    collection.insert_one(document, None).await?;

    println!("Document inserted successfully");

    Ok(())
}