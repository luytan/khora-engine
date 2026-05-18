use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

struct HtmlRequest {
    parameter: HtmlParameters,
    host: HtmlHost,
    user_agent: HtmlUserAgent,
    accept: String,
    end: String,
}
//impl HtmlRequest {
//    pub fn new(stream: &TcpStream) -> HtmlRequest {}
//}
#[derive(Debug)]
struct HtmlParameters {
    method: String,
    uri: String,
    version: String,
}
impl HtmlParameters {
    pub fn new(bytes: &[u8]) -> HtmlParameters {
        let mut parameters: Vec<&[u8]> = Vec::new();
        let mut i = 0;
        let mut old_i = 0;
        for byte in bytes {
            if *byte != 32 && parameters.len() != 2 {
                i += 1;
            } else if parameters.len() != 2 {
                parameters.push(&bytes[old_i..i]);
                i += 1;
                old_i = i;
            } else {
                parameters.push(&bytes[i - 1 + 1..bytes.len()]);
            }
        }
        HtmlParameters {
            method: String::from_utf8_lossy(&parameters[0]).to_string(),
            uri: String::from_utf8_lossy(&parameters[1]).to_string(),
            version: String::from_utf8_lossy(&parameters[2]).to_string(),
        }
    }
}

struct HtmlHost {
    ip_address: String,
    port: u8,
}

struct HtmlUserAgent {
    client: String,
    version: f32,
}

fn build_not_found() -> String {
    let parameter: &str = "HTTP/1.1 404 Not Found";
    let header: &str = "Content-Type: text/plain";
    let new_line: &str = "\r\n";
    let data: &str = "404 Not Found";
    return format!(
        "{}{}{}{}{}{}",
        parameter, new_line, header, new_line, new_line, data
    );
}

fn build_response(data: &String) -> String {
    let parameter: &str = "HTTP/1.1 200 OK";
    let header: &str = "Content-Type: text/html";
    let new_line: &str = "\r\n";;
    return format!(
        "{}{}{}{}{}{}",
        parameter, new_line, header, new_line, new_line, data
    );
}

fn return_message(mut stream: TcpStream, index: &String) {
    let mut buffer: Vec<u8> = vec![0; 1024];
    let _ = stream.read(&mut buffer);

    // find the parameter
    let mut i = 0;
    for byte in buffer.windows(2) {
        if byte != [13, 10] {
            i += 1;
        } else {
            // for "10"
            i += 2;
            break;
        }
    }
    let parameters: HtmlParameters = HtmlParameters::new(&buffer[0..i]);
    if parameters.method != "GET" {
        // here return error 500 or idk
        let _ = stream.write(build_not_found().as_bytes());
    } else {
        let _ = stream
            .write(build_response(index).as_bytes());
    }
}

fn main() -> std::io::Result<()> {
    println!("Starting server...");
    let index = fs::read_to_string("index.html")?;
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        println!("Wow a stream");
        return_message(stream?, &index);
    }
    Ok(())
}
