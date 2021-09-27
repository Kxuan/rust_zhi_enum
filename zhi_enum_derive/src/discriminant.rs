use syn::{Error, Expr, Ident,
          LitInt, parse_quote,
          Result};
use quote::__private::Span;

#[derive(Debug)]
pub(crate) struct Discriminant {
    repr: String,
    base: Option<Expr>,
    v: i64,
}

impl Discriminant {
    pub(crate) fn new(ident: &Ident) -> Result<Discriminant> {
        let (signed, size) = match &ident.to_string()[..] {
            "i8" => (true, 8u32),
            "i16" => (true, 16u32),
            "i32" => (true, 32u32),
            "i64" => (true, 64u32),
            "i128" => (true, 128u32),
            "u8" => (true, 8u32),
            "u16" => (true, 16u32),
            "u32" => (true, 32u32),
            "u64" => (true, 64u32),
            "u128" => (true, 128u32),
            "C" => {
                return Err(Error::new(ident.span().into(), "repr(C) is currently not supported"));
            }
            _ => {
                return Err(Error::new(ident.span().into(), "unexpected repr data type for enum"));
            }
        };
        Ok(Discriminant {
            repr: ident.to_string(),
            base: None,
            v: 0,
        })
    }

    pub(crate) fn next(&mut self, span: Span) -> Expr {
        let s = format!("{}_{}", self.v, self.repr);
        let literal = LitInt::new(s.as_str(), span.clone());
        self.v += 1;

        match &self.base {
            None => {
                parse_quote! {
                    #literal
                }
            }
            Some(expr) => {
                let repr = Ident::new(&self.repr, span.clone());
                let q = parse_quote! {
                    #repr::wrapping_add(#expr, #literal)
                };
                q
            }
        }
    }

    pub(crate) fn reset(&mut self, base: Expr) {
        self.base = Some(base);
        self.v = 1;
    }
}