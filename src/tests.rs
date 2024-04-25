use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_hello_world() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/hello").dispatch();

    assert_eq!(
        response.headers().get_one("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}

#[test]
fn test_gecho() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/gecho?text=Foo+Bar").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Echo: <b>Foo Bar</b>".into()));
}

#[test]
fn test_pecho() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client
        .post("/pecho")
        .header(ContentType::Form)
        .body("msg=Foo Bar")
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Echo: <b>Foo Bar</b>".into()));
}
