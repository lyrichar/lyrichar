use std::{fmt, io::Error as IoError};

use axum::serve;
use clap::Parser;
use thiserror::Error;
use tokio::{
    net::TcpListener,
    runtime::{Builder, Runtime},
};

use crate::routes;

pub type Port = u16;

pub const DEFAULT_HOST: &str = "0.0.0.0";
pub const DEFAULT_PORT: Port = 1313;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct App {
    #[arg(long, short = 'H', help = "Run on this host", default_value_t = DEFAULT_HOST.to_owned())]
    pub host: String,

    #[arg(long, short = 'P', help = "Run on this port", default_value_t = DEFAULT_PORT)]
    pub port: Port,
}

#[derive(Debug, Error)]
#[error("failed to build runtime: {error}")]
pub struct BuildError {
    #[from]
    error: IoError,
}

impl BuildError {
    pub const fn new(error: IoError) -> Self {
        Self { error }
    }

    pub fn get(self) -> IoError {
        self.error
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address {
    pub host: String,
    pub port: Port,
}

impl Address {
    pub const fn new(host: String, port: Port) -> Self {
        Self { host, port }
    }

    pub fn into_pair(self) -> (String, Port) {
        (self.host, self.port)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{host}:{port}",
            host = self.host,
            port = self.port
        )
    }
}

#[derive(Debug, Error)]
#[error("failed to bind to `{address}`; reason: {error}")]
pub struct BindError {
    error: IoError,
    address: Address,
}

#[derive(Debug, Error)]
#[error("failed to serve: {error}")]
pub struct ServeError {
    #[from]
    error: IoError,
}

impl ServeError {
    pub const fn new(error: IoError) -> Self {
        Self { error }
    }

    pub fn get(self) -> IoError {
        self.error
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RunError {
    Bind(#[from] BindError),
    Serve(#[from] ServeError),
}

impl BindError {
    pub const fn new(error: IoError, address: Address) -> Self {
        Self { error, address }
    }

    pub fn get(self) -> (IoError, Address) {
        (self.error, self.address)
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Build(#[from] BuildError),
    Run(#[from] RunError),
}

impl App {
    pub fn runtime() -> Result<Runtime, BuildError> {
        let runtime = Builder::new_multi_thread().enable_all().build()?;

        Ok(runtime)
    }

    pub async fn async_run(self) -> Result<(), RunError> {
        let address = Address::new(self.host, self.port);

        let cloned = address.clone();

        let listener = TcpListener::bind(cloned.into_pair())
            .await
            .map_err(|error| BindError::new(error, address))?;

        let router = routes::router();

        serve(listener, router).await.map_err(ServeError::new)?;

        Ok(())
    }

    pub fn run(self) -> Result<(), Error> {
        let runtime = Self::runtime()?;

        runtime.block_on(self.async_run())?;

        Ok(())
    }
}
