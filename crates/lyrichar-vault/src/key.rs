use std::fmt;

use argon2::{Argon2, Error as Argon2Error};
use chacha20poly1305::{KeyInit, XChaCha20Poly1305, aead::Aead};
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{
    nonce::{Nonce, NonceError},
    output::{Data, Output},
};

pub const SIZE: usize = 32;

pub type Array = [u8; SIZE];

pub const DERIVE: &str = "key derivation failed";

#[derive(Debug, Error)]
#[error("{DERIVE}: {error}")]
pub struct DeriveError {
    error: Argon2Error,
}

impl DeriveError {
    pub const fn new(error: Argon2Error) -> Self {
        Self { error }
    }

    pub const fn get(self) -> Argon2Error {
        self.error
    }
}

pub const ENCRYPT: &str = "encryption failed";
pub const DECRYPT: &str = "decryption failed";

#[derive(Debug, Error)]
#[error(transparent)]
pub enum EncryptError {
    Nonce(#[from] NonceError),
    With(#[from] EncryptWithError),
}

#[derive(Debug, Error)]
#[error("{ENCRYPT}")]
pub struct EncryptWithError;

#[derive(Debug, Error)]
#[error("{DECRYPT}")]
pub struct DecryptError;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Key {
    array: Array,
}

impl fmt::Debug for Key {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(stringify!(Key))
            .finish_non_exhaustive()
    }
}

impl Key {
    pub const fn new(array: Array) -> Self {
        Self { array }
    }

    pub fn try_derive<P: AsRef<[u8]>, S: AsRef<[u8]>>(
        password: P,
        salt: S,
    ) -> Result<Self, DeriveError> {
        let mut output = [0; SIZE];

        let argon2 = Argon2::default();

        argon2
            .hash_password_into(password.as_ref(), salt.as_ref(), output.as_mut_slice())
            .map_err(DeriveError::new)?;

        let key = Self::new(output);

        Ok(key)
    }

    pub fn derive<P: AsRef<[u8]>, S: AsRef<[u8]>>(password: P, salt: S) -> Self {
        Self::try_derive(password, salt).expect(DERIVE)
    }

    pub fn cipher(&self) -> XChaCha20Poly1305 {
        let key = &self.array;

        XChaCha20Poly1305::new(key.into())
    }

    pub fn try_encrypt<I: AsRef<[u8]>>(&self, input: I) -> Result<Output, EncryptError> {
        let nonce = Nonce::try_generate()?;

        let output = self.try_encrypt_with(nonce, input)?;

        Ok(output)
    }

    pub fn try_encrypt_with<I: AsRef<[u8]>>(
        &self,
        nonce: Nonce,
        input: I,
    ) -> Result<Output, EncryptWithError> {
        let data = self
            .cipher()
            .encrypt(nonce.as_x(), input.as_ref())
            .map_err(|_| EncryptWithError)?;

        let output = Output::new(data, nonce);

        Ok(output)
    }

    pub fn encrypt<I: AsRef<[u8]>>(&self, input: I) -> Output {
        self.try_encrypt(input).expect(ENCRYPT)
    }

    pub fn encrypt_with<I: AsRef<[u8]>>(&self, nonce: Nonce, input: I) -> Output {
        self.try_encrypt_with(nonce, input).expect(ENCRYPT)
    }

    pub fn try_decrypt_parts<D: AsRef<[u8]>>(
        &self,
        nonce: &Nonce,
        data: D,
    ) -> Result<Data, DecryptError> {
        self.cipher()
            .decrypt(nonce.as_x(), data.as_ref())
            .map_err(|_| DecryptError)
    }

    pub fn decrypt_parts<D: AsRef<[u8]>>(&self, nonce: &Nonce, data: D) -> Data {
        self.try_decrypt_parts(nonce, data).expect(DECRYPT)
    }

    pub fn try_decrypt(&self, output: &Output) -> Result<Data, DecryptError> {
        self.try_decrypt_parts(output.nonce(), output.data())
    }
}
