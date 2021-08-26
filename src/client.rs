use std::{result, time::Duration};

use reqwest::{Client as HttpClient, ClientBuilder as HttpClientBuilder};
use thiserror::Error;

use crate::Pkcs12Certificate;

/// Tempo padrão de timeout para conexão de client HTTP.
pub const CLIENT_CONNECT_TIMEOUT: u64 = 5;

/// Tempo padrão de timeout para transmissão de dados de client HTTP.
pub const CLIENT_TIMEOUT: u64 = 30;

/// Tipo para tratar erros relacionados a I/O e ao client HTTP.
#[derive(Error, Debug)]
pub enum ClientError {
    /// Erros relacionados a HTTP.
    #[error(transparent)]
    HttpClient(#[from] reqwest::Error),
}

/// Tipo para tratar retorno do client HTTP.
pub type ClientResult = result::Result<Vec<u8>, ClientError>;

/// Client HTTP com suporte a TLS e compressão de dados.
#[derive(Clone, Debug)]
pub struct Client {
    inner: HttpClient,
}

impl Client {
    /// Executa requisição ao servidor informando URL e informações de SOAP como action e XML.
    pub async fn execute(&self, url: &str, action: &str, xml: Vec<u8>) -> ClientResult {
        //TODO: tentativas de reconexão
        let res = self
            .inner
            .post(url)
            .header("Content-Type", "application/soap+xml; charset=utf-8")
            .header("SOAPAction", action)
            .body(xml)
            .send()
            .await?;
        Ok(res.bytes().await?.to_vec())
    }
}

/// Construtor de clients HTTP usando [build pattern](https://en.wikipedia.org/wiki/Builder_pattern).
#[derive(Debug)]
pub struct ClientBuilder {
    inner: HttpClientBuilder,
}

/// Tipo para tratar retorno do builder de client HTTP.
pub type ClientBuilderResult = result::Result<Client, ClientError>;

impl ClientBuilder {
    /// Cria uma nova instância do builder de client HTTP.
    pub fn new() -> Self {
        Self {
            inner: HttpClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .gzip(true)
                .user_agent("Rust-Fiscalidade")
                .timeout(Duration::from_secs(CLIENT_TIMEOUT))
                .connect_timeout(Duration::from_secs(CLIENT_CONNECT_TIMEOUT)),
        }
    }

    /// Aplica certificado PKCS12 ao client HTTP criado.
    pub fn set_pkcs12(self, pkcs12: Pkcs12Certificate) -> Self {
        self.with_inner(|inner| inner.identity(pkcs12.into_inner()))
    }

    /// Aplica tempo máximo de timeout para conexão do client HTTP criado.
    pub fn set_connect_timeout(self, timeout: Duration) -> Self {
        self.with_inner(|inner| inner.connect_timeout(timeout))
    }

    /// Aplica tempo máximo de timeout para transmissão de dados do client HTTP criado.
    pub fn set_timeout(self, timeout: Duration) -> Self {
        self.with_inner(|inner| inner.timeout(timeout))
    }

    /// Torna o client HTTP criado mais verboso, i.e. emite mais informações de log.
    pub fn set_verbose(self, verbose: bool) -> Self {
        self.with_inner(move |inner| inner.connection_verbose(verbose))
    }

    /// Constrói novo client HTTP pré-configurado.
    pub fn build(self) -> ClientBuilderResult {
        Ok(Client {
            inner: self.inner.build()?,
        })
    }

    #[inline]
    fn with_inner<F>(mut self, func: F) -> Self
    where
        F: FnOnce(HttpClientBuilder) -> HttpClientBuilder,
    {
        self.inner = func(self.inner);
        self
    }
}
