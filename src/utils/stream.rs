use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
  loop {
    let mut read = [0; 1028];

    match stream.read(&mut read) {
      Ok(n) => {
        if n == 0 {
          break;
        }

        match stream.write_all(&read[0..n]) {
          Ok(()) => {
            println!("Connection established")
          }
          Err(e) => {
            panic!("{}", e);
          }
        }
      }
      Err(e) => {
        panic!("{}", e);
      }
    }
  }
}
