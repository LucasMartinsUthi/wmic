use std::env;
use std::process;
use std::io::BufReader;
use std::include_bytes;

use snafu::prelude::*;

use rustls::{ClientConfig, PrivateKey, Certificate};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::net::TcpStream;

use wmic::{Args};

#[derive(Debug, Snafu)]
pub enum Error {
	#[snafu(display("IO error: {source}"))]
	Io { source: std::io::Error },
	#[snafu(display("TLS error: {source}"))]
	Tls { source: rustls::Error },
    // #[snafu(display("Serde error: {source}"))]
	// Serde { source: serde_json::Error },
    // #[snafu(display("TOML: {source}"))]
	// Toml { source: toml::de::Error },
	#[snafu(display("No private keys in key file"))]
	NoPrivateKeys,
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // let args: Vec<String> = env::args().collect();

    // let args = Args::new(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });


    // let key_file = include_bytes!("../server.key") as &[u8];
    
    // let mut key_file_rdr = BufReader::new(key_file);
    // let mut keys = pkcs8_private_keys(&mut key_file_rdr).context(IoSnafu)?;

	// if keys.is_empty() { return Err(Error::NoPrivateKeys) };


    // let key = PrivateKey(keys.remove(0));

    // let cert_file = include_bytes!("../server.pem") as &[u8];
    // let mut cert_file_rdr = BufReader::new(cert_file);
    // let certs = certs(&mut cert_file_rdr).context(IoSnafu)?;

    // let root_certs = rustls::RootCertStore::empty();

    // let config = ClientConfig::builder()
    //     .with_safe_defaults()
    //     .with_root_certificates(root_certs)
    //     .with_no_client_auth();
    //     // .with_single_cert(certs.into_iter().map(Certificate).collect(), key).context(TlsSnafu)?;

    // let server_name = &args.address
    //     .try_into()
    //     .expect("invalid address name");

    // let mut conn = rustls::ClientConnection::new(Arc::clone(config), server_name).unwrap();
    // let mut sock = TcpStream::connect(format!("{}:443", domain_name)).unwrap();
        
    // let mut stream = rustls::Stream::new(&mut conn, &mut sock);

    // let mut plaintext = Vec::new();
    // stream
    //     .read_to_end(&mut plaintext)
    //     .unwrap();
    // stdout().write_all(&plaintext).unwrap();
        

    

    // if let Err(e) = wmic::run(config) {
    //     println!("Problem TLS: {}", e);
    //     process::exit(1);
    // };

    Ok(())
}


