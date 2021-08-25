#[inline]
pub fn format_portal(tipo: &str) -> String {
    format!("http://www.portalfiscal.inf.br/{tipo}", tipo = tipo)
}

#[inline]
pub fn format_action(tipo: &str, operacao: &str) -> String {
    format!(
        "{tipo}/wsdl/{operacao}",
        tipo = format_portal(tipo),
        operacao = operacao
    )
}

#[inline]
pub fn format_dados_msg(tipo: &str, dados: &str, operacao: &str) -> String {
    format!(
        "<{tipo_prefixo}DadosMsg xmlns=\"{namespace}\">{dados}</{tipo_sufixo}DadosMsg>",
        tipo_prefixo = tipo,
        namespace = format_action(tipo, operacao),
        dados = dados,
        tipo_sufixo = tipo
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
pub fn format_cons_stat_serv(
    cuf: u8,
    tp_amb: u8,
    tipo: &str,
    versao: &str,
    operacao: &str,
) -> String {
    format_dados_msg(
        tipo,
        format!(
            concat!(
                "<consStatServ xmlns=\"{portal}\" versao=\"{versao}\">",
                "<tpAmb>{tp_amb}</tpAmb>",
                "<cUF>{cuf}</cUF>",
                "<xServ>STATUS</xServ>",
                "</consStatServ>"
            ),
            portal = format_portal(tipo),
            versao = versao,
            tp_amb = tp_amb,
            cuf = cuf
        )
        .as_str(),
        operacao,
    )
}

#[inline]
pub fn format_cons_cad(
    cuf: u8,
    tipo: &str,
    versao: &str,
    operacao: &str,
    doc: &str,
    doc_tag: &str,
) -> String {
    format_dados_msg(
        tipo,
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
            portal = format_portal(tipo),
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

#[inline]
pub fn format_cons_sit(
    tp_amb: u8,
    tipo: &str,
    tipo_nome: &str,
    versao: &str,
    operacao: &str,
    ch: &str,
) -> String {
    format_dados_msg(
        tipo,
        format!(
            concat!(
                "<consSit{prefixo} xmlns=\"{portal}\" versao=\"{versao}\">",
                "<tpAmb>{tp_amb}</tpAmb>",
                "<xServ>CONSULTAR</xServ>",
                "<ch{ch_prefixo}>{ch}</ch{ch_sufixo}>",
                "</consSit{sufixo}>"
            ),
            prefixo = tipo_nome,
            portal = format_portal(tipo),
            versao = versao,
            tp_amb = tp_amb,
            ch_prefixo = tipo_nome,
            ch = ch,
            ch_sufixo = tipo_nome,
            sufixo = tipo_nome,
        )
        .as_str(),
        operacao,
    )
}
