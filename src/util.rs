/// Valida chave de nota usando regra `^[0-9]{44}$` extraída de arquivo XSD da SEFAZ.
pub fn validar_chave(chave: &str) -> bool {
    if let Ok(re) = regex::Regex::new(r"^[0-9]{44}$") {
        return re.is_match(chave);
    }
    false
}
