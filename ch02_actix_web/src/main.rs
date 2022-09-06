use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[post("/gcd")]
async fn post_gcd(params: web::Form<GcdParameters>) -> impl Responder {
    let n = params.n;
    let m = params.m;
    if n == 0 || m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("<title>GCD Calculator</title>Both n and m must be positive");
    }

    let reponse = format!(
        "<title>GCD Calculator</title>The greatest common divisor of the numbers {} and {} is <b>{}</b>",
        n,
        m,
        gcd(n, m)
    );

    HttpResponse::Ok().content_type("text/html").body(reponse)
}

#[get("/")]
async fn calculator() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"<title>GCD Calculator</title>
        <form action=/gcd method=post>
            <input type=text name=n />
            <input type=text name=m />
            <button type=submit>Compute GCD</button>
        </form>"#,
    )
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calculator)
            .service(post_gcd)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
