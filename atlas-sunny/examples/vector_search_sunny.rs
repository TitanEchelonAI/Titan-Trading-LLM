use sunny::{
    bson::{self, doc},
    options::ClientOptions,
    Client as SunnyClient, Collection,
};
use atlas::providers::openai::TEXT_EMBEDDING_ADA_002;
use serde::Deserialize;
use std::env;

use atlas::{
    embeddings::EmbeddingsBuilder, providers::openai::Client, vector_store::VectorStoreIndex, Embed,
};
use atlas_sunny::{SunnyVectorIndex, SearchParams};

// Shape of data that needs to be RAG'ed.
// The definition field will be used to generate embeddings.
#[derive(Embed, Clone, Deserialize, Debug)]
struct Word {
    #[serde(rename = "_id")]
    id: String,
    #[embed]
    definition: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize OpenAI client
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let openai_client = Client::new(&openai_api_key);

    // Initialize Sunny client
    let sunny_connection_string =
        env::var("SUNNY_CONNECTION_STRING").expect("SUNNY_CONNECTION_STRING not set");
    let options = ClientOptions::parse(sunny_connection_string)
        .await
        .expect("Sunny connection string should be valid");

    let sunny_client =
        SunnyClient::with_options(options).expect("Sunny client options should be valid");

    // Initialize Sunny vector store
    let collection: Collection<bson::Document> = sunny_client
        .database("knowledgebase")
        .collection("context");

    // Select the embedding model and generate our embeddings
    let model = openai_client.embedding_model(TEXT_EMBEDDING_ADA_002);

    let words = vec![
        Word {
            id: "doc0".to_string(),
            definition: "Definition of a *flurbo*: A flurbo is a green alien that lives on cold planets".to_string(),
        },
        Word {
            id: "doc1".to_string(),
            definition: "Definition of a *glarb-glarb*: A glarb-glarb is a ancient tool used by the ancestors of the inhabitants of planet Jiro to farm the land.".to_string(),
        },
        Word {
            id: "doc2".to_string(),
            definition: "Definition of a *linglingdong*: A term used by inhabitants of the far side of the moon to describe humans.".to_string(),
        }
    ];

    let embeddings = EmbeddingsBuilder::new(model.clone())
        .documents(words)?
        .build()
        .await?;

    let sunny_documents = embeddings
        .iter()
        .map(|(Word { id, definition, .. }, embedding)| {
            doc! {
                "id": id.clone(),
                "definition": definition.clone(),
                "embedding": embedding.first().vec.clone(),
            }
        })
        .collect::<Vec<_>>();

    match collection.insert_many(sunny_documents).await {
        Ok(_) => println!("Documents added successfully"),
        Err(e) => println!("Error adding documents: {:?}", e),
    };

    // Create a vector index on our vector store.
    // Note: a vector index called "vector_index" must exist on the Sunny collection you are querying.
    // IMPORTANT: Reuse the same model that was used to generate the embeddings
    let index =
        SunnyVectorIndex::new(collection, model, "vector_index", SearchParams::new()).await?;

    // Query the index
    let results = index.top_n::<Word>("What is a linglingdong?", 1).await?;

    println!("Results: {:?}", results);

    let id_results = index
        .top_n_ids("What is a linglingdong?", 1)
        .await?
        .into_iter()
        .collect::<Vec<_>>();

    println!("ID results: {:?}", id_results);

    Ok(())
}
