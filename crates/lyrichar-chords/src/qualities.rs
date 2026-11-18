use std::str::FromStr;

use thiserror::Error;

use crate::statics::StaticStr;

pub const MAJOR: &str = "";
pub const MINOR: &str = "m";
pub const SEVENTH: &str = "7";
pub const MAJOR_SEVENTH: &str = "maj7";
pub const MINOR_SEVENTH: &str = "m7";
pub const SIXTH: &str = "6";
pub const MINOR_SIXTH: &str = "m6";
pub const NINTH_DOMINANT: &str = "9";
pub const MAJOR_NINTH: &str = "maj9";
pub const MINOR_NINTH: &str = "m9";
pub const SUSPENDED_TWO: &str = "sus2";
pub const SUSPENDED_FOUR: &str = "sus4";
pub const DIMINISHED: &str = "dim";
pub const DIMINISHED_SEVENTH: &str = "dim7";
pub const AUGMENTED: &str = "aug";
pub const ADD_NINE: &str = "add9";
pub const HALF_DIMINISHED: &str = "m7b5";

#[derive(Debug, Error)]
#[error("invalid quality `{string}`")]
pub struct Error {
    string: String,
}

impl Error {
    pub const fn new(string: String) -> Self {
        Self { string }
    }

    pub const fn as_str(&self) -> &str {
        self.string.as_str()
    }

    pub fn get(self) -> String {
        self.string
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quality {
    Major,
    Minor,
    Seventh,
    MajorSeventh,
    MinorSeventh,
    Sixth,
    MinorSixth,
    NinthDominant,
    MajorNinth,
    MinorNinth,
    SuspendedTwo,
    SuspendedFour,
    Diminished,
    DiminishedSeventh,
    Augmented,
    AddNine,
    HalfDiminished,
}

impl FromStr for Quality {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse(string)
    }
}

impl Default for Quality {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Quality {
    pub const fn suffix(self) -> StaticStr {
        match self {
            Self::Major => MAJOR,
            Self::Minor => MINOR,
            Self::Seventh => SEVENTH,
            Self::MajorSeventh => MAJOR_SEVENTH,
            Self::MinorSeventh => MINOR_SEVENTH,
            Self::Sixth => SIXTH,
            Self::MinorSixth => MINOR_SIXTH,
            Self::NinthDominant => NINTH_DOMINANT,
            Self::MajorNinth => MAJOR_NINTH,
            Self::MinorNinth => MINOR_NINTH,
            Self::SuspendedTwo => SUSPENDED_TWO,
            Self::SuspendedFour => SUSPENDED_FOUR,
            Self::Diminished => DIMINISHED,
            Self::DiminishedSeventh => DIMINISHED_SEVENTH,
            Self::Augmented => AUGMENTED,
            Self::AddNine => ADD_NINE,
            Self::HalfDiminished => HALF_DIMINISHED,
        }
    }

    pub fn parse<S: AsRef<str>>(string: S) -> Result<Self, Error> {
        match string.as_ref() {
            MAJOR => Ok(Self::Major),
            MINOR => Ok(Self::Minor),
            SEVENTH => Ok(Self::Seventh),
            MAJOR_SEVENTH => Ok(Self::MajorSeventh),
            MINOR_SEVENTH => Ok(Self::MinorSeventh),
            SIXTH => Ok(Self::Sixth),
            MINOR_SIXTH => Ok(Self::MinorSixth),
            NINTH_DOMINANT => Ok(Self::NinthDominant),
            MAJOR_NINTH => Ok(Self::MajorNinth),
            MINOR_NINTH => Ok(Self::MinorNinth),
            SUSPENDED_TWO => Ok(Self::SuspendedTwo),
            SUSPENDED_FOUR => Ok(Self::SuspendedFour),
            DIMINISHED => Ok(Self::Diminished),
            DIMINISHED_SEVENTH => Ok(Self::DiminishedSeventh),
            AUGMENTED => Ok(Self::Augmented),
            ADD_NINE => Ok(Self::AddNine),
            HALF_DIMINISHED => Ok(Self::HalfDiminished),
            string => Err(Error::new(string.to_owned())),
        }
    }

    pub const DEFAULT: Self = Self::Major;
}

#[cfg(feature = "proc-macro")]
mod tokens {
    use proc_macro2::{Span, TokenStream};
    use quote::{ToTokens, quote};
    use syn::Error;

    use super::{Error as QualityError, Quality};

    impl From<QualityError> for Error {
        fn from(error: QualityError) -> Self {
            Self::new(Span::call_site(), error)
        }
    }

    impl Quality {
        fn variant(self) -> TokenStream {
            match self {
                Self::Major => quote! {
                    Major
                },
                Self::Minor => quote! {
                    Minor
                },
                Self::Seventh => quote! {
                    Seventh
                },
                Self::MajorSeventh => quote! {
                    MajorSeventh
                },
                Self::MinorSeventh => quote! {
                    MinorSeventh
                },
                Self::Sixth => quote! {
                    Sixth
                },
                Self::MinorSixth => quote! {
                    MinorSixth
                },
                Self::NinthDominant => quote! {
                    NinthDominant
                },
                Self::MajorNinth => quote! {
                    MajorNinth
                },
                Self::MinorNinth => quote! {
                    MinorNinth
                },
                Self::SuspendedTwo => quote! {
                    SuspendedTwo
                },
                Self::SuspendedFour => quote! {
                    SuspendedFour
                },
                Self::Diminished => quote! {
                    Diminished
                },
                Self::DiminishedSeventh => quote! {
                    DiminishedSeventh
                },
                Self::Augmented => quote! {
                    Augmented
                },
                Self::AddNine => quote! {
                    AddNine
                },
                Self::HalfDiminished => quote! {
                    HalfDiminished
                },
            }
        }
    }

    impl ToTokens for Quality {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                lyrichar_chords::qualities::Quality
            };

            let variant = self.variant();

            let tokens = quote! {
                #path::#variant
            };

            stream.extend(tokens);
        }
    }
}
