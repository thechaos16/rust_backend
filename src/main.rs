use rocket::form::Form;

#[macro_use] extern crate rocket;

#[derive(FromForm)]
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
fn add(input: From<AddInput>) -> String {
    format!("Answer is: {}",  data)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, print_val, add])
}