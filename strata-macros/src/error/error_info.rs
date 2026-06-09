use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, MetaNameValue, Token, parse_macro_input, punctuated::Punctuated,
};

pub(crate) fn derive_error_info_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let data = match &input.data {
        Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(&input, "ErrorInfo derive macro only supports enums")
                .to_compile_error()
                .into();
        }
    };

    let parsed = data
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            let info_attr = variant
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("info"))
                .ok_or_else(|| {
                    syn::Error::new_spanned(
                        variant,
                        "missing #[info(kind, code, message)] attribute",
                    )
                })?;

            let meta_items: Punctuated<MetaNameValue, Token![,]> =
                info_attr.parse_args_with(Punctuated::parse_terminated)?;

            let mut kind = None;
            let mut code = None;
            let mut message = None;
            for meta in &meta_items {
                let field = meta.path.get_ident().map(|i| i.to_string());
                let value = match &meta.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) => s.value(),
                    _ => {
                        let name = field.as_deref().unwrap_or("unknown");
                        return Err(syn::Error::new_spanned(
                            &meta.value,
                            format!("`{name}` must be a string literal"),
                        ));
                    }
                };
                match field.as_deref() {
                    Some("kind") => kind = Some(value),
                    Some("code") => code = Some(value),
                    Some("message") => message = Some(value),
                    _ => {}
                }
            }

            let kind =
                kind.ok_or_else(|| syn::Error::new_spanned(info_attr, "missing `kind` attribute"))?;
            let code =
                code.ok_or_else(|| syn::Error::new_spanned(info_attr, "missing `code` attribute"))?;
            let message = message
                .ok_or_else(|| syn::Error::new_spanned(info_attr, "missing `message` attribute"))?;

            let kind_ident = syn::Ident::new(&kind, variant_name.span());

            Ok((
                variant_name.clone(),
                kind_ident,
                code,
                message,
                variant.fields.clone(),
            ))
        })
        .collect::<Result<Vec<_>, syn::Error>>();

    let parsed = match parsed {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };

    let mut kind_arms = vec![];
    let mut code_arms = vec![];
    let mut message_arms = vec![];

    for (variant_name, kind_ident, code, message, fields) in &parsed {
        let pat = match fields {
            Fields::Unit => quote! { Self::#variant_name },
            Fields::Unnamed(_) => quote! { Self::#variant_name(..) },
            Fields::Named(_) => quote! { Self::#variant_name { .. } },
        };

        kind_arms.push(quote! {
            #pat => ::strata::error::ErrorKind::#kind_ident,
        });
        code_arms.push(quote! {
            #pat => #code,
        });
        message_arms.push(quote! {
            #pat => #message,
        });
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::strata::error::ErrorInfo for #name #ty_generics #where_clause {
            fn kind(&self) -> ::strata::error::ErrorKind {
                match self {
                    #(#kind_arms)*
                }
            }

            fn code(&self) -> &'static str {
                match self {
                    #(#code_arms)*
                }
            }

            fn message(&self) -> &'static str {
                match self {
                    #(#message_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
