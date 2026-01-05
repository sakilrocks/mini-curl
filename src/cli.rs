
use std::env;
use std::error::Error;
use reqwest::Method;


pub struct Config {
    pub method: Method,
    pub url: String,
    pub body: Option<String>,
}



pub fn parse_args() -> Result<Config, Box<dyn Error>> {
    let mut args = env::args().skip(1);

    let mut method = Method::GET;
    let mut body: Option<String> = None;
    let mut url: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-X" => {
                let m = args.next().ok_or("missing HTTP method after -X")?;
                method = m.parse()?;
            }
            "-d" => {
                body = Some(args.next().ok_or("missing data after -d")?);
            }
            _ => {
                url = Some(arg);
            }
        }

    }

    let url = url.ok_or("usage: mini_curl [-X METHOD] [-d DATA] <URL>")?;

    Ok(Config { method, url, body })

    
}