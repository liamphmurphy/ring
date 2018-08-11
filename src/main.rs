extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::process::{Command, Stdio};
use std::env;
use std::vec::Vec;

#[derive(Deserialize)]
struct Server {
    games: BTreeMap<String, Games>,
}

#[derive(Deserialize)]
struct Games {
    ip_addr: String
}

fn load_list() -> Server {
    let path = Path::new("./games.json");
    let file = File::open(path).expect("Could not open file, exiting program.");
    let deserialize_list: Server = serde_json::from_reader(file).expect("Error reading json.");
    return deserialize_list
}

fn print_list() {
    let list = load_list();
    for (game, value) in list.games.iter() {
        println!("Game: {} -- IP: {}", game, value.ip_addr);
    }
}

fn ping_command<'a>(ip: &String, server_target: &'a std::string::String) -> std::process::Child {
    println!("Pinging {} servers...", server_target);

    // Let the OS run a ping command, provide args and stdout
    if cfg!(windows){
        let ping = Command::new("ping")
            .arg("-n")
            .arg("4")
            .arg(&ip)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Error pinging {}, continuing.");
        return ping

    } else {
        let ping = Command::new("ping")
            .arg("-q")
            .arg("-c")
            .arg("4")
            .arg(&ip)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Error pinging {}, continuing.");
        return ping
    };

}

fn gather_output (ping: std::process::Child) -> String {
    
    // Let the ping exit, when done, get content from stdout
    let output = ping.wait_with_output().unwrap();
    let out = BufReader::new(&*output.stdout);

    let mut out_vector = Vec::new();
    for line in out.lines() {
        out_vector.push(line.unwrap().to_string());
    }
    // Initialize result string
    let result: String;

    // Due to ping commands differing on platforms, change which element to display based on platform of user.
    if cfg!(windows) {
        result = out_vector[10].to_string();
        return result
    } else if cfg!(unix) {
        result = out_vector[4].to_string();
        return result
    } else {
        return String::from("Error gathering output.")
    }
}

fn split_output (output: String) {
    if cfg!(unix) {
        // Unix ping syntax requires splitting / signs to access desired data.
        let split = output.split("/");
        let split_vec: Vec<&str> = split.collect();

        // If length of split_vec is under 2 (which means the ping failed), use if to continue to next server if any.
        if split_vec.len() < 2 {
            println!("Ping response was empty, moving to next server.");
        } else {
            let min_ping_split = split_vec[3].split("= ");
            let min_vec: Vec<&str> = min_ping_split.collect();
            let min_ping = min_vec[1];
            let avg_ping = split_vec[4];
            let max_ping = split_vec[5];
        display_output(min_ping, avg_ping, max_ping);
        }
    }
    if cfg!(windows){
        // Windows ping syntax requires splitting = signs to access desired data.
        let split = output.split("=");
        let split_vec: Vec<&str> = split.collect();
        if split_vec.len() < 2 {
            println!("Ping response was empty, moving to next server.");
        } else {
            let min_ping_split = split_vec[1].split(",");
            let max_ping_split = split_vec[2].split(",");
            let min_vec: Vec<&str> = min_ping_split.collect();
            let max_vec: Vec<&str> = max_ping_split.collect();

            let min_ping = min_vec[0];
            let avg_ping = split_vec[3];
            let max_ping = max_vec[0];
            display_output(min_ping, avg_ping, max_ping);
        }
    }
}

fn display_output(min: &str, avg: &str, max: &str) {
    println!("\nMinimum Ping: {} ms", min);
    println!("Maximum Ping: {} ms", max);
    println!("Average Ping: {} ms\n", avg);
}

fn main() {
    println!("Welcome to the Ring ping tool.\n");

    // For any possible number of user args, collect them into vector
    let args: Vec<String> = env::args().collect();

    // Call load_list to deserialize games.json
    let list = load_list();

    let mut ping_result : std::process::Child;
    let mut get_output: String;
    for arg in &args {
        if arg == "list" {
            print_list();
        }
        for (game, value) in list.games.iter() {
            if arg == "all"{
                ping_result = ping_command(&value.ip_addr, game);
                get_output = gather_output(ping_result);
                split_output(get_output);
            }   
            if &arg == &game {
               // Check OS of user, because ping syntax changes. Better solution for this would be nice.
                ping_result = ping_command(&value.ip_addr, game);
                get_output = gather_output(ping_result);
                split_output(get_output);
                } 
           }
   }
    println!("All requested pings are complete.")
}
