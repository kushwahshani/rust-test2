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
    installation: i64,
}

pub async fn auth(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // console_log!("this is auth function");
    // let client_id = "fe56c0cf0d804e83ddbbce365e1c2353";
    // let scope = "read_products";
    // let redirect_url = "https://test2.shanikushwahonline.workers.dev/token";
    // Get D1 database binding
    let url = req.url()?;
    let d1 = ctx.env.d1("DB")?;

    // // Extract the shop ID from the query parameters
    let shop_name = url
        .query_pairs()
        .find(|(key, _)| key == "shop")
        .map(|(_, value)| value);

    
    // if shop_name.is_empty() {
        //     return Response::error("Shop name is required", 400);
        // }
        
        if let Some(shop_name) = shop_name {
            let check_query = "SELECT shop, auth_token, installation FROM shops WHERE shop = ?";
                let statement = d1
                    .prepare(check_query)
                    .bind(&["ac-dev-25.myshopify.com".to_string().into()])?;
                let query_result = statement.first::<Shop>(None).await?;
                    //  let results: Option<Shop> = statement.first(None).await?;
                     if let Some(value) = query_result {
                        return Response::from_json(&json!({
                            "status": "success",
                            "message": "this is a result",
                            "auth_token": shop_name
                        }));
                     }else {
                         return Response::ok("this is query result:");
                     }
            return Response::from_json(&json!({
                "status": "success",
                "message": "this is shop name",
                "auth_token": shop_name
            }));
        }
        return Response::from_json(&json!({
            "status": "success",
            "message": "Token generated and shop installed",
            "auth_token": shop_name
        }));
    //     console_log!("test 2{}", shop_name.clone());


    //     // Check if the shop is already installed
    //     // let check_query = "SELECT shop, auth_token, installation FROM shops WHERE shop = ?";

   

    //     console_log!("Query result: {:?}", results);
    //     return Response::ok("shop name");
    // }
    // else {
    //     return Response::error("this is an error", 400);
    // }
    // if let Some(shop) = query_result {
    //     if shop.installation == 1.0 {
    //         let redirect_url = Url::parse("https://shopify-test1.pages.dev/home")?;
    //         return Response::redirect(redirect_url);
    //     } else {
    //         return Response::error("Shop exists but is not installed", 400);
    //     }
    // } else {
    //     let shop_url = Url::parse(&format!(
    //                     "https://{}/admin/oauth/authorize?client_id={}&scope={}&redirect_uri={}&state=nonce",
    //                     shop_name, client_id, scope, redirect_url
    //                 ))?;
    //     return Response::redirect(shop_url);
    // }

    // Return a response including the received shop name
    let response = Response::ok(format!("Received shop name:"))?;

    Ok(response)
}
