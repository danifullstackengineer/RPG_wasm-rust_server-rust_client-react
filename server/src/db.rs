use mongodb::{Client, options::ClientOptions, Database};


pub async fn create_conn() -> Result<(Client, Database), mongodb::error::Error> {
    let client = Client::with_uri_str("mongodb+srv://babadany2999:Immboold1@kanji.zw8f3.mongodb.net/Kanji?retryWrites=true&w=majority").await?;
    let db = client.database("Kanji");
    Ok((client,db))
}