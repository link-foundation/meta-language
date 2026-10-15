//! Existing external and import records, re-exported through the surface API.
use super::{Span, Type};
use serde::{Deserialize, Serialize};

/// A named import of items of another module of the crate,
/// `import { a, b as c } from './m.mjs'`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SImport {
    /// The imported module's path inside the crate.
    pub module: Vec<String>,
    pub names: Vec<SImportName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

/// An imported item: its name in its module and the name the import binds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SImportName {
    pub imported: String,
    pub local: String,
}

/// Another item of the module, which a program calls or reads by name
/// without declaring it.
///
/// A function comes with its signature, and a constant with its type and
/// whether a literal gives its value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "k")]
pub enum External {
    #[serde(rename = "fn")]
    Function {
        name: String,
        params: Vec<ExternalParam>,
        ret: Type,
    },
    #[serde(rename = "const")]
    Constant {
        name: String,
        #[serde(rename = "type")]
        ty: Type,
        literal: bool,
    },
}

impl External {
    /// The name the program calls or reads the item by.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Function { name, .. } | Self::Constant { name, .. } => name,
        }
    }

    /// The types of a function's parameters and its result, or a constant's type.
    #[must_use]
    pub fn signature(&self) -> (Vec<&Type>, &Type) {
        match self {
            Self::Function { params, ret, .. } => {
                (params.iter().map(|param| &param.ty).collect(), ret)
            }
            Self::Constant { ty, .. } => (Vec::new(), ty),
        }
    }

    /// The item under the name `name`.
    #[must_use]
    pub fn renamed(&self, name: &str) -> Self {
        let mut renamed = self.clone();
        match &mut renamed {
            Self::Function { name: own, .. } | Self::Constant { name: own, .. } => {
                name.clone_into(own);
            }
        }
        renamed
    }
}

/// A parameter of another item's function.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalParam {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
}
