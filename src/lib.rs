use chrono::DateTime;
use colored::*;
use prettytable::{cell, row, Table};
use reqwest::blocking::{Client, Request};
use reqwest::Method;
use serde_json::{from_str as json_from_str, Value as JsonValue};

fn get_covid19_api(country: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!(
        "https://covid19.mathdro.id/api/countries/{}",
        country
    ))
}

fn handle_fetch_request(
    url: String,
) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    Ok(Client::new()
        .execute(Request::new(Method::GET, url.parse().unwrap()))
        .unwrap())
}

pub fn init(country: String) {
    let response = handle_fetch_request(get_covid19_api(country).unwrap()).unwrap();
    let response_json: JsonValue = json_from_str(&response.text().unwrap()).unwrap();
    let response_map = response_json.as_object().unwrap().to_owned();

    let mut table = Table::new();

    table.add_row(row![
        "Last Update",
        DateTime::parse_from_rfc3339(response_map["lastUpdate"].as_str().unwrap()).unwrap()
    ]);

    table.add_row(row![
        "Confirmed",
        response_map["confirmed"].as_object().unwrap()["value"]
            .as_i64()
            .unwrap()
            .to_string()
            .yellow()
    ]);

    table.add_row(row![
        "Recovered",
        response_map["recovered"].as_object().unwrap()["value"]
            .as_i64()
            .unwrap()
            .to_string()
            .green()
    ]);

    table.add_row(row![
        "Deaths",
        response_map["deaths"].as_object().unwrap()["value"]
            .as_i64()
            .unwrap()
            .to_string()
            .red()
    ]);

    table.printstd();
}
