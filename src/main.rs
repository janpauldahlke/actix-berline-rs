#![allow(unused)]
use actix_web::{
    get,
    guard::{self, Guard, GuardContext},
    http::{self, header::HeaderValue},
    web::{self, Path},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};

#[derive(Debug, serde::Deserialize)]
pub struct PersonQuery {
    name: String,
    location: String,
    age: u16,
}

#[derive(Debug, serde::Deserialize)]
pub struct PersonParams {
    id: String,
    comment_id: String,
}

#[get("/person")]
async fn person_route_querry(query: web::Query<PersonQuery>) -> HttpResponse {
    let result = format!(
        "route query params are, name{} {} {}",
        query.name, query.location, query.age
    );
    HttpResponse::Ok().body(result)
}
#[get("/person/{id}/comments/{comment_id}")]
async fn person_route_params(params: web::Path<PersonParams>) -> HttpResponse {
    let args = params.into_inner();
    println!("{:?}", args);
    let result_text = format!("route params are, {} {}", args.id, args.comment_id);
    HttpResponse::Ok().body(result_text)
}

#[get("/person/{name}")]
async fn greet_handler(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();
    if name.len() > 5 {
        return HttpResponse::Ok().body(format!("Hello {:?}", name));
    } else {
        return HttpResponse::BadRequest().into();
    }
}

#[get("/ferris")]
async fn display_ferris() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../client/index.html"))
}

// imaginge open ssl or something
const _SECRET: &str = "HIDDEN";
const _HEADER: &str = "X-SECRET";

//FIXME: this is not annotated with #[get("/")] so we can use web::get()::to() later

async fn guarded_route() -> HttpResponse {
    HttpResponse::Ok().body(format!("From guarded , you are authorized!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .service(person_route_params)
            .service(person_route_querry)
            .service(greet_handler)
            .service(display_ferris)
            .service(
                web::resource("/guarded").route(
                    web::route()
                        //.guard(guard::Get())
                        .guard(guard::Header(_HEADER, _SECRET))
                        .to(guarded_route),
                ),
            )
    })
    .bind("127.0.0.1:8080")?;

    server.run().await
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{
        test::{self, call_service, TestRequest},
        web, App,
    };

    #[actix_web::test]
    async fn test_person_route_querry() {
        let app = test::init_service(App::new().service(person_route_querry)).await;
        let request =
            TestRequest::with_uri("/person?name=actix&location=world&age=43").to_request();

        let response = call_service(&app, request).await;
        assert!(response.status().is_success());

        let body = test::read_body(response).await;
        println!("{:?}", body.clone()); //borrowed
        assert_eq!(body, "route query params are, nameactix world 42");
    }

    #[actix_web::test]
    async fn test_person_route_params() {
        let app = test::init_service(App::new().service(web::resource("/guarded"))).await;
        let request =
            TestRequest::with_uri("/person/PERSONID_1337/comments/COMMENT_ID_42").to_request();

        let response = call_service(&app, request).await;
        assert!(response.status().is_success());

        let body = test::read_body(response).await;
        println!("{:?}", body.clone()); //borrowed
        assert_eq!(body, "route params are, PERSONID_1337 COMMENT_ID_42");
    }

    #[actix_web::test]
    async fn test_guarded_route_with_secret() {
        let app = test::init_service(
            App::new().service(
                web::resource("/guarded").route(
                    web::route()
                        .guard(guard::Get())
                        .guard(guard::Header(_HEADER, _SECRET))
                        .to(guarded_route),
                ),
            ),
        )
        .await;

        let request = TestRequest::default()
            .uri("/guarded")
            .insert_header((_HEADER, _SECRET))
            .to_request();

        let response = call_service(&app, request).await;

        // let body = test::read_body(response).await;
        // println!("{:?}", body.clone()); //borrowed
        // assert_eq!(body, "From guarded , you are authorized!");

        assert!(response.status().is_success()); //200
    }

    #[actix_web::test]
    async fn test_guarded_route_without_secret() {
        let app = test::init_service(App::new().service(person_route_params)).await;
        let request =
            //TestRequest::with_uri("/person/PERSONID_1337/comments/COMMENT_ID_42").to_request();
            TestRequest::default()
            .uri("/guarded")
            .insert_header(("X-SECRET" , "WRONK_SECRET"))
            .to_request();

        let response = call_service(&app, request).await;
        assert!(response.status().is_client_error()); //405
    }
}
