
use rocket::http::{Status};
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome;


use fields::{EmailField, PasswordField};
use db::models::user;

#[derive(Debug, FromForm)]
pub struct LoginForm<'r> {
	email: Result<EmailField<'r>, &'static str>,
	password: Result<PasswordField<'r>, &'static str>,
}

#[derive(Debug, Clone)]
pub struct AuthContext {
	pub user_id: u64,
	pub is_authenticated: bool,
}

impl AuthContext {
	pub fn new(id: String) -> AuthContext {
		AuthContext { user_id: id.parse().unwrap(), is_authenticated: true }
	}

}

impl<'a, 'r> FromRequest<'a, 'r> for AuthContext {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
		let user_id = request
			.cookies()
			.get_private("user_id")
			.and_then(|cookie| cookie.value().parse().ok())
				.map(AuthContext::new)
			.or(None);
		
		match user_id {
			Some(user) => Outcome::Success(user),
			None => Outcome::Failure((Status::Unauthorized, ()))
		}
			
	}
}
