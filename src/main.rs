use std::env::set_var;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use futures::executor::block_on;
use meilisearch_sdk::Client;
use log::info;

fn main() {
     block_on(async move{
         set_var("RUST_LOG", "debug");
         env_logger::init();
         info!("开始导入电影资源");
         let client = Client::new("http://localhost:7700",Some("aSampleMatserKey"));
         let mut file = File::open("movies.json").unwrap();
         let mut content = String::new();
         file.read_to_string(&mut content).unwrap();
         let movies_doc: Vec<Movie> = serde_json::from_str(&content).unwrap();
         client.index("movies").add_documents(&movies_doc,None).await.unwrap();
         //删除索引
         // client.delete_index("movies").await.unwrap();
         info!("导入结束");
     });
}

#[derive(Serialize,Deserialize)]
struct Movie {
    id: i64,
    title: String,
    poster: String,
    overview: String,
    release_date: i64,
    genres: Vec<String>
}