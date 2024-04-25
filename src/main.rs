#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::response::content;

#[derive(FromForm)]
struct EchoInput<'r> {
    msg: &'r str,
}

#[derive(FromForm)]
struct AddInput {
    first: u8,
    second: u8,
}


#[get("/")]
fn index() -> content::RawHtml<&'static str> {
    rocket::info!("some info message");
    content::RawHtml(
        r#"
    <h2>Echo GET</h2>
    <form action="/gecho">
    <input name="text">
    <input type="submit" value="Echo">
    </form>

    <h2>Echo POST</h2>
    <form action="/pecho" method="POST">
    <input name="msg">
    <input type="submit" value="Echo">
    </form>

    <hr>
    <h2>Add GET</h2>
    <form action="/gadd">
    <input name="first">
    <input name="second">
    <input type="submit" value="Add">
    </form>


    <hr>
    <h2>Add POST</h2>
    <form action="/padd" method="POST">
    <input name="first">
    <input name="second">
    <input type="submit" value="Add">
    </form>

    "#,
    )
}


#[get("/hello")]
fn hello() -> content::RawHtml<&'static str> {
    rocket::info!("some info message");
    content::RawHtml("Hello, world!")
}

#[get("/gecho?<text>")]
fn gecho(text: String) -> content::RawHtml<String> {
    rocket::info!("Received: {text:?}");
    content::RawHtml(format!("Echo: <b>{text}</b>"))
}

#[post("/pecho", data = "<input>")]
fn pecho(input: Form<EchoInput<'_>>) -> content::RawHtml<String> {
    rocket::info!("Received: {:?}", input.msg);
    content::RawHtml(format!("Echo: <b>{}</b>", input.msg))
}

#[get("/gadd?<first>&<second>")]
fn gadd(first: u8, second: u8) -> content::RawHtml<String> {
    rocket::info!("Received: {first:?} {second:?}");
    let result = first + second;
    content::RawHtml(format!("GET Add: {first} + {second} = <b>{result}</b>"))
}


#[post("/padd", data="<input>")]
fn padd(input: Form<AddInput>) -> content::RawHtml<String> {
    rocket::info!("Received: {:?} {:?}", input.first, input.second);
    let result = input.first + input.second;
    content::RawHtml(format!("POST Add: {} + {} = <b>{result}</b>", input.first, input.second))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, gecho, pecho, gadd, padd])
}

#[cfg(test)]
mod tests;
