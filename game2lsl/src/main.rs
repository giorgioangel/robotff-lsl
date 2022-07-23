use std::str;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use lsl;
use lsl::{StreamOutlet, Pushable};
use clap::{Arg, App};

fn handle_client(mut stream: TcpStream, outlet_float: &StreamOutlet, outlet_string: &StreamOutlet) {
    let mut data = [0 as u8; 1024]; // using 1024 byte buffer
    while match stream.read(&mut data) {
        Ok(_size) => {
            let message = str::from_utf8(&data).unwrap();
            let message = message.replace(&['\u{0}'][..], "");
            let message_string  = message.split_whitespace();
            let data_string : Vec<&str> = message_string.collect();

            let vec_float = vec![data_string[0].parse::<f32>().unwrap(), data_string[1].parse::<f32>().unwrap(), data_string[3].parse::<f32>().unwrap(), data_string[4].parse::<f32>().unwrap(), data_string[5].parse::<f32>().unwrap(), data_string[7].parse::<f32>().unwrap(), data_string[8].parse::<f32>().unwrap(), data_string[9].parse::<f32>().unwrap(), data_string[10].parse::<f32>().unwrap()];
            let vec_str : Vec<&str> = vec![data_string[2], data_string[6], data_string[11], data_string[12], data_string[13], data_string[14], data_string[15]];
            println!("HAI Vector String {:?}", vec_str);
            println!("HAI Vector Float {:?}", vec_float);
            // push!
            outlet_float.push_sample(&vec_float).ok();
            outlet_string.push_sample(&vec_str).ok();
            // echo everything!
            stream.write("Data received and streamed to LSL!".as_bytes()).ok();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}


fn main() {
    let matches = App::new("ISAE Robot Firefighter NodeJS Game to LSL")
        .version("0.1.0")
        .author("Giorgio Angelotti <giorgio.angelotti@isae-supaero.fr>")
        .about("Streams HAI data from ISAE Robot Firefighter NodeJS Game to LSL, needs to set up a TCP server.")
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
    let info_float = lsl::StreamInfo::new(
            "nodejsFloat", "HAIfloat", 9, 0.0,
            lsl::ChannelFormat::Float32, "HAIfloatuid").unwrap();
    let info_string = lsl::StreamInfo::new(
                "nodejsString", "HAIstring", 7, 0.0,
                lsl::ChannelFormat::String, "HAIstringuid").unwrap();
    
        // create outlets
    let outlet_float = lsl::StreamOutlet::new(&info_float, 0, 360).unwrap();
    let outlet_string = lsl::StreamOutlet::new(&info_string, 0, 360).unwrap();

    // Creating the server
    let listener = TcpListener::bind(&full_address).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("TCP Server listening HAI data on {}", &full_address);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream, &outlet_float, &outlet_string);
    }
    // close the socket server
    drop(listener);
}