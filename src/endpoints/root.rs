pub mod endpoint {
    use crate::WORKDIR;
    use actix_web::{HttpResponse, Responder, get, http::header::ContentType, mime};

    #[get("/")]
    async fn root() -> impl Responder {
        HttpResponse::Ok().body(
            std::fs::read_to_string(format!("{WORKDIR}/public/index.html"))
                .unwrap_or(String::from("")),
        )
    }

    #[get("/style.css")]
    async fn style() -> impl Responder {
        HttpResponse::Ok()
            .insert_header(ContentType(mime::TEXT_CSS))
            .body(
                std::fs::read_to_string(format!("{WORKDIR}/public/style.css"))
                    .unwrap_or(String::from("")),
            )
    }

    #[get("/app.js")]
    async fn js() -> impl Responder {
        HttpResponse::Ok()
            .insert_header(ContentType(mime::TEXT_JAVASCRIPT))
            .body(
                std::fs::read_to_string(format!("{WORKDIR}/public/app.js"))
                    .unwrap_or(String::from("")),
            )
    }
}
