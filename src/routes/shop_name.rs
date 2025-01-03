use worker::{Request, Response, Result, RouteContext};



pub async fn shop_name( _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    
    // // Handle the OPTIONS preflight request
    // if req.method() == Method::Options {
    //     let mut headers = Headers::new();
    //     headers.set("Access-Control-Allow-Origin", "*")?; // Allow all origins (use specific origins for production)
    //     headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?; // Allow methods
    //     headers.set("Access-Control-Allow-Headers", "Content-Type")?; // Allow headers

    //     let response = Response::ok("")?.with_headers(headers); // Use with_headers to add CORS headers
    //     return Ok(response); // Return the response
    // }

    // // Parse the JSON body and extract the shop_name
    // let body = req.json::<InputData>().await?;

    // // Display the received shop name in the backend
    // println!("Received shop name: {}", body.shop_name);

    // // Return a response including the received shop name
    // let mut headers = Headers::new();
    // headers.set("Access-Control-Allow-Origin", "*")?;

    // // Return a simple response with the shop name
    // let response =
    //     Response::ok(format!("Received shop name: {}", body.shop_name))?.with_headers(headers);
    // Ok(response)
    Response::ok("This Is shop home page")
}
