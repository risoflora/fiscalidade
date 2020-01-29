/// Valida chave de nota usando regra _[0-9]{44}_ extraída de arquivo _XSD_ da _SEFAZ_.
pub fn validar_chave(chave: &str) -> bool {
    if let Ok(re) = regex::Regex::new(r"^[0-9]{44}$") {
        return re.is_match(chave);
    }
    false
}
