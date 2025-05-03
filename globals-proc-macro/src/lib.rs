use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, parse_macro_input, Expr, Ident, Type};

struct Declaration {
    var_name: Ident,
    var_type: Type,
    var_default: Option<Expr>,
}
impl syn::parse::Parse for Declaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![static]>()?;
        _ = input.parse::<syn::Token![mut]>()?;
        let var_name: Ident = input.parse()?;
        _ = input.parse::<syn::Token![:]>()?;
        let var_type: Type = input.parse()?;
        match input.parse::<syn::Token![=]>() {
            Ok(_) => {
                        let var_default: Expr = input.parse()?;
                        _ = input.parse::<syn::Token![;]>()?;
                        Ok(Self {
                            var_name,
                            var_type,
                            var_default: Some(var_default),
                        })
                    }
            Err(_) => {
                _ = input.parse::<syn::Token![;]>()?;
                Ok(Self {
                    var_name,
                    var_type,
                    var_default: None,
                })
            },
        }
    }
}
impl quote::ToTokens for Declaration {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let var_name = self.var_name.clone();
        let var_name_lower = Ident::new(self.var_name.clone().to_string().to_lowercase().as_str(), self.var_name.span());
        let var_type = self.var_type.clone();
        match self.var_default {
            Some(ref var_default) => {
                let var_type = self.var_type.clone();
                tokens.extend(quote! {
                    static mut #var_name: #var_type = #var_default;
                    pub fn #var_name_lower() -> &'static mut #var_type {
                        unsafe {
                            &mut *(&raw mut #var_name as *mut #var_type)
                        }
                    }
                });
            },
            None => {
                let var_name_init = Ident::new((self.var_name.clone().to_string().to_lowercase() + "_init").as_str(), self.var_name.span());
                tokens.extend(quote! {
                    static mut #var_name: #var_type = unsafe { std::mem::zeroed::<#var_type>() };

                    pub fn #var_name_init<F: std::ops::FnMut() -> #var_type >(mut f: F) {
                        unsafe {
                            #var_name = f();
                        }
                    }

                    pub fn #var_name_lower() -> &'static mut #var_type {
                        unsafe {
                            &mut *(&raw mut #var_name)
                        }
                    }
                });

            },
        }
    }
}

#[proc_macro_attribute]
pub fn unsafe_global(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Declaration);

    quote!{
        #input
    }.into()
}
