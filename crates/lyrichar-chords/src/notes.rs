use std::{fmt, str::FromStr};

use non_empty_str::{EmptyStr, FromNonEmptyStr, NonEmptyStr, NonEmptyString, const_non_empty_str};
use thiserror::Error;

use crate::statics::{StaticNonEmptyStr, StaticStr};

pub const A_NOTE: &str = "A";
pub const A_SHARP_NOTE: &str = "A#";
pub const A_FLAT_NOTE: &str = "Ab";
pub const B_NOTE: &str = "B";
pub const B_FLAT_NOTE: &str = "Bb";
pub const C_NOTE: &str = "C";
pub const C_SHARP_NOTE: &str = "C#";
pub const D_NOTE: &str = "D";
pub const D_SHARP_NOTE: &str = "D#";
pub const D_FLAT_NOTE: &str = "Db";
pub const E_NOTE: &str = "E";
pub const E_FLAT_NOTE: &str = "Eb";
pub const F_NOTE: &str = "F";
pub const F_SHARP_NOTE: &str = "F#";
pub const G_NOTE: &str = "G";
pub const G_SHARP_NOTE: &str = "G#";
pub const G_FLAT_NOTE: &str = "Gb";

pub const A: &NonEmptyStr = const_non_empty_str!(A_NOTE);
pub const A_SHARP: &NonEmptyStr = const_non_empty_str!(A_SHARP_NOTE);
pub const A_FLAT: &NonEmptyStr = const_non_empty_str!(A_FLAT_NOTE);
pub const B: &NonEmptyStr = const_non_empty_str!(B_NOTE);
pub const B_FLAT: &NonEmptyStr = const_non_empty_str!(B_FLAT_NOTE);
pub const C: &NonEmptyStr = const_non_empty_str!(C_NOTE);
pub const C_SHARP: &NonEmptyStr = const_non_empty_str!(C_SHARP_NOTE);
pub const D: &NonEmptyStr = const_non_empty_str!(D_NOTE);
pub const D_SHARP: &NonEmptyStr = const_non_empty_str!(D_SHARP_NOTE);
pub const D_FLAT: &NonEmptyStr = const_non_empty_str!(D_FLAT_NOTE);
pub const E: &NonEmptyStr = const_non_empty_str!(E_NOTE);
pub const E_FLAT: &NonEmptyStr = const_non_empty_str!(E_FLAT_NOTE);
pub const F: &NonEmptyStr = const_non_empty_str!(F_NOTE);
pub const F_SHARP: &NonEmptyStr = const_non_empty_str!(F_SHARP_NOTE);
pub const G: &NonEmptyStr = const_non_empty_str!(G_NOTE);
pub const G_SHARP: &NonEmptyStr = const_non_empty_str!(G_SHARP_NOTE);
pub const G_FLAT: &NonEmptyStr = const_non_empty_str!(G_FLAT_NOTE);

#[derive(Debug, Error)]
#[error("invalid note `{non_empty}`")]
pub struct NonEmptyError {
    non_empty: NonEmptyString,
}

impl NonEmptyError {
    pub const fn new(non_empty: NonEmptyString) -> Self {
        Self { non_empty }
    }

    pub const fn as_non_empty_str(&self) -> &NonEmptyStr {
        self.non_empty.as_non_empty_str()
    }

    pub const fn as_str(&self) -> &str {
        self.non_empty.as_str()
    }

    pub fn get_non_empty(self) -> NonEmptyString {
        self.non_empty
    }

    pub fn get(self) -> String {
        self.non_empty.into_string()
    }
}

#[derive(Debug, Error)]
#[error("the note can not be empty: {error}")]
pub struct EmptyError {
    #[from]
    error: EmptyStr,
}

impl EmptyError {
    pub const fn new(error: EmptyStr) -> Self {
        Self { error }
    }

