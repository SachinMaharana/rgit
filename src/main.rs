#[macro_use]
extern crate serde_json;

extern crate dotenv;
extern crate reqwest;

extern crate rgit;

use dotenv::dotenv;
use reqwest::header::{Authorization, Bearer, Headers};
use reqwest::Client;
use rgit::Config;
use std::env;
// use std::process;

const BASE_URL: &'static str = "https://api.github.com/user/repos";

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();

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
                println!("Something else happened. Github Says: {:?}", resp.status());
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

fn construct_headers() -> Headers {
    let token = env::var("TOKEN").unwrap();

    let mut headers = Headers::new();
    headers.set(Authorization(Bearer {
        token: token.to_owned(),
    }));
    headers
}

fn make_request(config: Config) -> Result<reqwest::Response, reqwest::Error> {
    let repo_payload = json!({
       "name": config.repo_name,
       "description": config.repo_desc,
    });
    let client = Client::new();
    let resp = client
        .post(BASE_URL)
        .headers(construct_headers())
        .json(&repo_payload)
        .send()?;

    Ok(resp)
}

// fn run() -> Result<(), reqwest::Error> {
//     let gist_body = json!({
//        "name": "Ok",
//        "description": "This is your first repository",
//         });

//     let resp = Client::new()
//         .post(BASE_URL)
//         .headers(construct_headers())
//         .json(&gist_body)
//         .send()?;
//     if resp.status().is_success() {
//         println!("success!");
//     } else if resp.status().is_server_error() {
//         println!("server error!");
//     } else {
//         println!("Something else happened. Status: {:?}", resp.status());
//     }

//     // match resp.status() {
//     //     StatusCode::Ok => println!("success!"),
//     //     StatusCode::UnprocessableEntity => {
//     //         println!("Already Exist");
//     //     }
//     //     s => println!("Received response status: {:?}", s),
//     // };
//     Ok(())
// }
