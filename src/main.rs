extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::env;
use std::vec::Vec;

#[derive(Deserialize)]
struct Server {
    games: HashMap<String, Games>,
}

#[derive(Deserialize)]
struct Games {
    ip_addr: String
}

fn print_list() {
    let list = load_list();
    for (game, value) in list.games.iter() {
        println!("Game: {} -- IP: {}", game, value.ip_addr);
    }
}

fn load_list() -> Server {
    let path = Path::new("src/games.json");
    let file = File::open(path).expect("Could not open file, exiting program.");
    let desereialize_list: Server = serde_json::from_reader(file).expect("Error reading json.");
    return desereialize_list
}

fn ping_unix<'a>(ip: &String, server_target: &'a std::string::String) -> String{
    println!("Pinging {} servers...", server_target);
    // Let the OS run a ping command, provide args and stdout
    let ping = Command::new("ping")
        .arg("-q")
        .arg("-c")
        .arg("4")
        .arg(&ip)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // Let the ping exit, when done, get content from stdout
    let output = ping.wait_with_output().unwrap();
    let out = BufReader::new(&*output.stdout);

    let mut out_vector = Vec::new();
    for line in out.lines() {
        out_vector.push(line.unwrap().to_string());
    }

    // 4th element in out_vector is what shows ping, so we return that as a string
    return out_vector[4].to_string()
}

fn ping_windows<'a>(ip: &String, server_target: &'a std::string::String) -> String{
    println!("Pinging {} servers...", server_target);
    // Let the OS run a ping command, provide args and stdout
    let ping = Command::new("ping")
        .arg("-n")
        .arg("4")
        .arg(&ip)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // Let the ping exit, when done, get content from stdout
    let output = ping.wait_with_output().unwrap();
    let out = BufReader::new(&*output.stdout);

    let mut out_vector = Vec::new();
    for line in out.lines() {
        out_vector.push(line.unwrap().to_string());
    }

    // 4th element in out_vector is what shows ping, so we return that as a string
    return out_vector[4].to_string()
}

fn split_output(output: String) {
    let split = output.split("/");
    let split_vec: Vec<&str> = split.collect();

    println!("\nAverage Ping: {} ms", split_vec[4]);
    
}

fn main() {
    println!("Welcome to the Ring ping tool.\n");

    // For any possible number of user args, collect them into vector
    let args: Vec<String> = env::args().collect();

    // Call load_list to deserialize games.json
    let list = load_list();

    let mut ping_result : String;
    for arg in &args {
        if arg == "list" {
            print_list();
        }
        for (game, value) in list.games.iter() {   
           if &arg == &game {
               // Check OS of user, because ping syntax changes. Better solution for this would be nice.
                if cfg!(unix){
                    ping_result = ping_unix(&value.ip_addr, game);
                    split_output(ping_result);
                } else if cfg!(windows){
                    ping_result = ping_windows(&value.ip_addr, game);
                    split_output(ping_result);
                } 
           }
       }
   }
    // Take user args vector and call server_list
    //server_list(args);

    

   
    
}
