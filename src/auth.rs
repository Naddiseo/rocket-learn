

use rocket::request::{Form};
use rocket::response::{Redirect,Flash};
use rocket::http::{Cookie, Cookies};
use forms::login::LoginForm;



#[post("/login", data="<login_form>")]
pub fn login<'r>(mut cookies: Cookies, login_form: Form<'r, LoginForm<'r>>) -> Redirect {
	cookies.add_private(Cookie::new("user_id", "123"));
	Redirect::to("/home")
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
	cookies.remove_private(Cookie::named("user_id"));
	Flash::success(Redirect::to("/"), "Successfully logged out")
}
