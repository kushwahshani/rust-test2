// use crate::wasm_bindgen::JsValue;
// use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::{console_log, Request, Response, Result, RouteContext, Url};

// use super::shop_name;

#[derive(Deserialize, Serialize, Debug)]
struct Shop {
    shop: String,
    auth_token: String,
    installation: i32,
}

pub async fn auth(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // console_log!("this is auth function");
    let client_id = "fe56c0cf0d804e83ddbbce365e1c2353";
    let scope = "read_products,read_orders,write_orders,write_products";
    let redirect_url = "https://test2.shanikushwahonline.workers.dev/token";
    let redirect_home = "https://shopify-test1.pages.dev/home";
    // Get D1 database binding
    let url = req.url()?;
    let d1 = ctx.env.d1("DB")?;

    // // Extract the shop ID from the query parameters
    let shop_name = url
        .query_pairs()
        .find(|(key, _)| key == "shop")
        .map(|(_, value)| value);
    

    if let Some(shop) = shop_name {
        let check_query = "SELECT shop, auth_token, installation FROM shops WHERE shop = ?";
        let statement = d1
            .prepare(check_query)
            .bind(&[shop.to_string().into()])?;
        
        // .bind(&["ac-dev-25.myshopify.com".to_string().into()])?;
        // let query_result = statement.first::<Shop>(None).await?;
        //  let query_result: Option<Shop> = statement.first(None).await?;

        let query_result: Option<Shop> = match statement.first(None).await {
            Ok(result) => result,
            Err(e) => {
                console_log!("Error executing query: {:?}", e);
                return Response::error("Database query failed", 500);
            }
        };
        // Check app install or not
        // console_log!("simple chack: {:?}", query_result);
        if let Some(shop_value) = query_result {
            // console_log!("this is query result{:?}", &shop_value.installation);
            if shop_value.installation == 1 {
                return Response::redirect(Url::parse(&redirect_home)?);
            } else {
                return Response::error("Shop exists but is not installed", 400);
            }

        } else {
            let shop_url = Url::parse(&format!(
                                    "https://{}/admin/oauth/authorize?client_id={}&scope={}&redirect_uri={}&state=nonce",
                                    shop, client_id, scope, redirect_url
                                ))?;
            return Response::redirect(shop_url);
        }
    }
    return Response::from_json(&json!({
        "status": "error",
        "message": "missing shop query perameter",
        // "shop name": shop_name

    }));
}
