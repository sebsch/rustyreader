use std::option::Option;
use ureq;

use log::{debug, info};

pub fn create_url(permalink: &str, optional_args: Option<&str>) -> String {
    let args = optional_args.unwrap_or("");
    format!("https://www.reddit.com{}.json{}", permalink, args)
}

struct Request {
    methode: String,
    url: String,
    body: Option<String>,
    header: Option<String>,
}

impl Request {
    pub fn send(&self) -> ureq::Response {
        match self.methode.as_str() {
            "GET" => ureq::get(&self.url).call(),
            "POST" => ureq::post(&self.url).call(),
            _ => panic!("methode {} not defined!", self.methode),
        }
    }
}

pub fn gather_site(url: String) -> String {
    let req = Request {
        methode: String::from("GET"),
        url,
        body: None,
        header: None,
    };
    info!("({}) >> {} ", req.methode, req.url);
    let resp = req.send();
    info!("  [{}] ", resp.status());

    let text = resp.into_string().unwrap_or_else(|error| {
        // ToDo Handle error better -- don't panic!
        panic!("{:?}", error)
    });

    debug!("{}", text);
    text
}
