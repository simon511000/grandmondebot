use std::io::Error;

use mc_query::{rcon::RconClient, status};
use tokio::process::Command;

use crate::config::ServerConfig;

#[derive(Debug)]
pub struct Server {
    config: ServerConfig,

    client: RconClient,
}

impl Server {
    pub async fn new(config: &ServerConfig) -> Result<Self, Error> {
        let mut client = RconClient::new(&config.host, config.rcon_port).await?;
        client.authenticate(&config.rcon_password).await?;

        Ok(Self {
            config: config.clone(),
            client,
        })
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
        self.client.run_command("stop").await?;

        Ok(())
    }
}
