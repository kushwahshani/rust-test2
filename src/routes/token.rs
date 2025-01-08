use crate::wasm_bindgen::JsValue;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;
use worker::{console_log, Request, Response, Result, RouteContext};

// access token struct
#[derive(Deserialize, Serialize)]
struct TokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    // scope: String
}


// Shopify OAuth Access Token
pub async fn generate_token(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let redirect_home = "https://shopify-test1.pages.dev/home";
    // Extract the required query perameters
    let url = req.url()?;
    let shop = url
        .query_pairs()
        .find(|(key, _)| key == "shop")
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();

    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();

    // println!("this is a token {:?}",code);

    if shop.is_empty() || code.is_empty() {
        return Response::error("Missing required query parameter: code and shop ", 400);
    }

    //shopify client credentials
    let client_id = "fe56c0cf0d804e83ddbbce365e1c2353".to_string();
    let client_secret = "2d0ba24b5141f43dca919eca02878a2a".to_string();

    //Token endpoint URL for the given shop

    let token_url = format!("https://{}/admin/oauth/access_token", shop);

    // create a request body

    let token_request = TokenRequest {
        client_id,
        client_secret,
        code,
    };

    // make a post request to shopify api
    let client = Client::new();
    let response = client.post(&token_url).json(&token_request).send().await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                // Parse the token response
                let token_response: TokenResponse = resp.json().await.unwrap();
                let access_token = token_response.access_token;
                // return Response::ok(format!("Access Token: {}", access_token ));
                console_log!("Access Token: {:?}",access_token);

                // // record insert in database like a shop name and token instalation true and false
                let d1 = ctx.env.d1("DB")?;
                let insert_query =
                    "INSERT INTO shops (shop, auth_token, installation) VALUES (?, ?, ?)";
                d1.prepare(insert_query)
                    .bind(&[JsValue::from(shop), JsValue::from(access_token.clone()), JsValue::from(true)])?
                    .run()
                    .await?;
                return Response::redirect(Url::parse(&redirect_home)?);
            } else {
                return Response::error("Failed to fetch access token", 404);
            }
        }
        Err(err) => {
            eprintln!("Error fetching access token: {}", err);
            return Response::error("Internal Server Error", 500);
        }
    }
}
