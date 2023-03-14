use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};

// a mvp of a web server
#[derive(Debug, serde::Deserialize)]
pub struct PersonParams {
    name: String,
    location: String,
    age: u16,
}

// endregion

#[get("/foo")]
async fn foo_querry_params(query: web::Query<PersonParams>) -> HttpResponse {
    let result = format!(
        "route params are, name{} {} {}",
        query.name, query.location, query.age
    );
    HttpResponse::Ok().body(result)
}
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
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
    let server =
        HttpServer::new(|| App::new().service(foo_querry_params)).bind("127.0.0.1:8080")?;

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
    async fn test_foo_querry_params() {
        let app = test::init_service(App::new().service(foo_querry_params)).await;
        let request = TestRequest::with_uri("/foo?name=actix&location=world&age=43").to_request();

        let response = call_service(&app, request).await;
        assert!(response.status().is_success());

        let body = test::read_body(response).await;
        println!("{:?}", body.clone()); //borrowed
        assert_eq!(body, "route params are, nameactix world 43");
    }
}
