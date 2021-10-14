#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[allow(unused)]

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;



fn main() {

    rocket::ignite()
    .mount("/me", StaticFiles::from("static"))
    .mount("/dano", StaticFiles::from("dano"))
    .register(catchers![not_found])
    .launch();

}

#[catch(404)]
fn not_found(_req: &Request) -> Redirect {
    Redirect::to("/me/index.html")
}
