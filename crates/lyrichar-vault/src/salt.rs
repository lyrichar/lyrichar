use std::fmt;

use rand::{
    TryRng,
    rngs::{SysError, SysRng},
};
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const SIZE: usize = 16;

pub type Array = [u8; SIZE];

pub const SALT: &str = "salt generation failed";

#[derive(Debug, Error)]
#[error("{SALT}: {error}")]
pub struct SaltError {
    error: SysError,
}

impl SaltError {
    pub const fn new(error: SysError) -> Self {
        Self { error }
    }

    pub const fn get(self) -> SysError {
        self.error
    }
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct Salt {
    array: Array,
}

impl fmt::Debug for Salt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(stringify!(Salt))
            .finish_non_exhaustive()
    }
}

impl Salt {
    pub const fn new(array: Array) -> Self {
        Self { array }
    }

    pub fn try_generate() -> Result<Self, SaltError> {
        let mut output = [0; SIZE];

        SysRng
            .try_fill_bytes(output.as_mut_slice())
            .map_err(SaltError::new)?;

        let salt = Self::new(output);

        Ok(salt)
    }

    pub fn generate() -> Self {
        Self::try_generate().expect(SALT)
    }
}

impl AsRef<[u8]> for Salt {
    fn as_ref(&self) -> &[u8] {
        self.array.as_slice()
    }
}
