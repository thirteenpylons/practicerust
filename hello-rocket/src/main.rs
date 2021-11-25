#[macro_use] extern crate rocket;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, shop])
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/shop")]
fn shop() -> &'static str {
    "Welcome to the shop!"
}


