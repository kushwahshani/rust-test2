use crate::wasm_bindgen::JsValue;
// use reqwest::Client;
use serde::{Deserialize, Serialize};
use worker::{Request, Response, Result, RouteContext, Url};

#[derive(Deserialize, Serialize)]
struct Shop {
    shop: String,
    auth_token: String,
    installation: f32,
}

pub async fn auth(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let client_id = "fe56c0cf0d804e83ddbbce365e1c2353"; 
    let scope = "read_products";
    let redirect_url = "https://test2.shanikushwahonline.workers.dev/token";
    // Get D1 database binding
    let d1 = ctx.env.d1("DB")?;

    // Extract the shop ID from the query parameters
    let url = req.url()?;
    let shop_name = url
        .query_pairs()
        .find(|(key, _)| key == "shop")
        .map(|(_, value)| value.to_string())
        .unwrap_or_default(); // Default to an empty string if "shop" is not found

    if shop_name.is_empty() {
        return Response::error("Shop name is required", 400);
    }

    // Check if the shop is already installed
    let check_query = "SELECT shop, auth_token, installation FROM shops WHERE shop = ?";
    let statement = d1
        .prepare(check_query)
        .bind(&[JsValue::from(shop_name.clone())])?;
    let query_result = statement.first::<Shop>(None).await?;

    if let Some(shop) = query_result {
        if shop.installation == 1.0 {
            let redirect_url = Url::parse("https://shopify-test1.pages.dev/home")?;
            return Response::redirect(redirect_url);
        } else {
            return Response::error("Shop exists but is not installed", 400);
        }
    } else {
        let shop_url = Url::parse(&format!(
                        "https://{}/admin/oauth/authorize?client_id={}&scope={}&redirect_uri={}&state=nonce",
                        shop_name, client_id, scope, redirect_url
                    ))?;
        return Response::redirect(shop_url);
    }

    // Return a response including the received shop name
    // let response = Response::ok(format!("Received shop name: {}", shop_name))?;

    // Ok(response)
}
