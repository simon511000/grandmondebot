use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn reload_config(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_in_guild(
        ctx,
        &ctx.framework().options().commands,
        ctx.guild_id().unwrap(),
    )
    .await?;

    Ok(())
}
