use std::net::TcpStream;
use std::io::Write;
use lsl;
use lsl::Pullable;
use clap::{Arg, App};

fn main() {
    let matches = App::new("ISAE Robot Firefighter NodeJS Game to LSL")
        .version("0.1.0")
        .author("Giorgio Angelotti <giorgio.angelotti@isae-supaero.fr>")
        .about("Streams POMDP Action from LSL to ISAE Robot Firefighter NodeJS Game.")
        .arg(Arg::with_name("address")
                 .short('a')
                 .long("address")
                 .takes_value(true)
                 .help("The address of the TCP server."))
        .arg(Arg::with_name("port")
                 .short('p')
                 .long("port")
                 .takes_value(true)
                 .help("The port of the TCP server."))
        .get_matches();

    let server_address = matches.value_of("address");
    let server_port = matches.value_of("port");
    let full_address = server_address.unwrap().to_owned() + ":" + server_port.unwrap();

    // declare streams
    let action_stream = lsl::resolve_bypred("name='POMDPactionStream'", 1, lsl::FOREVER).unwrap();
    
    // create inlet
    let action_inlet = lsl::StreamInlet::new(&action_stream[0], 360, 0, true).unwrap();


    println!("Looking for the server.");
    // Creating the client
    match TcpStream::connect(&full_address) {
        Ok(mut stream) => {
            println!("Successfully connected");

            // read the streaming data and print the multi-channel samples 
            loop {
                let (action, ts): (Vec<i8>, _) = action_inlet.pull_sample(lsl::FOREVER).unwrap();
                println!("From LSL got {:?} at time {}", action.first().unwrap(), ts);
                let message = action.first().unwrap().to_owned().to_string();
                stream.write(message.as_bytes()).unwrap();
                println!("Sent action {}.", &message);
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}