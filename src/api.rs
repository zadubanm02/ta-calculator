use poem::handler;

use crate::services::make_req_for_rsi;

#[handler]
pub async fn get_rsi(coin: String) {
    let url = format!("http://binanceurl/coins/{}", coin);
    let response = make_req_for_rsi(url).await;

    let result = match response {
        
    }
}
