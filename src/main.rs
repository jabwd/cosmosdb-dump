use std::collections::HashMap;
use azure_data_cosmos::prelude::*;
use serde_json::Value;
use tokio_stream::StreamExt;
use clap::Parser;

mod args;
mod dump_file;

async fn dump_database(client: &CosmosClient, id: &String) -> Option<dump_file::Database> {
    let db_client = client.database_client(id.clone());
    let mut col_stream = db_client.list_collections().into_stream();
    let mut collections: Vec<dump_file::Collection> = vec![];
    while let Some(collection) = col_stream.next().await {
        match collection {
            Ok(collection) => {
                for col in collection.collections {
                    if let Some(result) = dump_collection(&db_client, &col.id).await {
                        collections.push(result);
                    }
                }
            },
            Err(why) => {
                eprintln!("Err: failed to fetch collection {:?}", why);
                return None;
            }
        }
    } 

    Some(dump_file::Database {
        name: id.clone(),
        collections,
    })
}

async fn dump_collection(client: &DatabaseClient, id: &String) -> Option<dump_file::Collection> {
    let col_client = client.collection_client(id.clone());
    let mut document_stream = col_client.list_documents().into_stream::<HashMap<String, Value>>();
    let mut documents: Vec<HashMap<String, Value>> = vec![];
    while let Some(document) = document_stream.next().await {
        match document {
            Ok(document) => {
                for doc in document.documents {
                    documents.push(doc.document);
                }
            },
            Err(why) => {
                eprintln!("Err: failed to fetch document {:?}", why);
                return None;
            }
        }
    }

    Some(dump_file::Collection {
        name: id.clone(),
        documents,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Cli::parse();

    let auth_token = AuthorizationToken::primary_from_base64(&args.key).expect("Unable to create auth token");
    let client = CosmosClient::new(args.account, auth_token);

    let mut databases = client.list_databases().into_stream();
    let mut db_list: Vec<dump_file::Database> = vec![];
    while let Some(db) = databases.next().await {
        match db {
            Ok(db) => {
                for bla in db.databases {
                    if let Some(r) = dump_database(&client, &bla.id).await {
                        db_list.push(r);
                    }
                }
            },
            Err(why) => {
                println!("Failed to list db: {:?}", why);
                return Ok(());
            }
        }
    }

    let dump = dump_file::DumpFile {
        databases: db_list,
    };

    match serde_json::to_string_pretty(&dump) {
        Ok(s) => {
            _ = tokio::fs::write(args.out, &s).await
        },
        Err(why) => {
            eprintln!("Err generating json output: {:?}", why);
            return Ok(());
        }
    }

    Ok(())
}
