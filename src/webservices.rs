use std::{io, path::Path, result, str};

use thiserror::Error;
use tokio::fs;
use toml::{de::Error as TomlError, Value as TomlValue};

use crate::tipos::{self, Ambiente, Modelo, Servico, Uf};

#[derive(Error, Debug)]
pub enum WebServicesError {
    /// Erros relacionados a I/O.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Erros relacionados a TOML.
    #[error(transparent)]
    Toml(#[from] TomlError),
}

#[derive(Clone)]
pub struct WebServices {
    inner: TomlValue,
}

pub type WebServicesResult = result::Result<WebServices, WebServicesError>;

impl WebServices {
    #[inline]
    fn make(value: TomlValue) -> WebServicesResult {
        Ok(Self { inner: value })
    }

    pub fn from_slice(bytes: &[u8]) -> WebServicesResult {
        Self::make(toml::from_slice(str::from_utf8(bytes))?)
    }

    pub fn from_str(s: &str) -> WebServicesResult {
        Self::make(toml::from_str(s)?)
    }

    pub async fn from_file<P: AsRef<Path>>(path: P) -> WebServicesResult {
        Self::make(toml::from_str(&fs::read_to_string(path).await?)?)
    }

    #[cfg(feature = "embed_webservices")]
    pub fn from_embedded() -> WebServicesResult {
        Self::make(toml::from_str(include_str!(
            "../resources/webservices.toml"
        ))?)
    }

    pub fn get_from(&self, section: &str, key: &str) -> Option<&str> {
        let table = self.inner.as_table()?;
        let pairs = table.get(section)?;
        let value = pairs.get(key)?;
        value.as_str()
    }
}

#[derive(Error, Debug)]
pub enum WebServicesBuilderError {
    #[error("TOML de webservices não informado")]
    TomlNaoInformado,
    #[error("UF não informada")]
    UfNaoInformada,
    #[error("Ambiente não informado")]
    AmbienteNaoInformado,
    #[error("Serviço não informado")]
    ServicoNaoInformado,
    #[error("Modelo de NF não informado")]
    ModeloNaoInformado,
    #[error("UF não possui webservice para consulta de cadastro")]
    UfSemWebServiceConsultaCadastro,
    #[error("WebService não encontrado para {uf}: {servico}")]
    WebServiceNaoEncontrado {
        uf: tipos::Uf,
        servico: tipos::Servico,
    },
}

#[derive(Clone)]
pub struct WebServicesBuilder {
    toml: Option<WebServices>,
    modelo: Option<Modelo>,
    uf: Option<Uf>,
    ambiente: Option<Ambiente>,
    servico: Option<Servico>,
    contingencia: bool,
}

pub type WebServicesBuilderResult = result::Result<String, WebServicesBuilderError>;

impl WebServicesBuilder {
    pub fn new() -> Self {
        Self {
            toml: None,
            modelo: None,
            uf: None,
            ambiente: None,
            servico: None,
            contingencia: false,
        }
    }

    pub fn set_toml(mut self, toml: WebServices) -> Self {
        self.toml = Some(toml);
        self
    }

    pub fn set_modelo(mut self, modelo: Modelo) -> Self {
        self.modelo = Some(modelo);
        self
    }

    pub fn set_uf(mut self, uf: Uf) -> Self {
        self.uf = Some(uf);
        self
    }

    pub fn set_ambiente(mut self, ambiente: Ambiente) -> Self {
        self.ambiente = Some(ambiente);
        self
    }

    pub fn set_servico(mut self, servico: Servico) -> Self {
        self.servico = Some(servico);
        self
    }

    pub fn set_contingencia(mut self, contingencia: bool) -> Self {
        self.contingencia = contingencia;
        self
    }

    pub fn build(self) -> WebServicesBuilderResult {
        let toml = self.toml.ok_or(WebServicesBuilderError::TomlNaoInformado)?;
        let modelo = self
            .modelo
            .ok_or(WebServicesBuilderError::ModeloNaoInformado)?;
        let uf = self.uf.ok_or(WebServicesBuilderError::UfNaoInformada)?;
        let ambiente = self
            .ambiente
            .ok_or(WebServicesBuilderError::AmbienteNaoInformado)?;
        let servico = self
            .servico
            .ok_or(WebServicesBuilderError::ServicoNaoInformado)?;
        let mut secao = format!("{}_{}_{}", modelo, uf, ambiente);
        let url = toml.get_from(secao.as_str(), "Usar");

        // Não há "clean code" neste trecho de código porque precisamos manter compatibilidade com:
        // https://github.com/Samuel-Oliveira/Java_NFe/blob/df575658da4b7d3911a8c15ffb2841184ad8cfeb/src/main/java/br/com/swconsultoria/nfe/util/WebServiceUtil.java#L71

        // URL consulta cadastro
        if servico == Servico::ConsultaCadastro
            && (uf == Uf::Pa
                || uf == Uf::Am
                || uf == Uf::Al
                || uf == Uf::Ap
                || uf == Uf::Df
                || uf == Uf::Pi
                || uf == Uf::Rj
                || uf == Uf::Ro
                || uf == Uf::Se
                || uf == Uf::To)
        {
            return Err(WebServicesBuilderError::UfSemWebServiceConsultaCadastro);
        }
        // URL de ambiente nacional
        if servico == Servico::DistribuicaoDfe
            || servico == Servico::Manifestacao
            || servico == Servico::Epec
        {
            secao = if ambiente == Ambiente::Homologacao {
                "NFe_AN_H".to_string()
            } else {
                "NFe_AN_P".to_string()
            };
        } else if servico != Servico::UrlConsultaNfce
            && servico != Servico::UrlQrCode
            && url.is_some()
        {
            secao = url.unwrap().to_string()
        } else if self.contingencia {
            // SVC-RS
            if uf == Uf::Go
                || uf == Uf::Am
                || uf == Uf::Ba
                || uf == Uf::Ce
                || uf == Uf::Ma
                || uf == Uf::Ms
                || uf == Uf::Mt
                || uf == Uf::Pa
                || uf == Uf::Pe
                || uf == Uf::Pi
                || uf == Uf::Pr
            {
                secao = format!("{}_SVRS_{}", modelo, ambiente);
            // SVC-AN
            } else {
                secao = format!("{}_SVC-AN_{}", modelo, ambiente);
            }
        }

        let url = toml
            .get_from(secao.as_str(), servico.chave().as_str())
            .ok_or(WebServicesBuilderError::WebServiceNaoEncontrado { uf, servico })?;
        Ok(url.to_string())
    }
}
