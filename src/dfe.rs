use std::{fmt, result};

use thiserror::Error;

use crate::{
    client::{ClientBuilder, ClientError},
    soap12,
    tipos::{Ambiente, Documento, Modelo, Servico, Uf},
    util,
    webservices::{WebServices, WebServicesBuilder, WebServicesBuilderError},
    Pkcs12Certificate,
};

#[derive(Error, Debug)]
pub enum DfeError {
    #[error(transparent)]
    WebServices(#[from] WebServicesBuilderError),
    #[error(transparent)]
    Client(#[from] ClientError),
    #[error("Operação inexistente para este serviço")]
    OperacaoInexistente,
    #[error("Chave de NF-e inválida: {0}")]
    ChaveInvalida(String),
    #[error("Código de recibo inválido: {0}")]
    ReciboInvalido(String),
}

pub struct Dfe {
    webservices_builder: WebServicesBuilder,
    client_builder: ClientBuilder,
}

#[derive(Debug)]
pub struct Xml(pub Vec<u8>);

pub type DfeResult = result::Result<Xml, DfeError>;

impl fmt::Display for Xml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl Dfe {
    pub fn new() -> Self {
        Self {
            webservices_builder: WebServicesBuilder::new(),
            client_builder: ClientBuilder::new(),
        }
    }

    pub fn set_webservices(self, webservices: WebServices) -> Self {
        self.with_ws_builder(|ws_builder| ws_builder.set_toml(webservices))
    }

    pub fn set_pkcs12(self, pkcs12: Pkcs12Certificate) -> Self {
        self.with_cli_builder(|cli_builder| cli_builder.set_pkcs12(pkcs12))
    }

    pub async fn status_servico(self, modelo: Modelo, uf: Uf, ambiente: Ambiente) -> DfeResult {
        self.send(
            modelo,
            uf,
            ambiente,
            Servico::StatusServico,
            |cuf, tp_amb, versao, operacao| {
                let xml = soap12::format_cons_stat_serv(cuf, tp_amb, versao, operacao);
                xml
            },
        )
        .await
    }

    pub async fn consultar_protocolo(
        self,
        modelo: Modelo,
        uf: Uf,
        ambiente: Ambiente,
        chave: &str,
    ) -> DfeResult {
        if !util::validar_chave(chave) {
            return Err(DfeError::ChaveInvalida(chave.to_string()));
        }
        self.send(
            modelo,
            uf,
            ambiente,
            Servico::ConsultaProtocolo,
            |_, tp_amb, versao, operacao| {
                let xml = soap12::format_cons_sit(tp_amb, versao, operacao, chave);
                xml
            },
        )
        .await
    }

    pub async fn consultar_autorizacao(
        self,
        modelo: Modelo,
        uf: Uf,
        ambiente: Ambiente,
        recibo: &str,
    ) -> DfeResult {
        if !util::validar_recibo(recibo) {
            return Err(DfeError::ReciboInvalido(recibo.to_string()));
        }
        self.send(
            modelo,
            uf,
            ambiente,
            Servico::ConsultaAutorizacao,
            |_, tp_amb, versao, operacao| {
                let xml = soap12::format_cons_reci(tp_amb, versao, operacao, recibo);
                xml
            },
        )
        .await
    }

    pub async fn consultar_cadastro(
        self,
        modelo: Modelo,
        uf: Uf,
        ambiente: Ambiente,
        documento: Documento<'_>,
    ) -> DfeResult {
        //TODO: validar doc
        self.send(
            modelo,
            uf,
            ambiente,
            Servico::ConsultaCadastro,
            |cuf, _, versao, operacao| {
                let xml = soap12::format_cons_cad(
                    cuf,
                    versao,
                    operacao,
                    documento.as_str(),
                    documento.tipo().as_str(),
                );
                xml
            },
        )
        .await
    }

    #[inline]
    async fn send<F>(
        self,
        modelo: Modelo,
        uf: Uf,
        ambiente: Ambiente,
        servico: Servico,
        envelope_fn: F,
    ) -> DfeResult
    where
        F: FnOnce(u8, u8, &str, &str) -> String,
    {
        let operacao = servico.operacao().ok_or(DfeError::OperacaoInexistente)?;
        let dfe = self.with_ws_builder(|ws_builder| {
            ws_builder
                .set_modelo(modelo)
                .set_uf(uf)
                .set_ambiente(ambiente)
                .set_servico(servico)
        });
        let ws_builder = dfe.webservices_builder;
        let cli_builder = dfe.client_builder;
        let ws = ws_builder.build()?;
        let cli = cli_builder.build()?;
        let xml = soap12::format_xml(
            envelope_fn(
                uf.cuf(),
                ambiente.tp_amb(),
                servico.versao_url().as_str(),
                operacao,
            )
            .as_str(),
        );
        let retorno = cli
            .execute(
                ws.as_str(),
                soap12::format_action(operacao).as_str(),
                xml.as_bytes().to_vec(),
            )
            .await?;
        Ok(Xml(retorno))
    }

    #[inline]
    fn with_ws_builder<F>(mut self, func: F) -> Self
    where
        F: FnOnce(WebServicesBuilder) -> WebServicesBuilder,
    {
        self.webservices_builder = func(self.webservices_builder);
        self
    }

    #[inline]
    fn with_cli_builder<F>(mut self, func: F) -> Self
    where
        F: FnOnce(ClientBuilder) -> ClientBuilder,
    {
        self.client_builder = func(self.client_builder);
        self
    }
}
