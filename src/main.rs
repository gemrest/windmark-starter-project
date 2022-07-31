use windmark::Response;

#[windmark::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    windmark::Router::new()
        .set_private_key_file("windmark_starter_project_private.pem")
        .set_certificate_file("windmark_starter_project_public.pem")
        .mount("/", Box::new(|_| Response::Success("Hello, World!".into())))
        .set_error_handler(Box::new(|_| {
            Response::PermanentFailure("This route does not exist!".into())
        }))
        .run()
        .await
}
