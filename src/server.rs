use std::io::Error;

use mc_query::{rcon::RconClient, status};
use serde::Deserialize;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
pub struct Server {
    container: String,
    host: String,
    query_port: u16,
    rcon_port: u16,

    #[serde(skip)]
    client: Option<RconClient>,
}

impl Server {
    pub async fn nb_players(&self) -> Result<u32, Error> {
        Ok(status(&self.host, self.query_port).await?.players.online)
    }

    pub async fn start(&self) -> Result<(), Error> {
        Command::new("docker")
            .arg("start")
            .arg(&self.container)
            .output()
            .await?;

        Ok(())
    }
}
