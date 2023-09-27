use er_admin::bootstrap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bootstrap::app::start().await
}
