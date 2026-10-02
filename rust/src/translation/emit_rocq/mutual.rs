//! Mutually recursive functions as one `ml_fix` over a sum.

use super::*;

/// The group being emitted: each member's position, and its inhabitant for
/// the unreachable arms of a projection.
pub(super) struct Mutual {
    indices: HashMap<String, usize>,
    inhabitants: Vec<String>,
}

/// A member of the group, renamed and with its Rocq types.
struct Member<'f> {
    function: &'f FnDecl,
    name: String,
    params: Vec<Param>,
    body: Expr,
    names: Vec<String>,
    domain: String,
    result: String,
    inhabitant: String,
}

/// The injection of the `index`th of `count` types into their sum `A + (B + (C + …))`.
fn injection(index: usize, count: usize, text: &str) -> String {
    let mut wrapped = if index < count - 1 {
        format!("(inl {text})")
    } else {
        text.to_owned()
    };
    for _ in 0..index {
        wrapped = format!("(inr {wrapped})");
    }
    wrapped
}

/// A tuple of arguments or parameters, `tt` when there are none.
fn tuple(parts: &[String]) -> String {
    match parts {
        [] => "tt".to_owned(),
        [only] => only.clone(),
        _ => format!("({})", parts.join(", ")),
    }
}

/// `A + (B + C)`, each part parenthesised by its maker where it needs it.
fn sum(parts: &[String]) -> String {
    let mut rest = parts[parts.len() - 1].clone();
    for part in parts[..parts.len() - 1].iter().rev() {
        rest = format!("({part} + {rest})");
    }
    rest
}

impl RocqEmitter<'_> {
    /// Mutually recursive functions are one `ml_fix` over the sum of their
    /// parameter tuples, returning the sum of their results: the case of a
    /// function is its injection, a call of any of them is a call of `ml_rec`
    /// projected back, and each function is its projection of the whole.
    pub(super) fn mutual(&mut self, group: &[&Decl]) -> Result<String> {
        self.helpers.insert(Helper::Fix);
        self.state.encode(
            "mutual-recursion",
            "mutually recursive functions are one ml_fix over the sum of their parameter tuples, which unfolds lazily to 2^64 nested calls: every run that terminates computes the same value, and Rocq accepts each function as a plain Definition, its projection of the whole",
        );
        let mut members = Vec::new();
        for entry in group {
            let Decl::Fn(function) = entry else {
                continue;
            };
            let (params, body) = rename_function(function, &ident, &self.state.local_reserved());
            let name = self.state.local_name(&function.full_name).to_owned();
            self.state.map(entry, &name);
            let mut types = Vec::new();
            for param in &params {
                types.push(self.ty(&param.ty)?);
            }
            let domain = match types.as_slice() {
                [] => "unit".to_owned(),
                [only] => only.clone(),
                _ => format!("({})", types.join(" * ")),
            };
            let names: Vec<String> = params.iter().map(|param| param.name.clone()).collect();
            members.push(Member {
                function,
                name,
                params,
                body,
                names,
                domain,
                result: self.ty(&function.ret)?,
                inhabitant: self.inhabitant(&function.ret)?,
            });
        }
        let count = members.len();
        let whole = format!("ml_mutual_{}", members[0].name);
        let domain = sum(&members
            .iter()
            .map(|member| member.domain.clone())
            .collect::<Vec<_>>());
        let result = sum(&members
            .iter()
            .map(|member| member.result.clone())
            .collect::<Vec<_>>());
        let mutual = Rc::new(Mutual {
            indices: members
                .iter()
                .enumerate()
                .map(|(index, member)| (member.function.full_name.clone(), index))
                .collect(),
            inhabitants: members
                .iter()
                .map(|member| member.inhabitant.clone())
                .collect(),
        });
        let mut cases = Vec::new();
        for (index, member) in members.iter().enumerate() {
            self.current = Some(Current {
                full_name: member.function.full_name.clone(),
                name: member.name.clone(),
                recursive: false,
                general: false,
                fuel: false,
                mutual: Some(Rc::clone(&mutual)),
            });
            let text = self.expr(&member.body)?;
            self.current = None;
            let pattern = if member.names.is_empty() {
                "_".to_owned()
            } else {
                tuple(&member.names)
            };
            cases.push(format!(
                "    | {} => {}",
                injection(index, count, &pattern),
                injection(index, count, &format!("({text})"))
            ));
        }
        let fallback: Vec<String> = members
            .iter()
            .enumerate()
            .map(|(index, member)| {
                format!(
                    "{} => {}",
                    injection(index, count, "_"),
                    injection(index, count, &member.inhabitant)
                )
            })
            .collect();
        let mut blocks = vec![format!(
            "Definition {whole} : {domain} -> {result} :=\n  ml_fix 64 (fun (ml_rec : {domain} -> {result}) (ml_args : {domain}) =>\n    match ml_args with\n{}\n    end)\n    (fun ml_args => match ml_args with {} end).",
            cases.join("\n"),
            fallback.join(" | ")
        )];
        for (index, member) in members.iter().enumerate() {
            let mut binders = String::new();
            for param in &member.params {
                let ty = self.ty(&param.ty)?;
                let _ = write!(binders, " ({} : {ty})", param.name);
            }
            blocks.push(format!(
                "Definition {}{binders} : {} :=\n  match {whole} {} with {} => ml_r | _ => {} end.",
                member.name,
                member.result,
                injection(index, count, &tuple(&member.names)),
                injection(index, count, "ml_r"),
                member.inhabitant
            ));
        }
        Ok(blocks.join("\n\n"))
    }
}

impl Mutual {
    pub(super) fn index(&self, name: &str) -> Option<usize> {
        self.indices.get(name).copied()
    }

    /// A call of the `index`th member: `ml_rec` on its injection, projected back.
    pub(super) fn call(&self, index: usize, parts: &[String]) -> String {
        let count = self.inhabitants.len();
        format!(
            "(match ml_rec {} with {} => ml_r | _ => {} end)",
            injection(index, count, &tuple(parts)),
            injection(index, count, "ml_r"),
            self.inhabitants[index]
        )
    }
}
