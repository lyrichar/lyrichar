use proc_macro::TokenStream;
use syn::{Error, LitStr, parse_macro_input};

mod expand;

#[proc_macro]
pub fn chord(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitStr);

    let string = literal.value();

    expand::try_chord(string)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
