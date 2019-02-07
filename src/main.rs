#[macro_use]
extern crate serde_json;

extern crate dotenv;
extern crate reqwest;

extern crate rgit;

use dotenv::dotenv;
use reqwest::header;
use reqwest::header::HeaderMap;


use reqwest::Client;
use rgit::Config;
use std::env;
// use std::process;



fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).expect("Something");

    let c = config.clone();

    match make_request(config) {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("Success! Repo {} created.", c.repo_name);
                println!("Push an existing repository from the command line");
                println!(
                    "git remote add origin https://github.com/SachinMaharana/{}.git",
                    c.repo_name
                );
            } else if resp.status().is_server_error() {
                println!("Server error!");
            } else {
                println!("Something else happened. Github Says: {:?}", resp. error_for_status());
                println!("Repo {} Might Already Exist!", c.repo_name);
            }
        }
        Err(e) => {
            println!("Error during request.");
            println!("Error is ..{}", e);
        }
    };
    // list_repo()
}

fn construct_headers() -> HeaderMap {
    // let token = env::var("TOKEN").unwrap().to_owned();
    // let t = token.to_owned();
    let mut headers = header::HeaderMap::new();

    // let mut headers = Headers::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static(""));
    // headers.set(Authorization(Bearer {
    //     token: token.to_owned(),
    // }));
    headers
}

fn make_request(config: Config) -> Result<reqwest::Response, reqwest::Error> {
    let repo_payload = json!({
       "name": config.repo_name,
       "description": config.repo_desc,
    });
    let BASE_URL = "https://api.github.com/user/repos";
    let token = env::var("TOKEN").unwrap().to_owned();


    let T = format!("?access_token={}", token);
    let result = format!("{}{}", BASE_URL, T);


    let client = Client::new();
    let resp = client
        .post(&result)
        // .headers(construct_headers())
        .json(&repo_payload)
        .send()?;

    Ok(resp)
}
