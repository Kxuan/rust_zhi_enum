extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashSet;

use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Error, Ident, parse_macro_input, Result, Variant};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

use discriminant::{Discriminant as Disc, Generator as DiscGen};

mod kw {
    syn::custom_keyword!(unknown);
}

mod discriminant;

#[derive(Debug)]
struct FieldAttributeArgs {
    is_unknown: bool,
}

impl Parse for FieldAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        if !input.lookahead1().peek(kw::unknown) {
            Err(syn::Error::new(input.span(), "unexpected args. only \"unknown\" is supported."))
        } else {
            input.parse::<kw::unknown>().unwrap();
            Ok(Self {
                is_unknown: true,
            })
        }
    }
}

#[derive(Debug)]
struct UnknownVariant {
    ident: Ident,
}

#[derive(Debug)]
struct NormalVariant {
    ident: Ident,
    discriminant: Disc,
}

#[derive(Debug)]
struct EnumDefinition {
    ident: Ident,
    unknown: Option<UnknownVariant>,
    normals: Vec<NormalVariant>,

    repr: Ident,
    next_discriminant: DiscGen,
}

enum VariantKind {
    UnknownVariant(UnknownVariant),
    NormalVariant(NormalVariant),
}

impl EnumDefinition {
    fn parse_repr(input: &DeriveInput) -> Result<Ident> {
        for attr in &input.attrs {
            let ident = match attr.path.get_ident() {
                None => { continue; }
                Some(ident) => ident,
            };
            if ident != "repr" {
                continue;
            }

            let args = attr.parse_args::<Ident>()?;

            return Ok(args.clone());
        }

        Err(Error::new(input.span(), "add #[repr({integer data type})] to your enum"))
    }

    fn new(input: DeriveInput) -> Result<EnumDefinition> {
        let data = match &input.data {
            Data::Enum(e) => e,
            _ => {
                return Err(Error::new(input.span(), "require enum"));
            }
        };

        let repr = Self::parse_repr(&input)?;
        let mut r = EnumDefinition {
            ident: input.ident,
            unknown: None,
            normals: vec![],
            next_discriminant: DiscGen::new(&repr)?,
            repr,
        };
        let mut variant_idents = HashSet::new();

        for v in &data.variants {
            let item = r.parse_variant(v)?;
            match item {
                VariantKind::UnknownVariant(var) => {
                    if r.unknown.is_some() {
                        return Err(Error::new(v.span(), "an enum can only have one unknown variant"));
                    }
                    r.unknown = Some(var)
                }
                VariantKind::NormalVariant(var) => {
                    if variant_idents.contains(&var.ident) {
                        return Err(Error::new(v.span(), format!("duplicate variant: {}", var.ident)));
                    }
                    variant_idents.insert(var.ident.clone());
                    r.normals.push(var);
                }
            }
        }

        Ok(r)
    }
    fn parse_variant_unknown(v: &Variant) -> Result<UnknownVariant> {
        Ok(UnknownVariant {
            ident: v.ident.clone(),
        })
    }
    fn parse_variant_normal(&mut self, v: &Variant) -> Result<NormalVariant> {
        let discriminant = match &v.discriminant {
            Some(s) => {
                self.next_discriminant.reset(s.1.clone(), v.span())
            }
            None => {
                self.next_discriminant.next(v.span())
            }
        };

        Ok(NormalVariant {
            ident: v.ident.clone(),
            discriminant,
        })
    }
    fn parse_variant(&mut self, v: &Variant) -> Result<VariantKind> {
        for attr in &v.attrs {
            if !attr.path.is_ident("zhi_enum") {
                continue;
            }
            let args = attr.parse_args::<FieldAttributeArgs>()?;
            if args.is_unknown {
                return Ok(VariantKind::UnknownVariant(Self::parse_variant_unknown(v)?));
            }
        }
        Ok(VariantKind::NormalVariant(self.parse_variant_normal(v)?))
    }
}

impl Parse for EnumDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        Self::new(input.parse()?)
    }
}

