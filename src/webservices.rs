use std::{io, path::Path, result};

use thiserror::Error;
use tokio::fs;
use toml;

use crate::tipos::{self, Ambiente, Modelo, Servico, Uf};

#[derive(Error, Debug)]
pub enum WebServicesError {
    /// Erros relacionados a I/O.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Erros relacionados a TOML.
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

#[derive(Clone)]
pub struct WebServices {
    inner: toml::Value,
}

pub type WebServicesTomlResult = result::Result<WebServices, WebServicesError>;

impl WebServices {
    #[inline]
    fn make(value: toml::Value) -> WebServicesTomlResult {
        Ok(Self { inner: value })
    }

    pub fn from_slice(bytes: &[u8]) -> WebServicesTomlResult {
        Self::make(toml::from_slice(bytes)?)
    }

    pub fn from_str(s: &str) -> WebServicesTomlResult {
        Self::make(toml::from_str(s)?)
    }

    pub async fn from_file<P: AsRef<Path>>(path: P) -> WebServicesTomlResult {
        Self::make(toml::from_slice(&fs::read(path).await?)?)
    }

    #[cfg(feature = "embed_webservices")]
    pub fn from_embedded() -> WebServicesTomlResult {
        Self::make(toml::from_slice(include_bytes!(
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
    #[error("WebService não encontrado para {}: {}", uf, servico)]
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
        let toml = match self.toml {
            Some(toml) => toml,
            None => return Err(WebServicesBuilderError::TomlNaoInformado),
        };
        let modelo = match self.modelo {
            Some(modelo) => modelo,
            None => return Err(WebServicesBuilderError::ModeloNaoInformado),
        };
        let uf = match self.uf {
            Some(uf) => uf,
            None => return Err(WebServicesBuilderError::UfNaoInformada),
        };
        let ambiente = match self.ambiente {
            Some(ambiente) => ambiente,
            None => return Err(WebServicesBuilderError::AmbienteNaoInformado),
        };
        let servico = match self.servico {
            Some(servico) => servico,
            None => return Err(WebServicesBuilderError::ServicoNaoInformado),
        };
        let mut secao = format!("{}_{}_{}", modelo.as_str(), uf.as_str(), ambiente.as_str());
        let url = toml.get_from(secao.as_str(), "Usar");
        // URLs consulta cadastro
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
        // URLS de ambiente nacional
        } else if servico == Servico::DistribuicaoDfe
            || servico == Servico::Manifestacao
            || servico == Servico::Epec
        {
            secao = if ambiente == Ambiente::Homologacao {
                "NFe_AN_H".into()
            } else {
                "NFe_AN_P".into()
            };
        } else if servico != Servico::UrlConsultaNfce
            && servico != Servico::UrlQrCode
            && url.is_some()
        {
            secao = url.unwrap().into()
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
                secao = format!("{}_SVRS_{}", modelo.as_str(), ambiente.as_str());
            // SVC-AN
            } else {
                secao = format!("{}_SVC-AN_{}", modelo.as_str(), ambiente.as_str());
            }
        }
        let url = match toml.get_from(secao.as_str(), servico.nome().as_str()) {
            Some(url) => url,
            None => return Err(WebServicesBuilderError::WebServiceNaoEncontrado { uf, servico }),
        };
        Ok(url.to_string())
    }
}
