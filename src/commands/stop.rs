use crate::{Context, Error};

/// Arrêter le serveur minecraft
#[poise::command(slash_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    let mut server = ctx.data().server.lock().await;

    if let Ok(nb_players) = server.nb_players().await {
        if nb_players > 0 {
            ctx.say(format!(
                "Impossible d'arrêter le serveur, car il y a {} joueurs en ligne.",
                nb_players
            ))
            .await?;

            return Ok(());
        }
    } else {
        ctx.say("Le serveur est déjà arrêté.").await?;

        return Ok(());
    }

    server.stop().await?;

    ctx.say("Arrêt du serveur en cours...").await?;

    Ok(())
}
