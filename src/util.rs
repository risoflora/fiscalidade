use regex::Regex;

use crate::tipos::{Ambiente, Uf};

/// Valida chave de nota usando regra `^[0-9]{44}$` extraída de arquivo XSD da SEFAZ.
pub fn validar_chave(chave: &str) -> bool {
    Regex::new("^[0-9]{44}$")
        .map(|re| re.is_match(chave))
        .unwrap_or_default()
}

/// Valida se UF é está presente na lista de UFs válidas.
#[inline]
pub fn validar_uf(uf: &str) -> bool {
    Uf::from_str(uf).is_some()
}

/// Valida se Ambiente é está presente na lista de Ambientes válidos.
#[inline]
pub fn validar_ambiente(ambiente: &str) -> bool {
    Ambiente::from_str(ambiente).is_some()
}
