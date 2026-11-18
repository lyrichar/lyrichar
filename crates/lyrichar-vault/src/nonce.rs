use std::fmt;

use chacha20poly1305::XNonce;
use rand::{
    TryRng,
    rngs::{SysError, SysRng},
};
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const SIZE: usize = 24;

pub type Array = [u8; SIZE];

pub const NONCE: &str = "nonce generation failed";

#[derive(Debug, Error)]
#[error("{NONCE}: {error}")]
pub struct NonceError {
    error: SysError,
}

impl NonceError {
    pub const fn new(error: SysError) -> Self {
        Self { error }
    }

    pub const fn get(self) -> SysError {
        self.error
    }
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct Nonce {
    array: Array,
}

impl fmt::Debug for Nonce {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(stringify!(Nonce))
            .finish_non_exhaustive()
    }
}

impl Nonce {
    pub const fn new(array: Array) -> Self {
        Self { array }
    }

    pub fn try_generate() -> Result<Self, NonceError> {
        let mut output = [0; SIZE];

        SysRng
            .try_fill_bytes(output.as_mut_slice())
            .map_err(NonceError::new)?;

        let nonce = Self::new(output);

        Ok(nonce)
    }

    pub fn generate() -> Self {
        Self::try_generate().expect(NONCE)
    }

    pub fn as_x(&self) -> &XNonce {
        let nonce = &self.array;

        nonce.into()
    }
}

impl AsRef<[u8]> for Nonce {
    fn as_ref(&self) -> &[u8] {
        self.array.as_slice()
    }
}
