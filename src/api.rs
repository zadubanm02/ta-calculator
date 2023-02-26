use poem::{handler, http::StatusCode, web::Json, Error, IntoResponse};

use crate::services::make_req_for_rsi;

#[handler]
pub async fn get_rsi(coin: String) -> Result<impl IntoResponse, Error> {
    let url = format!("http://binanceurl/coins/{}", coin);
    let response = make_req_for_rsi(url).await;

    return match response {
        Ok(res) => {
            let json: serde_json::Value = res
                .json()
                .await
                .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR);
            return Ok((StatusCode::from_u16(200).unwrap(), Json(json)));
        }
        Err(err) => Err(Error::from_string(
            "Unexpected error",
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    };
}
