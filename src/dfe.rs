use std::{fmt, result};

use thiserror::Error;

use crate::{
    client::{ClientBuilder, ClientError},
    soap12,
    tipos::{Ambiente, Documento, Modelo, Servico, Tipo, Uf},
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
}

pub struct Dfe {
    webservices_builder: WebServicesBuilder,
    client_builder: ClientBuilder,
    tipo: Tipo,
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
    pub fn new(tipo: Tipo) -> Self {
        Self {
            webservices_builder: WebServicesBuilder::new(),
            client_builder: ClientBuilder::new(),
            tipo,
        }
    }

    pub fn set_webservices(self, webservices: WebServices) -> Self {
        self.with_ws_builder(|ws_builder| ws_builder.set_ini(webservices))
    }

    pub fn set_pkcs12(self, pkcs12: Pkcs12Certificate) -> Self {
        self.with_cli_builder(|cli_builder| cli_builder.set_pkcs12(pkcs12))
    }

    pub async fn status_servico(self, uf: Uf, ambiente: Ambiente) -> DfeResult {
        self.send(
            uf,
            ambiente,
            Servico::StatusServico,
            |cuf, tp_amb, tipo, _, versao, operacao| {
                let xml = soap12::format_cons_stat_serv(cuf, tp_amb, tipo, versao, operacao);
                //TODO: is_valid(versao, xml, "consStatServ");
                xml
            },
        )
        .await
    }

    pub async fn consultar_cadastro(
        self,
        uf: Uf,
        ambiente: Ambiente,
        documento: Documento,
    ) -> DfeResult {
        //TODO: validar doc
        self.send(
            uf,
            ambiente,
            Servico::ConsultaCadastro,
            |cuf, _, tipo, _, versao, operacao| {
                let xml = soap12::format_cons_cad(
                    cuf,
                    tipo,
                    versao,
                    operacao,
                    documento.conteudo().as_str(),
                    documento.tipo().as_str(),
                );
                //TODO: is_valid(versao, xml, "consCad");
                xml
            },
        )
        .await
    }

    pub async fn consultar_xml(self, uf: Uf, ambiente: Ambiente, chave: &str) -> DfeResult {
        if !util::validar_chave(chave) {
            return Err(DfeError::ChaveInvalida(chave.to_string()));
        }
        self.send(
            uf,
            ambiente,
            Servico::ConsultaXml,
            |_, tp_amb, tipo, tipo_nome, versao, operacao| {
                let xml = soap12::format_cons_sit(tp_amb, tipo, tipo_nome, versao, operacao, chave);
                //TODO: is_valid(versao, xml, "consSit{nome}");
                xml
            },
        )
        .await
    }

    #[inline]
    async fn send<F>(
        self,
        uf: Uf,
        ambiente: Ambiente,
        servico: Servico,
        envelope_fn: F,
    ) -> DfeResult
    where
        F: FnOnce(u8, u8, &str, &str, &str, &str) -> String,
    {
        let operacao = match servico.operacao() {
            Some(operacao) => operacao,
            None => return Err(DfeError::OperacaoInexistente),
        };
        let dfe = self.with_ws_builder(|ws_builder| {
            ws_builder
                .set_uf(uf)
                .set_ambiente(ambiente)
                .set_servico(servico)
                .set_modelo(Modelo::Nfe)
        });
        let ws_builder = dfe.webservices_builder;
        let cli_builder = dfe.client_builder;
        let ws = ws_builder.build()?;
        let cli = cli_builder.build()?;
        let xml = soap12::format_xml(
            envelope_fn(
                uf.cuf(),
                ambiente.tp_amb(),
                dfe.tipo.as_str(),
                dfe.tipo.nome(),
                servico.versao_url().as_str(),
                operacao,
            )
            .as_str(),
        );
        let retorno = cli
            .execute(
                ws.as_str(),
                soap12::format_action(dfe.tipo.as_str(), operacao).as_str(),
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
