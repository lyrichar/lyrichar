use std::{fmt, str::FromStr};

use bon::Builder;
use thiserror::Error;

use crate::{
    notes::{Error as NoteError, Note},
    qualities::{Error as QualityError, Quality},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Builder)]
pub struct Chord {
    root: Note,
    quality: Quality,
    bass: Option<Note>,
}

pub const SLASH: &str = "/";

impl fmt::Display for Chord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let root = self.root.non_empty_str();
        let suffix = self.quality.suffix();

        if let Some(bass) = self.bass.map(|note| note.non_empty_str()) {
            write!(formatter, "{root}{suffix}{SLASH}{bass}")
        } else {
            write!(formatter, "{root}{suffix}")
        }
    }
}

pub const A_CHAR: char = 'A';
pub const B_CHAR: char = 'B';
pub const C_CHAR: char = 'C';
pub const D_CHAR: char = 'D';
pub const E_CHAR: char = 'E';
pub const F_CHAR: char = 'F';
pub const G_CHAR: char = 'G';
pub const SHARP_CHAR: char = '#';
pub const FLAT_CHAR: char = 'b';

#[derive(Debug, Error)]
pub enum Error {
    #[error("the chord is empty")]
    Empty,
    #[error("the chord root can not be empty")]
    EmptyRoot,
    #[error("invalid root")]
    InvalidRoot,
    #[error("error parsing quality: {0}")]
    Quality(QualityError),
    #[error("error parsing bass: {0}")]
    Bass(NoteError),
    #[error("too many slashes (`{SLASH}`)")]
    TooMany,
}

impl FromStr for Chord {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse(string)
    }
}

impl Chord {
    pub fn parse<I: AsRef<str>>(input: I) -> Result<Self, Error> {
        let string = input.as_ref();

        // splitting the string early
        let mut split = string.split(SLASH);

        // the main part can not be empty
        let main = split.next().ok_or(Error::Empty)?;

        let mut chars = main.chars();

        // extract the first character for matching
        let first = chars.next().ok_or(Error::EmptyRoot)?;

        let rest = chars.as_str();

        let mut rest_chars = rest.chars();

        // optionally, the second character as the modifier
        let second = rest_chars.next();

        let final_rest = rest_chars.as_str();

        // match the first and second characters to determine `root` and `suffix`
        let (root, suffix) = match (first, second) {
            (A_CHAR, Some(SHARP_CHAR)) => (Note::ASharp, final_rest),
            (A_CHAR, Some(FLAT_CHAR)) => (Note::AFlat, final_rest),
            (A_CHAR, _) => (Note::A, rest),
            (B_CHAR, Some(FLAT_CHAR)) => (Note::BFlat, final_rest),
            (B_CHAR, _) => (Note::B, rest),
            (C_CHAR, Some(SHARP_CHAR)) => (Note::CSharp, final_rest),
            (C_CHAR, _) => (Note::C, rest),
            (D_CHAR, Some(SHARP_CHAR)) => (Note::DSharp, final_rest),
            (D_CHAR, Some(FLAT_CHAR)) => (Note::DFlat, final_rest),
            (D_CHAR, _) => (Note::D, rest),
            (E_CHAR, Some(FLAT_CHAR)) => (Note::EFlat, final_rest),
            (E_CHAR, _) => (Note::E, rest),
            (F_CHAR, Some(SHARP_CHAR)) => (Note::FSharp, final_rest),
            (F_CHAR, _) => (Note::F, rest),
            (G_CHAR, Some(SHARP_CHAR)) => (Note::GSharp, final_rest),
            (G_CHAR, Some(FLAT_CHAR)) => (Note::GFlat, final_rest),
            (G_CHAR, _) => (Note::G, rest),
            _ => return Err(Error::InvalidRoot),
        };

        let quality = suffix.parse().map_err(Error::Quality)?;

        let bass = split
            .next()
            .map(|string| string.parse())
            .transpose()
            .map_err(Error::Bass)?;

        // more than one slash is not allowed
        let None = split.next() else {
            return Err(Error::TooMany);
        };

        let chord = Self::builder()
            .root(root)
            .quality(quality)
            .maybe_bass(bass)
            .build();

        Ok(chord)
    }

    pub const fn note(root: Note) -> Self {
        Self::note_with(root, Quality::DEFAULT)
    }

    pub const fn note_with(root: Note, quality: Quality) -> Self {
        Self {
            root,
            quality,
            bass: None,
        }
    }

    pub const fn note_with_bass(root: Note, quality: Quality, bass: Note) -> Self {
        Self {
            root,
            quality,
            bass: Some(bass),
        }
    }

    pub const fn root(&self) -> Note {
        self.root
    }

    pub const fn quality(&self) -> Quality {
        self.quality
    }

    pub const fn bass(&self) -> Option<Note> {
        self.bass
    }
}

#[cfg(feature = "proc-macro")]
mod tokens {
    use proc_macro2::{Span, TokenStream};
    use quote::{ToTokens, quote};
    use syn::Error;

    use super::{Chord, Error as ChordError};

    impl From<ChordError> for Error {
        fn from(error: ChordError) -> Self {
            Self::new(Span::call_site(), error)
        }
    }

    impl ToTokens for Chord {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                lyrichar_chords::chords::Chord
            };

            let root = self.root();
            let quality = self.quality();

            let tokens = if let Some(bass) = self.bass() {
                quote! {
                    #path::note_with_bass(#root, #quality, #bass)
                }
            } else {
                quote! {
                    #path::note_with(#root, #quality)
                }
            };

            stream.extend(tokens);
        }
    }
}
