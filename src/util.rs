use poise::CreateReply;
use crate::Context;

/// Discord has an annoying need for apps to send a reply message to users.
/// Calling this function at the end of a command removes that
pub async fn consume_interaction<'a>(ctx: Context<'a>) {
	let _ = ctx.defer_ephemeral().await;
	let message = ctx.send(CreateReply::default().content("-# Sent!").ephemeral(true)).await;
	if let Ok(message) = message {
		message.delete(ctx).await.unwrap()
	}
}

#[macro_export]
macro_rules! send_message {
    ($ctx:expr, $create_reply:expr) => {{
	    let reply: poise::CreateReply = $create_reply;
	    let mut reply = reply.reply(true);

	    let bots_channel = $crate::read_server!($ctx, channels => { channels.bots });
	    if let Some(channel) = bots_channel {
			if $ctx.channel_id().get() == channel {
				reply.ephemeral = Some(false);
			}
		}
	    $ctx.send(reply).await.bot_err()
    }};
}
#[macro_export]
macro_rules! send_message_str {
    ($ctx:expr, $text:expr) => {{
	    use crate::error::{BotErrorExt};
	    let reply = poise::CreateReply::default().content($text).reply(true);

	    // TODO: Add bot channel support back in; Store server configs via SQL
	    /*let bots_channel = $crate::read_server!($ctx, channels => { channels.bots });
	    if let Some(channel) = bots_channel {
		    if $ctx.channel_id().get() == channel {
			    reply.ephemeral = Some(false);
		    }
	    }*/
	    $ctx.send(reply).await.bot_err()
    }};
}
