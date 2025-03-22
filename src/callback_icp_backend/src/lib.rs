use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
        TransformArgs, TransformContext,
    },
    query, update,
};
use serde_json::Value;

//Update method with the HTTPS OutCalls methods feature
// curl --request GET \
//      --url 'https://api.coingecko.com/api/v3/coins/internet-computer?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false' \
//      --header 'accept: application/json' \
//      --header 'x-cg-demo-api-key: CG-fmzdzZXRherhMZb2u2kB9PCz'

#[update]
async fn get_icp_price() -> String {
    let host = "api.coingecko.com";

    // --url 'https://api.coingecko.com/api/v3/coins/internet-computer?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false'\

    let url = format!(
        "https://{}/api/v3/coins/internet-computer?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false",
        host,
    );

    //Prep  headers for the sys-http_req_call
    let req_headers = vec![HttpHeader {
        name: "User-Agent".to_string(),
        value: "application/json".to_string(),
    }];
            
    let req = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: Some(TransformContext::from_name("transform".to_string(), vec![])),
        headers: req_headers,
    };

    // Provide a cycles amount (e.g., 100_000_000 cycles as a placeholder)
    let cycles: u128 = 1_603_142_800;

    // Make the HTTPS req and wait for response
    match http_request(req, cycles).await {
        // Ok((response,)) => {
        //     let str_body =
        //         String::from_utf8(response.body).expect("response body is not valid utf8");

        //     //Return the body as a string and end the method
        //     str_body
        // }
        // Err((r, m)) => {
        //     let message =
        //         format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

        //     //Return the error as a string and end the method
        //     message
        // }
        Ok((response,)) => {
            let str_body =
                String::from_utf8(response.body).expect("Response body is not alid utf8");
            match serde_json::from_str::<Value>(&str_body) {
                Ok(json) => {
                    if let Some(price) = json["market_data"]["current_price"]["usd"].as_f64() {
                        return format!("ICP Price: ${}", price);
                    } else {
                        "Price data not found".to_string()
                    }
                }
                Err(_) => "Invalid JSON response".to_string(),
            }
        }
        Err((r, m)) => format!("HTTP request failed. RejectionCode: {:?}, Error: {}", r, m),
    }
}




// curl --request GET \
//      --url 'https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=internet-computer' \
//      --header 'accept: application/json' \
//      --header 'x-cg-demo-api-key: CG-fmzdzZXRherhMZb2u2kB9PCz'

// #[update]
// fn exchange() -> String{

// }
#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];

    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
        ..Default::default()
    };

    // Fix: Convert `res.status` to a comparable type
    // if res.status == candid::Nat::from(200u64)
    if res.status == candid::Nat::from(200u64) {
        res.body = raw.response.body; // Keep original body if status is 200
    } else {
        ic_cdk::api::print(
            format!("Received an error from CoinGecko: Status: {}, Body: {:?}",res.status, raw.response.body)
        );
    }
    res
}

//Enable candid export
ic_cdk::export_candid!();