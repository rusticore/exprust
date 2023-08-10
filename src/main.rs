use std::{net::TcpListener, thread};

use crate::utils::stream::handle_client;

mod utils {
  pub mod stream;
}

fn main() {
  let addr = "127.0.0.1:4000";
  let listener = TcpListener::bind(addr).unwrap();

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        thread::spawn(move || handle_client(stream));
      }
      Err(err) => {
        panic!("{}", err);
      }
    }
  }
}
