use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VersaoUrl {
    Ver100,
    Ver101,
    Ver200,
    Ver400,
}

impl VersaoUrl {
    #[inline]
    pub fn as_str(&self) -> &str {
        use super::VersaoUrl::*;
        match *self {
            Ver100 => "1.00",
            Ver101 => "1.01",
            Ver200 => "2.00",
            Ver400 => "4.00",
        }
    }
}

impl fmt::Display for VersaoUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Uf {
    Ro,
    Ac,
    Am,
    Rr,
    Pa,
    Ap,
    To,
    Ma,
    Pi,
    Ce,
    Rn,
    Pb,
    Pe,
    Al,
    Se,
    Ba,
    Mg,
    Es,
    Rj,
    Sp,
    Pr,
    Sc,
    Rs,
    Ms,
    Mt,
    Go,
    Df,
}

impl fmt::Display for Uf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Uf {
    #[inline]
    pub fn from_str(uf: &str) -> Option<Self> {
        use super::Uf::*;
        match uf.to_uppercase().as_str() {
            "RO" => Some(Ro),
            "AC" => Some(Ac),
            "AM" => Some(Am),
            "RR" => Some(Rr),
            "PA" => Some(Pa),
            "AP" => Some(Ap),
            "TO" => Some(To),
            "MA" => Some(Ma),
            "PI" => Some(Pi),
            "CE" => Some(Ce),
            "RN" => Some(Rn),
            "PB" => Some(Pb),
            "PE" => Some(Pe),
            "AL" => Some(Al),
            "SE" => Some(Se),
            "BA" => Some(Ba),
            "MG" => Some(Mg),
            "ES" => Some(Es),
            "RJ" => Some(Rj),
            "SP" => Some(Sp),
            "PR" => Some(Pr),
            "SC" => Some(Sc),
            "RS" => Some(Rs),
            "MS" => Some(Ms),
            "MT" => Some(Mt),
            "GO" => Some(Go),
            "DF" => Some(Df),
            _ => None,
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        use super::Uf::*;
        match *self {
            Ro => "RO",
            Ac => "AC",
            Am => "AM",
            Rr => "RR",
            Pa => "PA",
            Ap => "AP",
            To => "TO",
            Ma => "MA",
            Pi => "PI",
            Ce => "CE",
            Rn => "RN",
            Pb => "PB",
            Pe => "PE",
            Al => "AL",
            Se => "SE",
            Ba => "BA",
            Mg => "MG",
            Es => "ES",
            Rj => "RJ",
            Sp => "SP",
            Pr => "PR",
            Sc => "SC",
            Rs => "RS",
            Ms => "MS",
            Mt => "MT",
            Go => "GO",
            Df => "DF",
        }
    }

    #[inline]
    pub fn cuf(&self) -> u8 {
        use super::Uf::*;
        match *self {
            Ro => 11,
            Ac => 12,
            Am => 13,
            Rr => 14,
            Pa => 15,
            Ap => 16,
            To => 17,
            Ma => 21,
            Pi => 22,
            Ce => 23,
            Rn => 24,
            Pb => 25,
            Pe => 26,
            Al => 27,
            Se => 28,
            Ba => 29,
            Mg => 31,
            Es => 32,
            Rj => 33,
            Sp => 35,
            Pr => 41,
            Sc => 42,
            Rs => 43,
            Ms => 50,
            Mt => 51,
            Go => 52,
            Df => 53,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Ambiente {
    Producao,
    Homologacao,
}

impl Ambiente {
    #[inline]
    pub fn as_str(&self) -> &str {
        use super::Ambiente::*;
        match *self {
            Producao => "P",
            Homologacao => "H",
        }
    }

    #[inline]
    pub fn from_str(ambiente: &str) -> Option<Self> {
        use super::Ambiente::*;
        let c = ambiente.chars().next()?;
        match c {
            'P' | 'p' => Some(Producao),
            'H' | 'h' => Some(Homologacao),
            _ => None,
        }
    }

    #[inline]
    pub fn tp_amb(&self) -> u8 {
        use super::Ambiente::*;
        match *self {
            Producao => 1,
            Homologacao => 2,
        }
    }
}

impl fmt::Display for Ambiente {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Modelo {
    Nfe,
    Nfce,
}

impl Modelo {
    #[inline]
    pub fn from_str(tipo: &str) -> Option<Self> {
        use super::Modelo::*;
        match tipo.to_lowercase().as_str() {
            "nfe" => Some(Nfe),
            "nfce" => Some(Nfce),
            _ => None,
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        use super::Modelo::*;
        match *self {
            Nfe => "NFe",
            Nfce => "NFCe",
        }
    }

    #[inline]
    pub fn codigo(&self) -> u8 {
        use super::Modelo::*;
        match *self {
            Nfe => 55,
            Nfce => 65,
        }
    }
}

impl fmt::Display for Modelo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Servico {
    StatusServico,
    ConsultaProtocolo,
    ConsultaAutorizacao,
    ConsultaCadastro,
    Envio,
    Cce,
    Epec,
    Manifestacao,
    Cancelamento,
    CancelamentoSubstituicao,
    Inutilizacao,
    DistribuicaoDfe,
    UrlQrCode,
    UrlConsultaNfce,
}

impl fmt::Display for Servico {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use super::Servico::*;
        match *self {
            StatusServico => write!(f, "Status Serviço"),
            ConsultaProtocolo => write!(f, "Consulta protocolo"),
            ConsultaAutorizacao => write!(f, "Consulta Recibo"),
            ConsultaCadastro => write!(f, "Consulta Cadastro"),
            Envio => write!(f, "Envio"),
            Cce => write!(f, "CC-e"),
            Epec => write!(f, "EPEC"),
            Manifestacao => write!(f, "Manifestação"),
            Cancelamento => write!(f, "Cancelamento"),
            CancelamentoSubstituicao => write!(f, "Cancelamento Substituição"),
            Inutilizacao => write!(f, "Inutilização"),
            DistribuicaoDfe => write!(f, "Distribuição DF-e"),
            UrlQrCode => write!(f, "URL QRCode"),
            UrlConsultaNfce => write!(f, "URL Consulta NFC-e"),
        }
    }
}

impl Servico {
    #[inline]
    pub fn chave(&self) -> String {
        use super::Servico::*;
        let ver = self.versao_url();
        match *self {
            StatusServico => format!("NfeStatusServico_{}", ver),
            ConsultaProtocolo => format!("NfeConsultaProtocolo_{}", ver),
            ConsultaAutorizacao => format!("NFeRetAutorizacao_{}", ver),
            ConsultaCadastro => format!("NfeConsultaCadastro_{}", ver),
            Envio => format!("NfeAutorizacao_{}", ver),
            Cce => format!("RecepcaoEvento_{}", ver),
            Epec => format!("RecepcaoEvento_{}", ver),
            Manifestacao => format!("RecepcaoEvento_{}", ver),
            Cancelamento => format!("RecepcaoEvento_{}", ver),
            CancelamentoSubstituicao => format!("RecepcaoEvento_{}", ver),
            Inutilizacao => format!("NfeInutilizacao_{}", ver),
            DistribuicaoDfe => format!("NFeDistribuicaoDFe_{}", ver),
            UrlQrCode => "URL-QRCode".to_string(),
            UrlConsultaNfce => "URL-ConsultaNFCe".to_string(),
        }
    }

    #[inline]
    pub fn operacao(&self) -> Option<&str> {
        use super::Servico::*;
        match *self {
            StatusServico => Some("NFeStatusServico4"),
            ConsultaProtocolo => Some("NFeConsultaProtocolo4"),
            ConsultaAutorizacao => Some("NFeRetAutorizacao4"),
            ConsultaCadastro => Some("CadConsultaCadastro4"),
            Envio => Some("NFeAutorizacao4"),
            Cce => Some("NFeRecepcaoEvento4"),
            Epec => Some("NFeRecepcaoEvento4"),
            Manifestacao => Some("RecepcaoEvento"),
            Cancelamento => Some("RecepcaoEvento"),
            CancelamentoSubstituicao => Some("RecepcaoEvento"),
            Inutilizacao => Some("NFeInutilizacao4"),
            DistribuicaoDfe => Some("NFeDistribuicaoDFe"),
            UrlQrCode => Some("NfeConsultaQR"),
            UrlConsultaNfce => None,
        }
    }

    #[inline]
    pub fn versao_url(&self) -> VersaoUrl {
        use super::Servico::*;
        use super::VersaoUrl::*;
        match *self {
            StatusServico => Ver400,
            ConsultaProtocolo => Ver400,
            ConsultaAutorizacao => Ver400,
            ConsultaCadastro => Ver400,
            Envio => Ver400,
            Cce => Ver400,
            Epec => Ver400,
            Manifestacao => Ver400,
            Cancelamento => Ver400,
            CancelamentoSubstituicao => Ver400,
            Inutilizacao => Ver400,
            DistribuicaoDfe => Ver101,
            UrlQrCode => Ver400,
            UrlConsultaNfce => Ver400,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TipoDocumento {
    Cpf,
    Cnpj,
    Ie,
}

impl TipoDocumento {
    #[inline]
    pub fn as_str(&self) -> &str {
        use super::TipoDocumento::*;
        match *self {
            Cpf => "CPF",
            Cnpj => "CNPJ",
            Ie => "IE",
        }
    }
}

impl fmt::Display for TipoDocumento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Documento<'a> {
    conteudo: &'a str,
    tipo: TipoDocumento,
}

impl<'a> Documento<'a> {
    #[inline]
    pub fn from_cpf(cpf: &str) -> Documento {
        Documento {
            conteudo: cpf,
            tipo: TipoDocumento::Cpf,
        }
    }

    #[inline]
    pub fn from_cnpj(cnpj: &str) -> Documento {
        Documento {
            conteudo: cnpj,
            tipo: TipoDocumento::Cnpj,
        }
    }

    #[inline]
    pub fn from_ie(ie: &str) -> Documento {
        Documento {
            conteudo: ie,
            tipo: TipoDocumento::Ie,
        }
    }

    #[inline]
    pub fn tipo(&self) -> TipoDocumento {
        self.tipo
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.conteudo
    }
}

impl fmt::Display for Documento<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
