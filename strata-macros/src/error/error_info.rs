use std::{collections::BTreeSet, fmt};

use quote::{format_ident, quote, quote_spanned};
use syn::{
    Data, DeriveInput, Fields, Lit, MetaNameValue, Token, parse_macro_input, punctuated::Punctuated,
};

enum Placeholder {
    Positional(usize),
    Named(String),
}

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Placeholder::Positional(idx) => write!(f, "{idx}"),
            Placeholder::Named(name) => f.write_str(name),
        }
    }
}

struct VariantInfo {
    variant_name: syn::Ident,
    is_transparent: bool,
    transparent_field_ident: Option<syn::Ident>,
    kind_ident: syn::Ident,
    code: String,
    message: String,
    message_span: proc_macro2::Span,
    fields: Fields,
    placeholders: Vec<Placeholder>,
    field_refs: BTreeSet<String>,
}

pub(crate) fn derive_error_info_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
                .ok_or_else(|| syn::Error::new_spanned(variant, "missing #[info] attribute"))?;

            let is_transparent = {
                let tokens = match &info_attr.meta {
                    syn::Meta::List(list) => list.tokens.clone(),
                    _ => {
                        return Err(syn::Error::new_spanned(info_attr, "expected #[info(...)]"));
                    }
                };

                let tts: Vec<proc_macro2::TokenTree> = tokens
                    .into_iter()
                    .filter(
                        |tt| !matches!(tt, proc_macro2::TokenTree::Punct(p) if p.as_char() == ','),
                    )
                    .collect();

                tts.len() == 1
                    && matches!(&tts[0], proc_macro2::TokenTree::Ident(id) if id == "transparent")
            };

            if is_transparent {
                let field_ident = validate_transparent_variant(variant)?;

                return Ok(VariantInfo {
                    variant_name: variant_name.clone(),
                    is_transparent: true,
                    transparent_field_ident: Some(field_ident),
                    kind_ident: format_ident!("unused"),
                    code: String::new(),
                    message: String::new(),
                    message_span: proc_macro2::Span::call_site(),
                    fields: variant.fields.clone(),
                    placeholders: Vec::new(),
                    field_refs: BTreeSet::new(),
                });
            }

            let meta_items: Punctuated<MetaNameValue, Token![,]> =
                info_attr.parse_args_with(Punctuated::parse_terminated)?;

            let mut kind = None;
            let mut code = None;
            let mut message = None;
            let mut message_span = proc_macro2::Span::call_site();
            for meta in &meta_items {
                let field = meta.path.get_ident().map(|i| i.to_string());
                let (value, span) = match &meta.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: Lit::Str(s), ..
                    }) => (s.value(), s.span()),
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
                    Some("message") => {
                        message = Some(value);
                        message_span = span;
                    }
                    _ => {}
                }
            }

            let kind =
                kind.ok_or_else(|| syn::Error::new_spanned(info_attr, "missing `kind` attribute"))?;
            let code =
                code.ok_or_else(|| syn::Error::new_spanned(info_attr, "missing `code` attribute"))?;
            let message = message
                .ok_or_else(|| syn::Error::new_spanned(info_attr, "missing `message` attribute"))?;

            let placeholders = parse_placeholders(&message);
            let field_refs =
                field_references(&placeholders, &variant.fields, variant_name, info_attr)?;

            let kind_ident = syn::Ident::new(&kind, variant_name.span());

            Ok(VariantInfo {
                variant_name: variant_name.clone(),
                is_transparent: false,
                transparent_field_ident: None,
                kind_ident,
                code,
                message,
                message_span,
                fields: variant.fields.clone(),
                placeholders,
                field_refs,
            })
        })
        .collect::<Result<Vec<_>, syn::Error>>();

    let parsed = match parsed {
        Ok(v) => v,
        Err(e) => return proc_macro::TokenStream::from(e.to_compile_error()),
    };

    let mut kind_arms: Vec<proc_macro2::TokenStream> = vec![];
    let mut code_arms: Vec<proc_macro2::TokenStream> = vec![];
    let mut message_arms: Vec<proc_macro2::TokenStream> = vec![];

    for info in &parsed {
        let VariantInfo {
            variant_name,
            is_transparent,
            transparent_field_ident,
            kind_ident,
            code,
            message,
            message_span,
            fields,
            placeholders,
            field_refs,
        } = info;

        if *is_transparent {
            let field_ident = transparent_field_ident.as_ref().unwrap();
            let pat = transparent_pattern(variant_name, fields, field_ident);

            kind_arms.push(quote! {
                #pat => ::strata::error::ErrorInfo::kind(#field_ident),
            });
            code_arms.push(quote! {
                #pat => ::strata::error::ErrorInfo::code(#field_ident),
            });
            message_arms.push(quote! {
                #pat => ::strata::error::ErrorInfo::message(#field_ident),
            });
        } else {
            let pat = message_pattern(variant_name, fields, field_refs);
            kind_arms.push(quote! { #pat => ::strata::error::ErrorKind::#kind_ident, });
            code_arms.push(quote! { #pat => #code, });

            let msg_body = message_body(message, *message_span, placeholders);
            message_arms.push(quote! { #pat => #msg_body, });
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        #[allow(unused_variables)]
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

            fn message(&self) -> ::std::borrow::Cow<'static, str> {
                match self {
                    #(#message_arms)*
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn validate_transparent_variant(variant: &syn::Variant) -> Result<syn::Ident, syn::Error> {
    match &variant.fields {
        Fields::Unit => Err(syn::Error::new_spanned(
            variant,
            "#[info(transparent)] requires exactly one field",
        )),
        Fields::Unnamed(unnamed) => {
            if unnamed.unnamed.len() != 1 {
                return Err(syn::Error::new_spanned(
                    variant,
                    "#[info(transparent)] requires exactly one field",
                ));
            }
            Ok(format_ident!("inner"))
        }
        Fields::Named(named) => {
            if named.named.len() != 1 {
                return Err(syn::Error::new_spanned(
                    variant,
                    "#[info(transparent)] requires exactly one field",
                ));
            }
            named
                .named
                .first()
                .unwrap()
                .ident
                .clone()
                .ok_or_else(|| syn::Error::new_spanned(variant, "field must have an identifier"))
        }
    }
}

fn transparent_pattern(
    variant_name: &syn::Ident,
    fields: &Fields,
    field_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unnamed(_) => quote! { Self::#variant_name(#field_ident) },
        Fields::Named(_) => quote! { Self::#variant_name { #field_ident } },
        Fields::Unit => unreachable!("already validated"),
    }
}

fn parse_placeholders(message: &str) -> Vec<Placeholder> {
    let mut placeholders = Vec::new();
    let mut chars = message.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            if let Some('{') = chars.peek() {
                chars.next();
                continue;
            }
            let mut raw = String::new();
            for next in chars.by_ref() {
                if next == '}' {
                    if !raw.is_empty() {
                        if let Ok(idx) = raw.parse::<usize>() {
                            placeholders.push(Placeholder::Positional(idx));
                        } else {
                            placeholders.push(Placeholder::Named(raw));
                        }
                    }
                    break;
                }
                raw.push(next);
            }
        }
    }

    placeholders
}

fn field_references(
    placeholders: &[Placeholder],
    fields: &Fields,
    variant_name: &syn::Ident,
    info_attr: &syn::Attribute,
) -> Result<BTreeSet<String>, syn::Error> {
    let mut refs = BTreeSet::new();

    for ph in placeholders {
        match (ph, fields) {
            (Placeholder::Named(name), _) if name.contains(':') => {
                return Err(syn::Error::new_spanned(
                    info_attr,
                    format!(
                        "format specifier in placeholder `{{{name}}}` is not supported, \
                         use {{0}}, {{1}}, or {{field_name}}"
                    ),
                ));
            }
            (Placeholder::Positional(_), Fields::Unit) => {
                return Err(syn::Error::new_spanned(
                    info_attr,
                    format!(
                        "variant `{variant_name}` is a unit variant, \
                         cannot use positional placeholder `{{{ph}}}`",
                    ),
                ));
            }
            (Placeholder::Positional(idx), Fields::Unnamed(unnamed)) => {
                if *idx >= unnamed.unnamed.len() {
                    return Err(syn::Error::new_spanned(
                        info_attr,
                        format!(
                            "variant `{variant_name}` has {} field(s), \
                             placeholder `{{{ph}}}` is out of range",
                            unnamed.unnamed.len(),
                        ),
                    ));
                }
                refs.insert(format!("_{idx}"));
            }
            (Placeholder::Positional(_), Fields::Named(_)) => {
                return Err(syn::Error::new_spanned(
                    info_attr,
                    format!(
                        "variant `{variant_name}` has named fields, \
                         placeholder `{{{ph}}}` must use field name",
                    ),
                ));
            }
            (Placeholder::Named(name), Fields::Named(named)) => {
                if named
                    .named
                    .iter()
                    .any(|f| f.ident.as_ref().is_some_and(|id| id == name.as_str()))
                {
                    refs.insert(name.clone());
                }
            }
            (Placeholder::Named(_), Fields::Unit | Fields::Unnamed(_)) => {
                // external constant, ok
            }
        }
    }

    Ok(refs)
}

fn message_pattern(
    variant_name: &syn::Ident,
    fields: &Fields,
    field_refs: &BTreeSet<String>,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unit => {
            quote! { Self::#variant_name }
        }
        Fields::Unnamed(unnamed) => {
            if field_refs.is_empty() {
                return quote! { Self::#variant_name(..) };
            }
            let mut bindings = Vec::new();
            for i in 0..unnamed.unnamed.len() {
                let ident = format_ident!("_{i}");
                bindings.push(quote! { #ident });
            }
            quote! { Self::#variant_name(#(#bindings,)*) }
        }
        Fields::Named(named) => {
            let ids = named.named.iter().filter_map(|f| f.ident.as_ref());
            quote! { Self::#variant_name { #(#ids),* } }
        }
    }
}

fn message_body(
    message: &str,
    span: proc_macro2::Span,
    placeholders: &[Placeholder],
) -> proc_macro2::TokenStream {
    let mut format_str = message.to_owned();
    let mut bindings = Vec::new();
    let mut seen_named = BTreeSet::new();

    for ph in placeholders {
        match ph {
            Placeholder::Positional(idx) => {
                format_str = format_str.replacen(&format!("{{{idx}}}"), "{}", 1);
                let ident = syn::Ident::new(&format!("_{idx}"), span);
                bindings.push(quote_spanned!(span=> #ident));
            }
            Placeholder::Named(name) => {
                if !seen_named.insert(name) {
                    continue;
                }
                let ident = syn::Ident::new(name, span);
                bindings.push(quote_spanned!(span=> #ident = #ident));
            }
        }
    }

    if bindings.is_empty() {
        return quote_spanned!(span=> ::std::borrow::Cow::Borrowed(#format_str));
    }

    quote_spanned!(span=> {
        ::std::borrow::Cow::Owned(::std::format!(#format_str, #(#bindings,)*))
    })
}
