const PORTAL_FISCAL: &str = "http://www.portalfiscal.inf.br/nfe";

#[inline]
pub fn format_action(operacao: &str) -> String {
    format!(
        "{portal}/wsdl/{operacao}",
        portal = PORTAL_FISCAL,
        operacao = operacao
    )
}

#[inline]
pub fn format_dados_msg(dados: &str, operacao: &str) -> String {
    format!(
        "<nfeDadosMsg xmlns=\"{namespace}\">{dados}</nfeDadosMsg>",
        namespace = format_action(operacao),
        dados = dados,
    )
}

#[inline]
pub fn format_envelope(body: &str) -> String {
    format!(
        concat!(
            "<soap12:Envelope xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\" xmlns:soap12=\"http://www.w3.org/2003/05/soap-envelope\">",
            "<soap12:Body>{body}</soap12:Body>",
            "</soap12:Envelope>"
        ),
        body = body
    )
}

#[inline]
pub fn format_xml(envelope: &str) -> String {
    format!(
        concat!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>", "{envelope}"),
        envelope = format_envelope(envelope)
    )
}

#[inline]
pub fn format_cons_stat_serv(cuf: u8, tp_amb: u8, versao: &str, operacao: &str) -> String {
    format_dados_msg(
        format!(
            concat!(
                "<consStatServ xmlns=\"{portal}\" versao=\"{versao}\">",
                "<tpAmb>{tp_amb}</tpAmb>",
                "<cUF>{cuf}</cUF>",
                "<xServ>STATUS</xServ>",
                "</consStatServ>"
            ),
            portal = PORTAL_FISCAL,
            versao = versao,
            tp_amb = tp_amb,
            cuf = cuf
        )
        .as_str(),
        operacao,
    )
}

#[inline]
pub fn format_cons_sit(tp_amb: u8, versao: &str, operacao: &str, ch: &str) -> String {
    format_dados_msg(
        format!(
            concat!(
                "<consSitNFe xmlns=\"{portal}\" versao=\"{versao}\">",
                "<tpAmb>{tp_amb}</tpAmb>",
                "<xServ>CONSULTAR</xServ>",
                "<chNFe>{ch}</chNFe>",
                "</consSitNFe>"
            ),
            portal = PORTAL_FISCAL,
            versao = versao,
            tp_amb = tp_amb,
            ch = ch,
        )
        .as_str(),
        operacao,
    )
}

#[inline]
pub fn format_cons_reci(tp_amb: u8, versao: &str, operacao: &str, rec: &str) -> String {
    format_dados_msg(
        format!(
            concat!(
                "<consReciNFe xmlns=\"{portal}\" versao=\"{versao}\">",
                "<tpAmb>{tp_amb}</tpAmb>",
                "<nRec>{rec}</nRec>",
                "</consReciNFe>"
            ),
            portal = PORTAL_FISCAL,
            versao = versao,
            tp_amb = tp_amb,
            rec = rec,
        )
        .as_str(),
        operacao,
    )
}

#[inline]
pub fn format_cons_cad(cuf: u8, versao: &str, operacao: &str, doc: &str, doc_tag: &str) -> String {
    format_dados_msg(
        format!(
            concat!(
                "<ConsCad xmlns=\"{portal}\" versao=\"{versao}\">",
                "<infCons>",
                "<xServ>CONS-CAD</xServ>",
                "<UF>{cuf}</UF>",
                "<{doc_tag_prefixo}>{doc}</{doc_tag_sufixo}>",
                "</infCons>",
                "</ConsCad>"
            ),
            portal = PORTAL_FISCAL,
            versao = versao,
            cuf = cuf,
            doc_tag_prefixo = doc_tag,
            doc = doc,
            doc_tag_sufixo = doc_tag,
        )
        .as_str(),
        operacao,
    )
}
