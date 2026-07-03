/// Post-Quantum Cryptography Networking Primitives

#[derive(Debug, Clone, Copy)]
pub enum EncryptionParam {
    Kyber512,
    Kyber768,
    Kyber1024,
}

#[derive(Debug, Clone, Copy)]
pub struct SecurityPolicy {
    pub param: EncryptionParam,
    pub enforce_auth: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            param: EncryptionParam::Kyber768,
            enforce_auth: true,
        }
    }
}
