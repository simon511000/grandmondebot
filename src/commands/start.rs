use crate::{config::CONFIG, Context, Error};

/// Démarrer le serveur minecraft
#[poise::command(slash_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    if let Ok(_nb_players) = CONFIG.server.nb_players().await {
        ctx.say("Le serveur est déjà démarré!").await?;

        return Ok(());
    }

    CONFIG.server.start().await?;

    ctx.say("Démarrage du serveur en cours...").await?;

    Ok(())
}
