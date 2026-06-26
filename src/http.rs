//! Model for http
use anyhow::Result;

pub enum HttpVersion {
    Http1_1,
    Http2,
}
#[derive(PartialEq, Debug)]
pub enum HttpMethod {
    Get,
    Pri,
    Put,
    Post,
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Option<HttpMethod> {
        match s {
            "GET" => Some(HttpMethod::Get),
            "POST" => Some(HttpMethod::Post),
            "PRI" => Some(HttpMethod::Pri),
            "PUT" => Some(HttpMethod::Put),
            _ => None,
        }
    }
}

/// From a byte slice return an Option that contain the http version
pub fn get_http_version() -> Option<HttpVersion> {
    Some(HttpVersion::Http2)
}

pub async fn handle_http_1_1() -> Result<()>{
    Ok(())
}

#[derive(Debug)]
pub struct HtmlParameters {
    method: HttpMethod,
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
            method: HttpMethod::from_str(&String::from_utf8_lossy(&parameters[0]).to_string()).expect("unknown method"),
            uri: String::from_utf8_lossy(&parameters[1]).to_string(),
            version: String::from_utf8_lossy(&parameters[2]).to_string(),
        }
    }
    pub fn method(&self) -> HttpMethod{
        HttpMethod::Get
    }
    pub fn uri(&self) -> &str{
        &self.uri
    }
}
pub struct HtmlRequest {
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
pub struct HtmlHost {
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
