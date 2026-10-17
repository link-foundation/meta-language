//! Internal expected fields keep a callable result absent until its body checks.
//! Public checked types remain complete; this is only checker context transport.

use serde_json::{Value, json};

use super::Type;

#[derive(Clone, Copy)]
pub(super) enum ExpectedType<'a> {
    Concrete(Option<&'a Type>),
    Callable {
        params: &'a [Type],
        ret: Option<&'a Type>,
    },
}

impl<'a> ExpectedType<'a> {
    pub(super) const fn concrete(self) -> Option<&'a Type> {
        match self {
            Self::Concrete(ty) => ty,
            Self::Callable { .. } => None,
        }
    }

    pub(super) fn kind(self) -> &'static str {
        match self {
            Self::Concrete(ty) => ty.map_or("", Type::kind),
            Self::Callable { .. } => "function",
        }
    }

    pub(super) fn result(self) -> Option<&'a Type> {
        match self {
            Self::Concrete(Some(Type::Function { ret, .. })) => Some(ret),
            Self::Callable { ret, .. } => ret,
            Self::Concrete(_) => None,
        }
    }

    pub(super) fn value(self) -> Value {
        match self {
            Self::Concrete(ty) => json!(ty),
            Self::Callable { params, ret } => {
                let mut value = json!({"kind":"function", "params":params});
                if let Some(ret) = ret {
                    value["ret"] = json!(ret);
                }
                value
            }
        }
    }
}
