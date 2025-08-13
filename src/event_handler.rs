use crate::serenity;
use log::debug;
use poise::FrameworkError;
use poise::serenity_prelude::{FullEvent, MessageBuilder};
use crate::{send_message_str, BotData};
use crate::error::BotError;

pub async fn error_handler(error: FrameworkError<'_, BotData, BotError>) {
	match error {
		FrameworkError::Command { error, ctx, .. } => {
			let text = match error {
				BotError::String(value) => value,
				BotError::Str(value) => value.to_string(),
				e => MessageBuilder::new().push_mono(e.to_string()).build()
			};
			let message = MessageBuilder::new()
				.push_bold("ERROR:").push(" ").push_safe(&text)
				.build();
			debug!("Skill issue [{}]: \"{text}\"", ctx.author().name);
			send_message_str!(ctx, message).unwrap();
		},
		error => poise::builtins::on_error(error).await.unwrap(),
	};
}


pub async fn event_handler(
	ctx: &serenity::Context,
	event: &FullEvent,
	_framework: poise::FrameworkContext<'_, BotData, BotError>,
	data: &BotData,
) -> Result<(), BotError> {
	match event {
		_ => {}
	}
	Ok(())
}