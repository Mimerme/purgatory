use serde_json::{self, Value};
use reqwest::blocking::Client;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct GenericError {
    msg: String,
}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for GenericError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub fn download(id: &str) -> Result<(String, String), Box<dyn Error>> {
    let (name, code) = get_shader_name_and_code(id)?;

    File::create(&name)
        .or_else(|_err| {
            Err(Box::new(GenericError {
                msg: "Error creating file when saving shader".to_string(),
            }))
        })?
        .write_all(code.as_bytes())
        .or_else(|_err| {
            Err(Box::new(GenericError {
                msg: "Error writing shader to file (check permissions)".to_string(),
            }))
        })?;

    Ok((name, code))
}

#[inline]
fn return_save_shader_error() -> Result<String, Box<dyn Error>> {
    Err(Box::new(GenericError {
        msg: "Error saving shaders".to_string(),
    }))
}

fn get_shader_name_and_code(id: &str) -> Result<(String, String), Box<dyn Error>> {
    let _https_url = "https://www.shadertoy.com/view/";
    let _http_url = "http://www.shadertoy.com/view/";
    let _url = "www.shadertoy.com/view/";

    let json = serde_json::from_str::<Value>(&get_json_string(id)?)?;

    extract_from_json(&json)
}

fn get_json_string(id: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    use reqwest::header::*;
    let mut headers = HeaderMap::new();
    headers.insert(
        REFERER,
        HeaderValue::from_static("https://www.shadertoy.com/"),
    );
    let mut res = client
        .post("https://www.shadertoy.com/shadertoy/")
        .headers(headers)
        .form(&[("s", format!("{{\"shaders\": [\"{}\"]}}", id))])
        .send()
        .unwrap();

    let mut buf = String::new();

    match res.read_to_string(&mut buf) {
        Ok(_) => {
            if buf == "[]" {
                Err(Box::new(GenericError {
                    msg: "Invalid shader. Pepehands".to_string(),
                }))
            } else {
                Ok(buf)
            }
        }
        Err(err) => Err(err.into()),
    }
}

fn extract_from_json(json: &Value) -> Result<(String, String), Box<dyn Error>> {
    let name = format!(
        "{}.frag",
        json[0]["info"]["name"].as_str().unwrap().replace(" ", "_")
    )
    .to_lowercase();
    let mut code = String::new();

    let shaders = json[0]["renderpass"].as_array().unwrap();

    if shaders.len() > 1 {
        for shader in shaders {
            if shader["name"] == "Image" {
                code = String::from(shader["code"].as_str().unwrap());
            }
        }
    } else {
        code = String::from(shaders[0]["code"].as_str().unwrap());
    }

    Ok((name, code))
}
