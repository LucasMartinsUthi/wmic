

use tokio::io::{AsyncWriteExt, AsyncReadExt, BufReader};
use serde::{Serialize, Deserialize};
use tokio::net::TcpStream;

use std::env;
use std::fmt;
use std::fmt::write;
use std::process;
use std::io::{self, Write};

use snafu::prelude::*;

use std::collections::HashMap;
use wmi::Variant;


use wmic::{Args};

#[derive(Serialize, Deserialize, Debug)]
struct MonstaVariant(Variant);

impl fmt::Display for MonstaVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        match self.0 {
            Variant::Empty => write!(f, "Empty"),
            Variant::Null => write!(f, "Null"),
            Variant::String(ref s) => write!(f, "{}", s),
            Variant::I1(i) => write!(f, "{}", i),
            Variant::I2(i) => write!(f, "{}", i),
            Variant::I4(i) => write!(f, "{}", i),
            Variant::I8(i) => write!(f, "{}", i),
            Variant::R4(i) => write!(f, "{}", i),
            Variant::R8(i) => write!(f, "{}", i),
            Variant::Bool(b) => write!(f, "{}", b),
            Variant::UI1(i) => write!(f, "{}", i),
            Variant::UI2(i) => write!(f, "{}", i),
            Variant::UI4(i) => write!(f, "{}", i),
            Variant::UI8(i) => write!(f, "{}", i),
            Variant::Array(ref v) => {
                write!(f, "[")?;
                // for i in v {
                //     // let monsta_variant = MonstaVariant(i);
                //     // write!(f, "{}", monsta_variant)?;
                // }
                write!(f, "]")
            }

        }
    }
}

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
    items: Vec<HashMap<String, MonstaVariant>>,
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
    #[snafu(display("Error addrs"))]
	ErrorAddrs,
}

type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let args = Args::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut stream = TcpStream::connect("127.0.0.1:7743").await.context(IoSnafu)?;

    let query = QueryWMI {
        // wql: args.wql,
        wql: "select * from Win32_ComputerSystem".to_string(),
        password: "secret".to_string(),
    };

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
    
    // response to json
    // println!("{}", response);
    let response: QueryResult  = serde_json::from_str(&response).context(SerdeSnafu)?;
    
    //get item keys from first item
    let mut keys = Vec::new();
    for (key, _) in response.items[0].iter() {
        keys.push(key.to_string());
    }
    println!("{}", keys.join("|"));


    // loop over items and print there values
    for item in response.items {
        for value in item.values() {
            print!("{}|", value);
        }
    }

    Ok(())
}