#[proc_macro_derive(EnumConvert, attributes(zhi_enum))]
pub fn derive_enum_convert(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_info = match EnumDefinition::new(input) {
        Ok(info) => { info }
        Err(err) => { return err.to_compile_error().into(); }
    };


    let repr = enum_info.repr;
    let ident = enum_info.ident;

    let mut compute_values = Vec::new();
    let mut n_cv = 0;
    let mut normal_variants_from = Vec::new();
    let mut normal_variants_into = Vec::new();
    for variant in enum_info.normals {
        let mut disc_expr = variant.discriminant.to_expr().to_token_stream();
        let ident = variant.ident;

        if variant.discriminant.have_base() {
            let cv_name = format!("ZHI_ENUM_COMPUTED_{}", n_cv);
            let ident = Ident::new(cv_name.as_str(), variant.discriminant.span());
            n_cv += 1;
            compute_values.push(quote! {
                const #ident:#repr = #disc_expr;
            });
            disc_expr = ident.to_token_stream();
        }

        normal_variants_from.push(quote! {
            #disc_expr => Self::#ident,
        });
        normal_variants_into.push(quote! {
            Self::#ident => #disc_expr,
        })
    }

    let unknown_variant_from;
    let unknown_variant_into;

    match enum_info.unknown {
        None => {
            unknown_variant_from = quote! { _ => panic!("unknown variant"),};
            unknown_variant_into = quote! { _ => panic!("unknown discriminant"),};
        }
        Some(variant) => {
            let ident = variant.ident;

            unknown_variant_from = quote! { v => Self::#ident(v),};
            unknown_variant_into = quote! { Self::#ident(v) => v,};
        }
    };

    let into_repr = Ident::new(format!("into_{}", repr.to_string()).as_str(), repr.span());
    let expanded = quote! {
        impl ::core::convert::From<#repr> for #ident {
            fn from(number: #repr) -> Self {
                #(#compute_values)*
                match number {
                    #(#normal_variants_from)*
                    #unknown_variant_from
                }
            }
        }

        impl ::core::convert::Into<#repr> for #ident {
            fn into(self) -> #repr {
                #(#compute_values)*
                match self {
                    #(#normal_variants_into)*
                    #unknown_variant_into
                }
            }
        }

        impl #ident {
            fn #into_repr(self) -> #repr {
                ::core::convert::Into::<#repr>::into(self)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(EnumTryConvert, attributes(zhi_enum))]
pub fn derive_enum_try_convert(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_info = match EnumDefinition::new(input) {
        Ok(info) => { info }
        Err(err) => { return err.to_compile_error().into(); }
    };


    let repr = enum_info.repr;
    let ident = enum_info.ident;

    let mut compute_values = Vec::new();
    let mut n_cv = 0;
    let mut normal_variants_from = Vec::new();
    let mut normal_variants_into = Vec::new();
    for variant in enum_info.normals {
        let mut disc_expr = variant.discriminant.to_expr().to_token_stream();
        let ident = variant.ident;

        if variant.discriminant.have_base() {
            let cv_name = format!("ZHI_ENUM_COMPUTED_{}", n_cv);
            let ident = Ident::new(cv_name.as_str(), variant.discriminant.span());
            n_cv += 1;
            compute_values.push(quote! {
                const #ident:#repr = #disc_expr;
            });
            disc_expr = ident.to_token_stream();
        }

        normal_variants_from.push(quote! {
            #disc_expr => Ok(Self::#ident),
        });
        normal_variants_into.push(quote! {
            Self::#ident => Ok(#disc_expr),
        })
    }

    let unknown_variant_try_from;
    let unknown_variant_try_into;

    match enum_info.unknown {
        None => {
            unknown_variant_try_from = quote! { v => Err(::zhi_enum::UnknownVariantError{}),};
            unknown_variant_try_into = quote! { _ => Err(::zhi_enum::UnknownVariantError{}),};
        }
        Some(variant) => {
            let ident = variant.ident;

            unknown_variant_try_from = quote! { v => Ok(Self::#ident(v)),};
            unknown_variant_try_into = quote! { Self::#ident(v) => Ok(v),};
        }
    };

    let try_into_repr = Ident::new(format!("try_into_{}", repr.to_string()).as_str(), repr.span());
    let expanded = quote! {
        impl ::core::convert::TryFrom<#repr> for #ident {
            type Error = zhi_enum::UnknownVariantError;
            fn try_from(v: #repr) -> ::core::result::Result<Self, Self::Error> {
                #(#compute_values)*
                match v {
                    #(#normal_variants_from)*
                    #unknown_variant_try_from
                }
            }
        }

        impl ::core::convert::TryInto<#repr> for #ident {
            type Error = ::zhi_enum::UnknownVariantError;
            fn try_into(self) -> ::core::result::Result<#repr, Self::Error> {
                #(#compute_values)*
                match self {
                    #(#normal_variants_into)*
                    #unknown_variant_try_into
                }
            }
        }

        impl #ident {
            fn #try_into_repr(self) -> ::core::result::Result<#repr, ::zhi_enum::UnknownVariantError> {
               <#ident as ::core::convert::TryInto::<#repr>>::try_into(self)
            }
        }
    };

    TokenStream::from(expanded)
}
