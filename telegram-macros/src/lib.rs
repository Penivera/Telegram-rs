use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

#[proc_macro_attribute]
pub fn bot(_args: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the item, maybe add runtime setup later
    item
}

#[proc_macro_attribute]
pub fn command(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let _name = &input.sig.ident;
    
    // We want to wrap this function to be usable as a handler
    // But for "highly abstracted", maybe we registers it?
    // For MVP, let's just leave it as is but maybe add a metadata struct?
    
    // Actually, to make it useful, we can't do much without a registry or traits.
    // Let's just return the function for now, but in a real implementation we would 
    // generate code to register it.
    
    let expanded = quote! {
        #input
    };
    
    TokenStream::from(expanded)
}
