use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

pub fn handle_connection(mut stream: TcpStream) {
  loop {
    let mut buffer = [0; 1028];

    match stream.read(&mut buffer) {
      Ok(n) => {
        if n == 0 {
          break;
        }

        match stream.write_all(&buffer[0..n]) {
          Ok(()) => {
            println!("{}", String::from_utf8_lossy(&buffer));
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
