extern crate colored;

use chrono::DateTime;
use colored::*;
use reqwest::blocking::{Client, Request};
use reqwest::Method;
use serde_json::{from_str as json_from_str, Value as JsonValue};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let country = &args[1].to_string();
    let api_covid19 = format!("https://covid19.mathdro.id/api/countries/{}", country);
    let request = Request::new(Method::GET, api_covid19.parse().expect("Bad api_covid19"));
    let client = Client::new();
    let response = client.execute(request).unwrap();
    let response_json: JsonValue = json_from_str(&response.text().unwrap()).unwrap();
    let response_map = response_json.as_object().unwrap().to_owned();
    println!(
        "Update: {}",
        DateTime::parse_from_rfc3339(response_map["lastUpdate"].as_str().unwrap()).unwrap()
    );
    println!(
        "Confirmed: {}",
        response_map["confirmed"].as_object().unwrap()["value"]
            .as_i64()
            .unwrap()
            .to_string()
            .yellow()
    );
    println!(
        "Recovered: {}",
        response_map["recovered"].as_object().unwrap()["value"]
            .as_i64()
            .unwrap()
            .to_string()
            .green()
    );
    println!(
        "Deaths: {}",
        response_map["deaths"].as_object().unwrap()["value"]
            .as_i64()
            .unwrap()
            .to_string()
            .red()
    );
    Ok(())
}
