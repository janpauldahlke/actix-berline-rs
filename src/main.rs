#![allow(unused)]

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};

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

// endregion

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

async fn greet(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

async fn greet_handler(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();
    if name.len() > 5 {
        return HttpResponse::Ok().body(format!("Hello {:?}", name));
    } else {
        return HttpResponse::BadRequest().into();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(person_route_params)
            .service(person_route_querry)
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
        let app = test::init_service(App::new().service(person_route_params)).await;
        let request =
            TestRequest::with_uri("/person/PERSONID_1337/comments/COMMENT_ID_42").to_request();

        let response = call_service(&app, request).await;
        assert!(response.status().is_success());

        let body = test::read_body(response).await;
        println!("{:?}", body.clone()); //borrowed
        assert_eq!(body, "route params are, PERSONID_1337 COMMENT_ID_42");
    }
}
