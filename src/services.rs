use reqwest::{self, Error, Response, StatusCode};

pub async fn make_req_for_rsi(url: String) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let result = client.get(url).send().await;
    return match result {
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(response),
            StatusCode::NO_CONTENT => Ok(response),
            _ => panic!("Unhandled status code"),
        },
        Err(error) => Err(Error::from(error)),
    };
}

pub async fn make_req_for_ema(url: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let result = client.get(url).send().await;
    return match result {
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(response),
            StatusCode::NO_CONTENT => Ok(response),
            _ => panic!("Unhandled status code"),
        },
        Err(error) => Err(Error::from(error)),
    };
}

pub async fn make_req_for_price(url: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let result = client.get(url).send().await;
    return match result {
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(response),
            StatusCode::NO_CONTENT => Ok(response),
            _ => panic!("Unhandled status code"),
        },
        Err(error) => Err(Error::from(error)),
    };
}
