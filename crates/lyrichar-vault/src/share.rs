use std::fmt;

use rand::{
    TryRng,
    rngs::{SysError, SysRng},
};
use thiserror::Error;
use x25519_dalek::{PublicKey as X25519PublicKey, SharedSecret as Shared, StaticSecret as Secret};
use zeroize::ZeroizeOnDrop;

use crate::{
    key::{EncryptError as EncryptedError, EncryptWithError as EncryptedWithError, Key},
    nonce::Nonce,
    output::Output,
};

pub const SIZE: usize = 32;

pub type Array = [u8; SIZE];

pub const GENERATE: &str = "key generation failed";

#[derive(Debug, Error)]
#[error("{GENERATE}: {error}")]
pub struct GenerateError {
    error: SysError,
}

impl GenerateError {
    pub const fn new(error: SysError) -> Self {
        Self { error }
    }

    pub const fn get(self) -> SysError {
        self.error
    }
}

#[derive(ZeroizeOnDrop)]
pub struct SecretKey {
    secret: Array,
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(stringify!(SecretKey))
            .finish_non_exhaustive()
    }
}

impl SecretKey {
    pub const fn new(secret: Array) -> Self {
        Self { secret }
    }

    pub fn try_generate() -> Result<Self, GenerateError> {
        let mut output = [0; SIZE];

        SysRng
            .try_fill_bytes(output.as_mut_slice())
            .map_err(GenerateError::new)?;

        let key = Self::new(output);

        Ok(key)
    }

    pub fn generate() -> Self {
        Self::try_generate().expect(GENERATE)
    }

    pub fn expose(&self) -> &[u8] {
        self.secret.as_slice()
    }

    pub fn secret(&self) -> Secret {
        self.secret.into()
    }

    pub fn public(&self) -> PublicKey {
        let secret = self.secret();

        let reference = &secret;

        PublicKey::new(reference.into())
    }

    pub fn try_encrypted(&self, key: &Key) -> Result<Output, EncryptedError> {
        key.try_encrypt(self.expose())
    }

    pub fn encrypted(&self, key: &Key) -> Output {
        key.encrypt(self.expose())
    }

    pub fn try_encrypted_with(
        &self,
        nonce: Nonce,
        key: &Key,
    ) -> Result<Output, EncryptedWithError> {
        key.try_encrypt_with(nonce, self.expose())
    }

    pub fn encrypted_with(&self, nonce: Nonce, key: &Key) -> Output {
        key.encrypt_with(nonce, self.expose())
    }
}

#[derive(Debug, Clone, ZeroizeOnDrop)]
pub struct PublicKey {
    public: X25519PublicKey,
}

impl PublicKey {
    pub const fn new(public: X25519PublicKey) -> Self {
        Self { public }
    }

    pub const fn public(&self) -> X25519PublicKey {
        self.public
    }
}

#[derive(ZeroizeOnDrop)]
pub struct SharedKey {
    shared: Shared,
}

impl SharedKey {
    pub const fn new(shared: Shared) -> Self {
        Self { shared }
    }

    pub fn exchange(our: &SecretKey, their: &PublicKey) -> Self {
        let secret = our.secret();
        let public = their.public();

        let shared = secret.diffie_hellman(&public);

        Self::new(shared)
    }

    pub fn key(&self) -> Key {
        todo!()
    }
}

pub struct Keys {
    secret: SecretKey,
    public: PublicKey,
}

impl Keys {
    pub const fn new(secret: SecretKey, public: PublicKey) -> Self {
        Self { secret, public }
    }

    pub fn derive(secret: SecretKey) -> Self {
        let public = secret.public();

        Self::new(secret, public)
    }

    pub fn try_generate() -> Result<Self, GenerateError> {
        let secret = SecretKey::try_generate()?;

        let keys = Self::derive(secret);

        Ok(keys)
    }

    pub fn generate() -> Self {
        Self::try_generate().expect(GENERATE)
    }

    pub fn into_pair(self) -> (SecretKey, PublicKey) {
        (self.secret, self.public)
    }
}
