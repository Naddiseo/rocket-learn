
use rocket_contrib::Template;
use rocket::http::{Cookie,Cookies, RawStr};
use tera::{Context};

use forms::login;

#[get("/home")]
pub fn home(user: login::AuthContext, mut cookies: Cookies) -> Template {
	let cookie = cookies.get_private("user_id").unwrap();
	let mut context = Context::new();
	context.add("user_id", &user.user_id);
	Template::render("home", &context)
}
