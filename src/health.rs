use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use tracing::instrument;

#[instrument]
async fn health_check(index: web::Data<u16>) -> HttpResponse {
    HttpResponse::Ok()
        .header("thread_id", index.to_string())
        .finish()
}

#[instrument(skip(cfg))]
pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

#[cfg(test)]
mod tests {
    use crate::health::{health_check, service};
    use actix_web::{
        http::StatusCode,
        test::{call_service, init_service, TestRequest},
        web, App,
    };

    #[actix_rt::test]
    async fn health_check_works() {
        let res = health_check(web::Data::new(5)).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
        let data = res
            .headers()
            .get("thread_id")
            .map(|h| h.to_str().ok())
            .flatten();
        assert_eq!(data, Some("5"));
    }

    #[actix_rt::test]
    async fn health_check_integration_works() {
        let app = App::new().app_data(web::Data::new(5u16)).configure(service);
        let mut app = init_service(app).await;
        let req = TestRequest::get().uri("/health").to_request();

        let res = call_service(&mut app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
    }
}
