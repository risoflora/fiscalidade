/* TODO: migrar para TOML */

use std::{io, path::Path, result};

use ini::Ini;
use thiserror::Error;
use tokio::fs;

use crate::tipos::{self, Ambiente, Modelo, Servico, Uf};

#[derive(Error, Debug)]
pub enum WebServicesError {
    /// Erros relacionados a I/O.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Erros relacionados a INI.
    #[error(transparent)]
    Ini(#[from] ini::Error),
    /// Erros relacionados a parsing de INI.
    #[error(transparent)]
    IniParse(#[from] ini::ParseError),
}

#[derive(Clone)]
pub struct WebServices {
    inner: Ini,
}

pub type WebServicesIniResult = result::Result<WebServices, WebServicesError>;

impl WebServices {
    #[inline]
    fn make(ini: Ini) -> WebServicesIniResult {
        Ok(Self { inner: ini })
    }

    pub fn from_bytes(bytes: &[u8]) -> WebServicesIniResult {
        Self::make(Ini::load_from_str(&String::from_utf8_lossy(bytes))?)
    }

    pub fn from_str(string: &str) -> WebServicesIniResult {
        Self::make(Ini::load_from_str(string)?)
    }

    pub async fn from_file<P: AsRef<Path>>(path: P) -> WebServicesIniResult {
        Self::make(Ini::load_from_str(
            fs::read_to_string(path).await?.as_str(),
        )?)
    }

    #[cfg(feature = "embed_webservices")]
    pub fn from_embedded() -> WebServicesIniResult {
        Self::make(Ini::load_from_str(include_str!(
            "../resources/webservices.ini"
        ))?)
    }

    pub fn get_from(&self, section: &str, key: &str) -> Option<&str> {
        self.inner.get_from(Some(section), key)
    }
}

impl Default for WebServices {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

#[derive(Error, Debug)]
pub enum WebServicesBuilderError {
    #[error("INI de webservices não informado")]
    IniNaoInformado,
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
    ini: Option<WebServices>,
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
            ini: None,
            modelo: None,
            uf: None,
            ambiente: None,
            servico: None,
            contingencia: false,
        }
    }

    pub fn set_ini(mut self, ini: WebServices) -> Self {
        self.ini = Some(ini);
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
        let ini = match self.ini {
            Some(ini) => ini,
            None => return Err(WebServicesBuilderError::IniNaoInformado),
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
        let url = ini.get_from(secao.as_str(), "Usar");
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
        let url = match ini.get_from(secao.as_str(), servico.nome().as_str()) {
            Some(url) => url,
            None => return Err(WebServicesBuilderError::WebServiceNaoEncontrado { uf, servico }),
        };
        Ok(url.to_string())
    }
}
