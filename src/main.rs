#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use serde::Deserialize;


#[derive(Debug, PartialEq, Eq, Deserialize)]
struct AddInput {
    a: i32,
    b: i32
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/print/<string>")]
fn print_val(string: String) -> String {
    format!("Hello, {}!", string.as_str())
}

#[post("/add", data = "<input>")]
fn add(input: Json<AddInput>) -> String {
    format!("Answer is: {}",  input.a + input.b)
}


fn main() {
    rocket::ignite().mount("/", routes![index, print_val, add]).launch();
}
