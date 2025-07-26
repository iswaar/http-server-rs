use crate::security::ratelimit;
use actix_web::{HttpRequest, HttpResponse, Responder, http::StatusCode, web::Data};
use sea_orm::{
    ConnectionTrait,
    DatabaseBackend::MySql as Backend,
    DatabaseConnection, Statement,
    Value::{BigUnsigned, String as Text},
    Values,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn endpoint(database: Data<DatabaseConnection>, request: HttpRequest) -> impl Responder {
    let ip = request // get the client IP (version 4) address
        .connection_info()
        .realip_remote_addr()
        .unwrap()
        .to_string();

    match ratelimit(database.clone(), ip.clone()).await {
        500 => return HttpResponse::InternalServerError().body("The clock has run backwards!!"),
        403 => return HttpResponse::Forbidden().body("Forbidden IP"),
        429 => return HttpResponse::TooManyRequests().body("Too many requests"),
        200 => {}
        _ => todo!("A new response code has come from the rate limit"),
    };

    match database
        .execute(Statement {
            // log the connection ( used by the rate limit in src/security.rs )
            sql: String::from("INSERT INTO iplog VALUES( ? , ? )"),
            values: Some(Values(vec![
                BigUnsigned(Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                )),
                Text(Some(Box::new(ip.clone()))),
            ])),
            db_backend: Backend,
        })
        .await
    {
        Err(x) => println!("error: {x}"),
        _ => {}
    }

    HttpResponse::Ok()
        .status(StatusCode::from_u16(200).unwrap())
        .body("Hello from the api\n")
}
