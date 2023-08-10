use std::{net::TcpListener, thread};

use crate::utils::stream::handle_connection;

mod utils {
  pub mod stream;
}

fn main() {
  let addr = "127.0.0.1:4000";

  match TcpListener::bind(addr) {
    Ok(l) => {
      for stream in l.incoming() {
        match stream {
          Ok(s) => {
            thread::spawn(move || handle_connection(s));
          }
          Err(e) => {
            panic!("{}", e);
          }
        }
      }
    }
    Err(e) => {
      panic!("{}", e);
    }
  }
}
