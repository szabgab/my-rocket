#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    rocket::info!("some info message");
    content::RawHtml(r#"<form action="/echo">
    <input name="text">
    <input type="submit" value="Echo">
    </form>
    "#)
}


#[get("/hello")]
fn hello() -> content::RawHtml<&'static str> {
    rocket::info!("some info message");
    content::RawHtml("Hello, world!")
}

#[get("/echo?<text>")]
fn echo(text: String) -> content::RawHtml<String> {
    rocket::info!("Received: {text:?}");
    content::RawHtml(format!("Echo: <b>{text}</b>"))
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, echo])
}


#[cfg(test)]
mod tests;