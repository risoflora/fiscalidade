use std::{fs, io, path::Path, result};

use quick_error::quick_error;
use reqwest::Identity;

#[derive(Debug)]
pub struct Pkcs12Certificate(Identity);

quick_error! {
    #[derive(Debug)]
    pub enum Pkcs12CertificateError {
        Io(err: io::Error) {
            from()
            display("Erro de I/O ao carregar certificado: {}", err)
        }
        Pkcs12(err: reqwest::Error){
            from()
            display("Erro ao carregar certificado: {}", err)
        }
    }
}

pub type Pkcs12CertificateResult = result::Result<Pkcs12Certificate, Pkcs12CertificateError>;

impl Pkcs12Certificate {
    #[inline]
    fn from(bytes: &[u8], password: &str) -> Pkcs12CertificateResult {
        Ok(Self {
            0: Identity::from_pkcs12_der(&bytes, password)?,
        })
    }

    pub fn from_bytes(bytes: &[u8], password: &str) -> Pkcs12CertificateResult {
        Self::from(bytes, password)
    }

    pub fn from_str(string: &str, password: &str) -> Pkcs12CertificateResult {
        Self::from(string.as_bytes(), password)
    }

    pub fn from_file<P: AsRef<Path>>(path: P, password: &str) -> Pkcs12CertificateResult {
        let bytes = fs::read(path)?;
        Self::from(&bytes, password)
    }

    pub(crate) fn into_inner(self) -> Identity {
        self.0
    }
}
