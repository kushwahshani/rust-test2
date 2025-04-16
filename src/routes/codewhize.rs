use worker::{ Request, Response, Result, RouteContext};

pub async fn codewhize(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {


    Response::ok("This Is Home page")
}