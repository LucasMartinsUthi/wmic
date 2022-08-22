

use tokio::io::{AsyncWriteExt, AsyncReadExt, BufReader};
use serde::{Serialize, Deserialize};
use tokio::net::TcpStream;

use std::env;
use std::process;

use snafu::prelude::*;

use std::collections::HashMap;

use wmic::Args;

#[derive(Serialize, Deserialize, Debug)]
struct QueryWMI {
    wql: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum QueryResultStatus {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryResult {
    
    items: Vec<HashMap<String, String>>,

    status: QueryResultStatus,
    message: String,
}

#[derive(Debug, Snafu)]
pub enum Error {
	#[snafu(display("IO error: {source}"))]
	Io { source: std::io::Error },
    #[snafu(display("Serde error: {source}"))]
	Serde { source: serde_json::Error },
	#[snafu(display("No private keys in key file"))]
	NoPrivateKeys,
    #[snafu(display("Error ErrorWinproxy"))]
	ErrorWinproxy
}

type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let args = Args::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let query = QueryWMI {
        wql: args.wql,
        password: String::new(),
    };

    
    let mut stream = TcpStream::connect(format!("{}:7743", args.address)).await.context(IoSnafu)?;

    let query = serde_json::to_string(&query).context(SerdeSnafu)?;
    let query_bytes = query.as_bytes();

    let query_len = query_bytes.len() as u32;
    let query_len_bytes: [u8; 4] = query_len.to_be_bytes();

    stream.write(&query_len_bytes).await.context(IoSnafu)?;
    stream.write(query_bytes).await.context(IoSnafu)?;


    // read all response 
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_to_string(&mut response).await.context(IoSnafu)?;
    
    // response to QueryResult
    let response: QueryResult  = serde_json::from_str(&response).context(SerdeSnafu)?;

    match response.status {
        QueryResultStatus::Success => {},
        QueryResultStatus::Failure => {
            println!("{}", response.message);
            return Err(Error::ErrorWinproxy);
        }
    }

    let mut keys = Vec::new();
    for (key, _) in response.items[0].iter() {
        keys.push(key.to_string());
    }

    println!("");
    println!("{}", keys.join("|"));


    for item in response.items.iter(){
        let mut values = Vec::new();

        for value in item.values() {
            values.push(value.to_string());
        }

        println!("{}", values.join("|"));
    }

    Ok(())
}


