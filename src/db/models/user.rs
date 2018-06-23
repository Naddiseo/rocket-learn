use db::models::schema::user::dsl::*;

#[derive(Queryable)]
pub struct User {
	pub id: i32,
	pub email: String,
}

impl User {
	/*pub fn by_id(id: &i32, conn: &SqliteConnection) -> &mut User {
		Self::find(id)
	}
	*/
}
