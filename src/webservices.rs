use std::{io, path::Path, result};

use ini::Ini;
use quick_error::quick_error;

use crate::tipos::{Ambiente, Modelo, Servico, Uf};

quick_error! {
    #[derive(Debug)]
    pub enum WebServicesIniError {
        Io(err: io::Error) {
            from()
            display("Erro de I/O ao carregar INI de webservices: {}", err)
        }
        Ini(err: ini::ini::Error) {
            from()
            display("Erro ao carregar INI de webservices: {}", err)
        }
        IniParse(err: ini::ini::ParseError) {
            from()
            display("Erro a fazer parse no INI de webservices: {}", err)
        }
    }
}

pub struct WebServicesIni {
    inner: Ini,
}

pub type WebServicesIniResult = result::Result<WebServicesIni, WebServicesIniError>;

impl WebServicesIni {
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

    pub fn from_file<P: AsRef<Path>>(path: P) -> WebServicesIniResult {
        Self::make(Ini::load_from_file(path)?)
    }

    pub fn get_from(&self, section: &str, key: &str) -> Option<&str> {
        self.inner.get_from(Some(section), key)
    }
}

impl Default for WebServicesIni {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

pub type WebService = String;

quick_error! {
    #[derive(Debug)]
    pub enum WebServicesBuilderError {
        IniNaoInformado {
            from()
            display("INI de webservices não informado")
        }
        UfNaoInformada {
            from()
            display("UF não informada")
        }
        AmbienteNaoInformado {
            from()
            display("Ambiente não informado")
        }
        ServicoNaoInformado {
            from()
            display("Serviço não informado")
        }
        ModeloNaoInformado {
            from()
            display("Modelo de NF não informado")
        }
        UfSemWebServiceConsultaCadastro {
            from()
            display("UF não possui webservice para consulta de cadastro")
        }
        WebServiceNaoEncontrado(uf: Uf, servico: Servico) {
            from(tipos::Uf)
            from(tipos::Servico)
            display("WebService não encontrado para {}: {}", uf, servico)
        }
    }
}

pub struct WebServicesBuilder {
    ini: Option<WebServicesIni>,
    uf: Option<Uf>,
    ambiente: Option<Ambiente>,
    servico: Option<Servico>,
    modelo: Option<Modelo>,
    contingencia: bool,
}

pub type WebServicesBuilderResult = result::Result<WebService, WebServicesBuilderError>;

impl WebServicesBuilder {
    pub fn new() -> Self {
        Self {
            ini: None,
            uf: None,
            ambiente: None,
            servico: None,
            modelo: None,
            contingencia: false,
        }
    }

    pub fn set_ini(mut self, ini: WebServicesIni) -> Self {
        self.ini = Some(ini);
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

    pub fn set_modelo(mut self, modelo: Modelo) -> Self {
        self.modelo = Some(modelo);
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
        let modelo = match self.modelo {
            Some(modelo) => modelo,
            None => return Err(WebServicesBuilderError::ModeloNaoInformado),
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
        } else if servico == Servico::DistribuicaoDfe || servico == Servico::Manifestacao {
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
            None => {
                return Err(WebServicesBuilderError::WebServiceNaoEncontrado(
                    uf, servico,
                ))
            }
        };
        Ok(url.to_string())
    }
}
