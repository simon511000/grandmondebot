use std::io::Error;

use mc_query::{rcon::RconClient, status};
use tokio::process::Command;

use crate::config::ServerConfig;

#[derive(Debug)]
pub struct Server {
    config: ServerConfig,
}

impl Server {
    pub fn new(config: &ServerConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn nb_players(&self) -> Result<u32, Error> {
        Ok(status(&self.config.host, self.config.query_port)
            .await?
            .players
            .online)
    }

    pub async fn start(&self) -> Result<(), Error> {
        Command::new("docker")
            .arg("start")
            .arg(&self.config.container)
            .output()
            .await?;

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        let mut client = RconClient::new(&self.config.host, self.config.rcon_port).await?;
        client.authenticate(&self.config.rcon_password).await?;

        client.run_command("stop").await?;

        Ok(())
    }
}
