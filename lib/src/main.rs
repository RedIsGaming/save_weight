use std::net::{ TcpListener, TcpStream };
use std::io::{ BufReader, prelude::* };
use std::{ error, fs, thread };

fn request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK \r\n text/html;";
    let contents = fs::read_to_string("../web/index.html").unwrap();
    let length = contents.len();
    
    let response = format!("
        {status_line}\r\nContent-Length:
        {length}\r\n\r\n
        {contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for tcp_stream in listener.incoming() {
        let stream = tcp_stream.unwrap();

        thread::spawn(|| {
            request(stream);
        });
    }

    Ok(())
}
