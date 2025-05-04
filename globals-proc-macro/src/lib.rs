use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, parse_macro_input, Expr, Ident, Type};

struct UncheckedDeclaration {
    var_name: Ident,
    var_type: Type,
    var_default: Option<Expr>,
}
impl syn::parse::Parse for UncheckedDeclaration {
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
impl quote::ToTokens for UncheckedDeclaration {
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
                    static mut #var_name: globals::UncheckedGlobal<#var_type> = globals::UncheckedGlobal::uninit();

                    pub fn #var_name_init(value: #var_type) {
                        unsafe {
                            #var_name = globals::UncheckedGlobal::new(value);
                        }
                    }

                    pub fn #var_name_lower() -> &'static mut #var_type {
                        globals::unchecked_get_mut(&raw mut #var_name)
                    }
                });

            },
        }
    }
}
struct CheckedDeclaration(UncheckedDeclaration);
impl syn::parse::Parse for CheckedDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(UncheckedDeclaration::parse(input)?))
    }
}
impl quote::ToTokens for CheckedDeclaration {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let var_name = self.0.var_name.clone();
        let var_name_lower = Ident::new(self.0.var_name.clone().to_string().to_lowercase().as_str(), self.0.var_name.span());
        let var_name_lower_or_init = Ident::new((self.0.var_name.clone().to_string().to_lowercase() + "_or_init") .as_str(), self.0.var_name.span());
        let var_type = self.0.var_type.clone();
        match self.0.var_default {
            Some(ref var_default) => {
                tokens.extend(quote! {
                    static mut #var_name: globals::CheckedGlobal<#var_type> = globals::CheckedGlobal::new(#var_default);
                    pub fn #var_name_lower() -> core::option::Option<&'static mut #var_type> {
                        globals::checked_get_mut(&raw mut #var_name)
                    }
                    pub fn #var_name_lower_or_init<T, F: FnOnce() -> #var_type>(f: F) -> &'static mut #var_type {
                        globals::checked_get_mut_or_init(&raw mut #var_name, f)
                    }
                });
            },
            None => {
                tokens.extend(quote! {
                    static mut #var_name: globals::CheckedGlobal<#var_type> = globals::CheckedGlobal::uninit();
                    pub fn #var_name_lower() -> core::option::Option<&'static mut #var_type> {
                        globals::checked_get_mut(&raw mut #var_name)
                    }
                    pub fn #var_name_lower_or_init<F: FnOnce() -> #var_type>(f: F) -> &'static mut #var_type {
                        globals::checked_get_mut_or_init(&raw mut #var_name, f)
                    }
                });

            },
        }
    }
}


#[proc_macro_attribute]
pub fn unchecked_global(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as UncheckedDeclaration);

    quote!{
        #input
    }.into()
}

#[proc_macro_attribute]
pub fn checked_global(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as CheckedDeclaration);

    quote!{
        #input
    }.into()
}
