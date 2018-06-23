
use rocket::request::{Form, FromFormValue};
use rocket::http::{Cookie,Cookies, RawStr};

#[derive(Debug)]
pub struct PasswordField<'r>(&'r str);

impl<'v> FromFormValue<'v> for PasswordField<'v> {
	type Error = &'static str;
	
	fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
		if v.len() < 4 {
			Err("too short!")
		}
		else {
			Ok(PasswordField(v.as_str()))
		}
	}
}

#[derive(Debug)]
pub struct EmailField<'r>(&'r str);

impl<'v> FromFormValue<'v> for EmailField<'v> {
	type Error = &'static str;
	fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
		
		
		Err("fail")
	}
}
