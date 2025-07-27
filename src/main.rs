mod endpoints;
mod environment;
mod security;
mod tests;
use crate::environment::retreive::{
    mariadb::{get_database, get_host, get_password, get_user},
    system::get_ip,
};
use actix_web::{App, HttpServer, web};
use endpoints::api::endpoint as api;
use endpoints::root::endpoint as root;
use std::net::IpAddr;

const WORKDIR: &str = env!("PWD");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host: String = get_host();
    let user: String = get_user();
    let password: String = get_password();
    let database_name: String = get_database();
    let base: IpAddr = get_ip();

    let database_url = format!("mysql://{user}:{password}@{host}/{database_name}");
    let conn = match sea_orm::Database::connect(database_url).await {
        Err(err) => {
            println!("connection error: {err}");
            return Ok(());
        }
        Ok(x) => x,
    };

    println!("Started on {base}");
    HttpServer::new(move || {
        App::new()
            .service(root::root)
            .service(root::style)
            .service(root::js)
            .app_data(web::Data::new(conn.clone()))
            .route("/api", web::get().to(api))
    })
    .bind((base, 8080))?
    .run()
    .await
}
