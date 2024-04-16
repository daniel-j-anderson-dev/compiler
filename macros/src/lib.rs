use proc_macro::{self, TokenStream};

#[proc_macro]
pub fn generate_4_fn(_item: TokenStream) -> TokenStream {
    return "const fn generated_4_fn() -> usize { 4 }".parse().unwrap();
}

