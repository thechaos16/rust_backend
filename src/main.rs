#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use serde::Deserialize;

mod path_handler;
use path_handler::get_urlx_urly;
use path_handler::get_path;


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
    let start_point = get_urlx_urly(37.556757, 127.028571);
    let end_point = get_urlx_urly(37.556124, 127.031843);
    println!("{:?}", get_path(start_point, end_point));
    format!("Hello, {}!", string.as_str())
}

#[post("/add", data = "<input>")]
fn add(input: Json<AddInput>) -> String {
    format!("Answer is: {}",  input.a + input.b)
}


fn main() {
    rocket::ignite().mount("/", routes![index, print_val, add]).launch();
}
