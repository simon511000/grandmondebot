use crate::{Context, Error};

/// Démarrer le serveur minecraft
#[poise::command(slash_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    let server = ctx.data().server.lock().await;

    if let Ok(_nb_players) = server.nb_players().await {
        ctx.say("Le serveur est déjà démarré!").await?;

        return Ok(());
    }

    server.start().await?;

    ctx.say("Démarrage du serveur en cours...").await?;

    Ok(())
}
