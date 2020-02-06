use std::result;

use quick_error::quick_error;

use crate::{
    client::{ClientBuilder, ClientError},
    soap,
    tipos::{Ambiente, Documento, Modelo, Servico, Tipo, Uf},
    util,
    webservices::{WebServices, WebServicesBuilder, WebServicesBuilderError},
    Pkcs12Certificate,
};

quick_error! {
    #[derive(Debug)]
    pub enum DfeError {
        WebServices(err: WebServicesBuilderError) {
            from()
            display("Erro ao construir webservice: {}", err)
        }
        Client(err: ClientError) {
            from()
            display("Erro ao construir client: {}", err)
        }
        OperacaoInexistente {
            from()
            display("Operação inexistente para este serviço")
        }
        ChaveInvalida(chave: String) {
            from()
            display("Chave de NF-e inválida: {}", chave)
        }
    }
}

pub struct Dfe {
    webservices_builder: WebServicesBuilder,
    client_builder: ClientBuilder,
    tipo: Tipo,
}

pub type DfeResult = result::Result<Vec<u8>, DfeError>;

impl Dfe {
    pub fn new() -> Self {
        Dfe {
            webservices_builder: WebServicesBuilder::new(),
            client_builder: ClientBuilder::new(),
            tipo: Tipo::Nfe,
        }
    }

    pub fn set_webservices(self, webservices: WebServices) -> Self {
        self.with_ws_builder(|ws_builder| ws_builder.set_ini(webservices))
    }

    pub fn set_pkcs12(self, pkcs12: Pkcs12Certificate) -> Self {
        self.with_cli_builder(|cli_builder| cli_builder.set_pkcs12(pkcs12))
    }

    pub fn status_servico(self, uf: Uf, ambiente: Ambiente) -> DfeResult {
        self.send(
            uf,
            ambiente,
            Servico::StatusServico,
            |cuf, tp_amb, tipo, _, versao, operacao| {
                let xml = soap::format_cons_stat_serv(cuf, tp_amb, tipo, versao, operacao);
                //TODO: is_valid(versao, xml, "consStatServ");
                xml
            },
        )
    }

    pub fn consultar_cadastro(self, uf: Uf, ambiente: Ambiente, documento: Documento) -> DfeResult {
        //TODO: validar doc
        self.send(
            uf,
            ambiente,
            Servico::ConsultaCadastro,
            |cuf, _, tipo, _, versao, operacao| {
                let xml = soap::format_cons_cad(
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
    }

    pub fn consultar_xml(self, uf: Uf, ambiente: Ambiente, chave: &str) -> DfeResult {
        if !util::validar_chave(chave) {
            return Err(DfeError::ChaveInvalida(chave.to_string()));
        }
        self.send(
            uf,
            ambiente,
            Servico::ConsultaXml,
            |_, tp_amb, tipo, tipo_nome, versao, operacao| {
                let xml = soap::format_cons_sit(tp_amb, tipo, tipo_nome, versao, operacao, chave);
                //TODO: is_valid(versao, xml, "consSit{nome}");
                xml
            },
        )
    }

    #[inline]
    fn send<F>(self, uf: Uf, ambiente: Ambiente, servico: Servico, format_xml: F) -> DfeResult
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
        let envelope = soap::format_envelope(
            format_xml(
                uf.cuf(),
                ambiente.tp_amb(),
                dfe.tipo.as_str(),
                dfe.tipo.nome(),
                servico.versao_url().as_str(),
                operacao,
            )
            .as_str(),
        );
        let retorno = cli.execute(
            ws.as_str(),
            soap::format_action(dfe.tipo.as_str(), operacao).as_str(),
            envelope.as_bytes().to_vec(),
        )?;
        Ok(retorno)
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
