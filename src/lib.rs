use worker::*;
mod routes;

#[event(fetch)]
async fn fetch(req: Request,env: Env,_ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();
    // Response::ok("Hello Shani kushwah you make a first cloudflare program")
   let router = Router::new();
   router
   .get_async("/", routes::hello_world::hello_world)
   .get_async("/read/:id", routes::read::read_student)
   .post_async("/update/:id", routes::update::update_student)
   .delete_async("/delete/:id", routes::delete::delete_student)
   .post_async("/shop", routes::shop_name::shop_name)
   .get_async("/auth", routes::auth::auth)
   .get_async("/token", routes::token::generate_token)
   .get_async("/hello", routes::codewhize::codewhize)
   .run(req, env)
   .await

}