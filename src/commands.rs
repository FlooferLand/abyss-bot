use crate::Context;

pub mod info;

pub async fn pre_command<'a>(ctx: Context<'a>) {
	// Ensuring the user exists in the database
	let uid = ctx.author().id.get() as i64;
	let name = ctx.author().display_name();
	let user = sqlx::query_file!("./assets/database/create_user_if_not_exist.sql", uid, name)
		.fetch_one(&ctx.data().postgres).await;
	if let Ok(user) = user {
		// New user waaaw
	}
}

pub async fn post_command<'a>(ctx: Context<'a>) {

}