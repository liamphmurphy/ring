use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::env;
use std::borrow::Cow;
use std::vec::Vec;

fn ping_server<'a>(ip: String) -> String{
    // Let the OS run a ping command.
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
    return out_vector[3].to_string()
}

fn server_list (args: Vec<String>) {
    for server in &args {
        if server == "target/debug/ring" {
            continue
        } else if server == "rocketleague" {
            println!("Pinging rocket league servers...");
        } else if server == "overwatch-west" {
            println!("Pinging overwatch-west servers...");
            let ip_string = String::from("24.105.30.129");
            let final_ping = ping_server(ip_string);
            println!("Final Ping: {}", final_ping);
        } else {
            println!("\n{} is not a valid game.", server)
        }
    }
}

fn main() {
    println!("Welcome to the Ring ping tool.\n");

    // For any possible number of user args, collect them into vector
    let args: Vec<String> = env::args().collect();
    
    // Take user args vector and call server_list
    server_list(args);

}
