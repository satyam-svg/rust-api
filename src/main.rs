#[macro_use]
extern crate rocket;



#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[post("/users")]
fn get_create_one() -> &'static str {
    "user created"
}
#[put("/users")]
fn get_put_one() -> &'static str {
    "user put created"
}
#[delete("/users")]
fn get_delete_one() -> &'static str {
    "user delete successfully"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![world])
        .mount("/", routes![get_create_one,get_put_one,get_delete_one])
}
