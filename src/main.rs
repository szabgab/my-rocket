#[macro_use]
extern crate rocket;

use rocket::response::content;

#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    rocket::info!("some info message");
    content::RawHtml("Hello, world!")
}

#[get("/echo?<text>")]
fn echo(text: String) -> content::RawHtml<String> {
    rocket::info!("Received: {text:?}");
    content::RawHtml(format!("Echo: {text}"))
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, echo])
}


#[cfg(test)]
mod tests;