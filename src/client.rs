use std::{io, result, time::Duration};

use quick_error::quick_error;
use reqwest::blocking::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder};

use crate::Pkcs12Certificate;

pub const CLIENT_CONNECT_TIMEOUT: u64 = 5;

pub const CLIENT_TIMEOUT: u64 = 30;

quick_error! {
    #[derive(Debug)]
    pub enum ClientError {
        Io(err: io::Error) {
            from()
            display("Erro de I/O no client HTTP: {}", err)
        }
        Reqwest(err: reqwest::Error) {
            from()
            display("Erro no client HTTP: {}", err)
        }
    }
}

pub type ClientResult = result::Result<Vec<u8>, ClientError>;

#[derive(Debug)]
pub struct Client {
    inner: ReqwestClient,
}

impl Client {
    pub fn execute(&self, url: &str, action: &str, xml: Vec<u8>) -> ClientResult {
        //TODO: tentativas
        let mut body = Vec::new();
        let mut res = self
            .inner
            .post(url)
            .header("Content-Type", "application/soap+xml; charset=utf-8")
            .header("SOAPAction", action)
            .body(xml)
            .send()?;
        res.copy_to(&mut body)?;
        Ok(body)
    }
}

pub type ClientBuilderResult = result::Result<Client, ClientError>;

#[derive(Debug)]
pub struct ClientBuilder {
    inner: ReqwestClientBuilder,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            inner: ReqwestClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .gzip(true)
                .user_agent("Rust/NF-e")
                .timeout(Duration::from_secs(CLIENT_TIMEOUT))
                .connect_timeout(Duration::from_secs(CLIENT_CONNECT_TIMEOUT)),
        }
    }

    pub fn set_pkcs12(self, pkcs12: Pkcs12Certificate) -> Self {
        self.with_inner(|inner| inner.identity(pkcs12.into_inner()))
    }

    pub fn set_connect_timeout(self, timeout: Duration) -> Self {
        self.with_inner(|inner| inner.connect_timeout(timeout))
    }

    pub fn set_timeout(self, timeout: Duration) -> Self {
        self.with_inner(|inner| inner.timeout(timeout))
    }

    pub fn set_verbose(self, verbose: bool) -> Self {
        self.with_inner(move |inner| inner.connection_verbose(verbose))
    }

    pub fn build(self) -> ClientBuilderResult {
        Ok(Client {
            inner: self.inner.build()?,
        })
    }

    #[inline]
    fn with_inner<F>(mut self, func: F) -> Self
    where
        F: FnOnce(ReqwestClientBuilder) -> ReqwestClientBuilder,
    {
        self.inner = func(self.inner);
        self
    }
}
