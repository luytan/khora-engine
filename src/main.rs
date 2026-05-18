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
                parameters.push(&bytes[i - 1 + 1..bytes.len() - 2]);
            }
        }
        HtmlParameters {
            method: String::from_utf8_lossy(&parameters[0]).to_string(),
            uri: String::from_utf8_lossy(&parameters[1]).to_string(),
            version: String::from_utf8_lossy(&parameters[2]).to_string(),
        }
    }
}

#[derive(Debug)]
struct HtmlHost {
    ip_address: String,
    port: String,
}
impl HtmlHost {
    pub fn new(bytes: &[u8]) -> HtmlHost {
        let mut parameters: Vec<&[u8]> = Vec::new();
        let mut i = 0;
        let mut old_i = 0;
        for byte in bytes {
            if *byte == 58 {
                parameters.push(&bytes[old_i..i]);
                i += 1;
                old_i = i;
            } else if parameters.len() == 2 {
                parameters.push(&bytes[i..bytes.len()-3]);
            } else {
                i += 1;
            }
        }
        HtmlHost {
            ip_address: String::from_utf8_lossy(&parameters[1]).to_string(),
            port: String::from_utf8_lossy(&parameters[2]).to_string(),
        }
    }
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

fn build_response(data: &String, kind: u8) -> String {
    let parameter: &str = "HTTP/1.1 200 OK";
    let header = match kind {
        0 => "Content-Type: text/html".to_string(),
        1 => "Content-Type: text/css".to_string(),
        2=> "Content-Type: application/javascript".to_string(),
        _ => "Content-Type: text/plain".to_string(),
    };
    let new_line: &str = "\r\n";
    return format!(
        "{}{}{}{}{}{}",
        parameter, new_line, header, new_line, new_line, data
    );
}

fn handle_request(mut stream: TcpStream, website: &Vec<String>) {
    let mut buffer: Vec<u8> = vec![0; 1024];
    let _ = stream.read(&mut buffer);
    let mut html_separation: Vec<u32> = Vec::new();

    // find the parameter
    let mut i = 0;
    for byte in buffer.windows(2) {
        if byte != [13, 10] {
            i += 1;
        } else {
            // for "10"
            i += 2;
            html_separation.push(i);
        }
    }
    let parameters: HtmlParameters = HtmlParameters::new(&buffer[0..html_separation[0] as usize]);
    println!("parameters: {:?}", parameters);
    let host: HtmlHost =
        HtmlHost::new(&buffer[html_separation[0] as usize..html_separation[1] as usize]);
    println!("host: {:?}", host);
    if parameters.method != "GET" {
        // here return error 500 or idk
        let _ = stream.write(build_not_found().as_bytes());
    }
    
    let _ = match parameters.uri.as_str() {
        "/" => stream.write(build_response(&website[0], 0).as_bytes()),
        "/assets/index-lJN6PqaB.js" => stream.write(build_response(&website[1], 2).as_bytes()),
        "/assets/index-Crm0PPzq.css" => stream.write(build_response(&website[2], 1).as_bytes()),
        _ => stream.write(build_not_found().as_bytes()),
    };
}

fn main() -> std::io::Result<()> {
    println!("Starting server...");
    let mut website: Vec<String> = Vec::new();
    website.push(fs::read_to_string("index.html")?);
    website.push(fs::read_to_string("index-lJN6PqaB.js")?);
    website.push(fs::read_to_string("index-Crm0PPzq.css")?);
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        println!("Wow a stream");
        handle_request(stream?, &website);
    }
    Ok(())
}
