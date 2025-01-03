use worker::{ Request, Response, Result, RouteContext};

pub async fn hello_world(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {

    // let url = req.url()?;
    // let shop_name = url.query_pairs()
    //     .find(|(key, _)| key == "shop")
    //     .map(|(_, value)| value)
    //     .unwrap_or_default(); // Default to an empty string if "shop" is not found

    // // Return a response including the received shop name
    // let response = Response::ok(format!("Received shop name: {}", shop_name))?;

    // Ok(response)

    Response::ok("This Is Home page")
}