# MVS 3.8J job Submit Tool Written In Rust

#### Introduction

MVS 3.8J has an external card reader that can be used to remotely submit jobs to MVS.

It supplies a port usually 3505 by default that a socket reader is listening to. You can connect to that TCP port via a socket client and send a stream of ascii bytes (jcl file). The socket reader will read the stream of bytes, convert it to EBCIDIC and submits a job to the *Internal Reader*.

#### Vocabulary 

Lets go over some terms that you will need to understand about running jobs on MVS.

1. **JOB** - Running a batch job on MVS. Usually a program or mainframe utility.

2. **JCL** - Job Control Language (JCL) is a name for scripting languages used on MVS 3.6J to instruct the system on how to run a batch job or start a subsystem. See the link below:  

   [JCL]: https://en.wikipedia.org/wiki/Job_Control_Language

   

3. **Card Reader** - On MVS, it emulates the old mainframe card readers that read a deck of physical cards. MVS supplies a card reader on port 3505 that we can connect to via a TCP socket.

   [CARD Reader]: https://en.wikipedia.org/wiki/Punched_card_input/output

4. **HERCULES Console** - The console that is started when you start MVS on you system.

5. **MVS 3.8J** - Operating system that runs on top of HERCULES mainframe emulator. 

   [MVS 3.8J]: http://wotho.ethz.ch/tk4-/

#### Our Mission

We will create a utility written in rust that will submit a job to MVS by connecting to port 3505, reading a JCL file and submitting it to the card reader on port 3505.

The source code can be found at: 

[mvs_submit_job]: https://github.com/GroupTheorist12/mvs_submit_job

Let's get to the code.

#### Create a Rust project using cargo called sub_job

```bash
$ cargo new sub_job
     Created binary (application) `sub_job` package
```

#### Update cargo.toml file

Change directories to **sub_job** and using your trusty text editor to update the **dependencies** section of the **cargo.toml** file to look like below:

```rust
[dependencies]
encoding = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "4.0"

```

#### Update main.rs

Change directories to **src** and edit the **main.js** file to look like below:

```rust
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

```

