use lyrichar_chords::chords::Chord;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub fn try_chord<S: AsRef<str>>(string: S) -> Result<TokenStream> {
    let chord = Chord::parse(string)?;

    let tokens = quote! {
        #chord
    };

    Ok(tokens)
}
