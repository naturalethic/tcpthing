#[macro_use]
extern crate clap;

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn random_bytes(size: u32) -> Vec<u8> {
  (0..size).map(|_| rand::random::<u8>()).collect()
}

fn main() {
  let error = |e| {
    eprintln!("{}", e);
    std::process::exit(1);
  };
  let matches = clap::App::new("tcpthing")
    .version("0.1.0")
    .args_from_usage(
      "<local-port> Port this process listens on
      <remote-port-1> First port this process connects to
      <remote-port-2> Second port this process connects to",
    )
    .get_matches();
  let local_port = value_t!(matches, "local-port", u16).unwrap_or_else(error);
  let remote_port_1 = value_t!(matches, "remote-port-1", u16).unwrap_or_else(error);

  let key_count = 2;
  let key_bytes = 16;
  let keys = random_bytes(key_count * key_bytes);

  let listener = TcpListener::bind(format!("0.0.0.0:{}", local_port)).unwrap();
  println!("Listening on port {}", local_port);

  std::thread::spawn(move || loop {
    thread::sleep(std::time::Duration::from_millis(3000));
    println!(
      "Attempting to connect to remote 1 on port {} ",
      remote_port_1
    );
    match TcpStream::connect(format!("localhost:{}", remote_port_1)) {
      Ok(mut stream) => {
        println!("Connected to remote 1 on port {}", remote_port_1);
        match stream.write(keys.as_slice()) {
          Ok(_) => println!("Sent {} bytes", keys.len()),
          Err(_) => {}
        }
        break;
      }
      Err(_) => {}
    }
  });

  let mut remote_keys: Vec<u8>;

  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        println!("Connection received from {}", stream.peer_addr().unwrap());
        // TODO: Read bytes int remote_keys
      }
      Err(_) => {}
    }
    break;
  }
}
