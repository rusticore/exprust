use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
  loop {
    let mut read = [0; 1028];

    match stream.read(&mut read) {
      Ok(n) => {
        if n == 0 {
          break;
        }

        stream.write_all(&read[0..n]).unwrap();
      }
      Err(err) => {
        panic!("{}", err);
      }
    }
  }
}
