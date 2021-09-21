use encoding::all::ASCII;
use encoding::{EncoderTrap, Encoding};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
extern crate serde;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
extern crate dirs;

//Structure to hold our host and port values de-serialized from json
#[derive(Serialize, Deserialize)]
pub struct HostInfo {
    pub submit_host: String,
    pub submit_port: String,
}

fn main() -> std::io::Result<()> {
    //Check if hostinfo.json file exists. If not, create it
    // with default values otherwise use what is in the json file
    let mut dir = dirs::home_dir().unwrap();
    dir.push("hostinfo.json");

    let hi = if dir.as_path().exists() {
        let mut file = File::open(dir.as_path()).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        let h: HostInfo = serde_json::from_str(&buff).unwrap();
        h
    } else {
        let h = HostInfo {
            submit_host: String::from("127.0.0.1"),
            submit_port: String::from("3505"),
        };

        let sptr = serde_json::to_string(&h).unwrap();
        let mut file = File::create(dir.as_path()).unwrap();
        write!(&mut file, "{}", sptr).unwrap();
        h
    };

    //Get command line parameters. Name of the jcl file to submit
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let jcl = &args[1];

        let data = fs::read_to_string(jcl).unwrap();

        let command_bytes = ASCII.encode(&data, EncoderTrap::Strict).unwrap();
        let mut stream = TcpStream::connect(format!("{}:{}", hi.submit_host, hi.submit_port))?;

        stream.write_all(&command_bytes).unwrap();
    }
    Ok(())
}
