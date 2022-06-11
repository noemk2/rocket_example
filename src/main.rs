#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};


#[get("/")]
fn index() -> Template {
    Template::render("index", context!{
        title: "vista" ,  
    } )
}

#[get("/hola")]
fn hola() -> &'static str{
    "twitter hola 2"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount( "/", routes![index, hola]).attach(Template::fairing())
}