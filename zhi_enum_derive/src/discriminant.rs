use quote::__private::Span;
use syn::{Error, Expr, Ident,
          Lit, LitInt,
          parse_quote, Result};

#[derive(Debug)]
pub(crate) struct Discriminant {
    span: Span,
    repr: String,
    v: i64,
    base: Option<Expr>,
}

impl Discriminant {
    pub(crate) fn span(&self) -> Span {
        self.span
    }
    pub(crate) fn have_base(&self) -> bool {
        self.base.is_some()
    }
    pub(crate) fn to_expr(&self) -> Expr {
        let s = format!("{}_{}", self.v, self.repr);
        let literal = LitInt::new(s.as_str(), Span::call_site());

        match &self.base {
            None => {
                parse_quote! { #literal }
            }
            Some(base) => {
                if self.v == 0 {
                    return base.clone()
                } else {
                    parse_quote! { #base + #literal }
                }
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Generator {
    repr: String,
    computable: bool,
    base: Option<Expr>,
    v: i64,
}

impl Generator {
    pub(crate) fn new(ident: &Ident) -> Result<Generator> {
        let computable = match &ident.to_string()[..] {
            "i8" | "i16" | "i32" | "i64" => true,
            "u8" | "u16" | "u32" => true,
            "i128" | "u64" | "u128" => false,
            "isize" | "usize" => false,
            "C" => {
                return Err(Error::new(ident.span().into(), "repr(C) is currently not supported"));
            }
            _ => {
                return Err(Error::new(ident.span().into(), "unexpected repr data type for enum"));
            }
        };
        Ok(Generator {
            repr: ident.to_string(),
            computable,
            base: None,
            v: 0,
        })
    }

    pub(crate) fn next(&mut self, span: Span) -> Discriminant {
        let v = self.v;
        self.v += 1;
        Discriminant {
            span,
            repr: self.repr.clone(),
            v,
            base: self.base.clone(),
        }
    }

    pub(crate) fn reset(&mut self, base: Expr, span: Span) -> Discriminant {
        if self.computable {
            if let Expr::Lit(el) = &base {
                if let Lit::Int(int) = &el.lit {
                    if let Ok(v) = int.base10_parse::<i64>() {
                        self.base = None;
                        self.v = v + 1;
                        return Discriminant {
                            span,
                            repr: self.repr.clone(),
                            v,
                            base: None,
                        };
                    }
                }
            }
        }
        self.base = Some(base);
        self.v = 1;

        Discriminant {
            span,
            repr: self.repr.clone(),
            v: 0,
            base: self.base.clone(),
        }
    }
}