#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

fn main() {
    rocket::build().mount("/", routes![index]).launch();
}


