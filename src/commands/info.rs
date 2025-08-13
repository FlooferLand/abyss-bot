use crate::error::BotError;
use crate::{send_message_str, Context};
use indoc::formatdoc;

/// Some information about teh bot!
#[poise::command(slash_command, rename="info")]
pub async fn bot_info(ctx: Context<'_>) -> Result<(), BotError> {
	let users = sqlx::query!("SELECT count(*) FROM users")
		.fetch_one(&ctx.data().postgres).await.unwrap();
	let message = formatdoc!(
        r#"
            Silly bot made by FlooferLand!
            Repo: <https://github.com/FlooferLand/abyss-bot>
            Version: `{Version}`
            Users: `{Users}`
        "#,
        Version = env!("CARGO_PKG_VERSION"),
		Users = users.count.unwrap()
    );
	send_message_str!(ctx, message)?;
	Ok(())
}