    pub const fn empty() -> Self {
        Self::new(EmptyStr)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Note {
    A,
    AFlat,
    ASharp,
    B,
    BFlat,
    C,
    CSharp,
    D,
    DSharp,
    DFlat,
    E,
    EFlat,
    F,
    FSharp,
    G,
    GSharp,
    GFlat,
}

impl fmt::Display for Note {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.non_empty_str().fmt(formatter)
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Empty(#[from] EmptyError),
    NonEmpty(#[from] NonEmptyError),
}

impl FromStr for Note {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse(string)
    }
}

impl FromNonEmptyStr for Note {
    type Error = NonEmptyError;

    fn from_non_empty_str(string: &NonEmptyStr) -> Result<Self, Self::Error> {
        Self::parse_non_empty(string)
    }
}

pub fn check_non_empty(string: &str) -> Result<&NonEmptyStr, EmptyError> {
    let non_empty = string.try_into()?;

    Ok(non_empty)
}

impl Note {
    pub const fn non_empty_str(self) -> StaticNonEmptyStr {
        match self {
            Self::A => A,
            Self::AFlat => A_FLAT,
            Self::ASharp => A_SHARP,
            Self::B => B,
            Self::BFlat => B_FLAT,
            Self::C => C,
            Self::CSharp => C_SHARP,
            Self::D => D,
            Self::DSharp => D_SHARP,
            Self::DFlat => D_FLAT,
            Self::E => E,
            Self::EFlat => E_FLAT,
            Self::F => F,
            Self::FSharp => F_SHARP,
            Self::G => G,
            Self::GSharp => G_SHARP,
            Self::GFlat => G_FLAT,
        }
    }

    pub const fn str(self) -> StaticStr {
        self.non_empty_str().as_str()
    }

    pub fn parse<S: AsRef<str>>(string: S) -> Result<Self, Error> {
        let note = check_non_empty(string.as_ref())?.parse_non_empty()?;

        Ok(note)
    }

    pub fn parse_non_empty<S: AsRef<NonEmptyStr>>(non_empty: S) -> Result<Self, NonEmptyError> {
        let string = non_empty.as_ref();

        match string.as_str() {
            A_NOTE => Ok(Self::A),
            A_SHARP_NOTE => Ok(Self::ASharp),
            A_FLAT_NOTE => Ok(Self::AFlat),
            B_NOTE => Ok(Self::B),
            B_FLAT_NOTE => Ok(Self::BFlat),
            C_NOTE => Ok(Self::C),
            C_SHARP_NOTE => Ok(Self::CSharp),
            D_NOTE => Ok(Self::D),
            D_FLAT_NOTE => Ok(Self::DFlat),
            D_SHARP_NOTE => Ok(Self::DSharp),
            E_NOTE => Ok(Self::E),
            E_FLAT_NOTE => Ok(Self::EFlat),
            F_NOTE => Ok(Self::F),
            F_SHARP_NOTE => Ok(Self::FSharp),
            G_NOTE => Ok(Self::G),
            G_FLAT_NOTE => Ok(Self::GFlat),
            G_SHARP_NOTE => Ok(Self::GSharp),
            _ => Err(NonEmptyError::new(string.to_owned())),
        }
    }
}

#[cfg(feature = "proc-macro")]
mod tokens {
    use proc_macro2::{Span, TokenStream};
    use quote::{ToTokens, quote};
    use syn::Error;

    use super::{Error as NoteError, Note};

    impl From<NoteError> for Error {
        fn from(error: NoteError) -> Self {
            Self::new(Span::call_site(), error)
        }
    }

    impl Note {
        fn variant(self) -> TokenStream {
            match self {
                Self::A => quote! {
                    A
                },
                Self::AFlat => quote! {
                    AFlat
                },
                Self::ASharp => quote! {
                    ASharp
                },
                Self::B => quote! {
                    B
                },
                Self::BFlat => quote! {
                    BFlat
                },
                Self::C => quote! {
                    C
                },
                Self::CSharp => quote! {
                    CSharp
                },
                Self::D => quote! {
                    D
                },
                Self::DSharp => quote! {
                    DSharp
                },
                Self::DFlat => quote! {
                    DFlat
                },
                Self::E => quote! {
                    E
                },
                Self::EFlat => quote! {
                    EFlat
                },
                Self::F => quote! {
                    F
                },
                Self::FSharp => quote! {
                    FSharp
                },
                Self::G => quote! {
                    G
                },
                Self::GSharp => quote! {
                    GSharp
                },
                Self::GFlat => quote! {
                    GFlat
                },
            }
        }
    }

    impl ToTokens for Note {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                lyrichar_chords::notes::Note
            };

            let variant = self.variant();

            let tokens = quote! {
                #path::#variant
            };

            stream.extend(tokens);
        }
    }
}
