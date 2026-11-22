//! Authenticate callback state against the enclosing source bindings.

use super::{
    Result, RustParser, Span, owned_record_effects::EffectCount, owned_record_state::Slot,
    unsupported,
};
use crate::translation::source_loop_folds::SourceStateBinder;
use crate::translation::surface::SParam;
use std::collections::HashSet;

pub(super) struct CaptureRead {
    roots: Vec<Slot>,
    entered: bool,
    pub(super) captures: Option<Vec<SourceCapture>>,
}

#[derive(Clone)]
pub(super) struct SourceCapture {
    pub(super) outer: Slot,
    pub(super) alias: Slot,
    pub(super) binder: Option<SourceStateBinder>,
}

pub(super) struct CaptureFrame {
    token: usize,
    aliases: Vec<(Slot, Slot)>,
    before: EffectCount,
}

fn fail(reason: &str, span: Option<Span>) -> super::TranslationError {
    unsupported("Rust source FnMut environment", reason, span)
}

impl RustParser {
    pub(super) fn callback_state_parameter(
        &self,
        slot: &Slot,
        span: Option<Span>,
    ) -> Result<SParam> {
        let ty = slot.origin.ty.as_ref().ok_or_else(|| {
            fail(
                "callback state needs its complete original source type",
                span,
            )
        })?;
        Ok(SParam {
            name: slot.current_name.clone(),
            ty: Some(self.record_source_type(ty, &slot.origin.path)),
            optional: false,
            default_value: None,
            guard: None,
            rocq_type: None,
            span,
        })
    }

    pub(super) fn begin_fn_mut_source_function_state(&mut self) -> Result<Option<usize>> {
        let Some(header) = self
            .fn_mut_source_header_read
            .as_ref()
            .filter(|read| read.body_replay)
        else {
            return Ok(None);
        };
        let roots = header
            .captures
            .iter()
            .map(|capture| capture.outer.clone())
            .collect::<Vec<_>>();
        if roots.is_empty() {
            return Ok(None);
        }
        let token = self.begin_owned_record_state_block(true);
        let registered = (|| {
            let parameters = self.enter_callback_state_aliases(&roots, token)?;
            for slot in &parameters {
                self.retain_callable_state_parameter(
                    slot.current_name.clone(),
                    slot.clone(),
                    true,
                )?;
            }
            self.fn_mut_source_header_read
                .as_mut()
                .expect("actual selected function header")
                .state_parameters = parameters;
            Ok(())
        })();
        if let Err(error) = registered {
            self.end_owned_record_state_block(token)?;
            return Err(error);
        }
        Ok(Some(token))
    }

    pub(super) fn begin_fn_mut_source_capture_read(&mut self) -> Result<()> {
        let roots = self.record_state_active_slots();
        if roots.len() > 256 || self.fn_mut_source_capture_read.is_some() {
            return Err(fail(
                "a selected callback needs its bounded enclosing source environment",
                None,
            ));
        }
        self.fn_mut_source_capture_read = Some(CaptureRead {
            roots,
            entered: false,
            captures: None,
        });
        Ok(())
    }

    pub(super) fn enter_fn_mut_source_capture(
        &mut self,
        span: Option<Span>,
    ) -> Result<Option<CaptureFrame>> {
        let Some(read) = &mut self.fn_mut_source_capture_read else {
            return Ok(None);
        };
        if read.entered {
            return Err(fail(
                "a nested callback needs its own source specialization",
                span,
            ));
        }
        read.entered = true;
        let roots = read.roots.clone();
        let before = self
            .owned_record_state
            .effects
            .get_or_insert_with(Default::default)
            .count();
        let token = self.begin_owned_record_state_block(false);
        let aliases = match self.enter_callback_state_aliases(&roots, token) {
            Ok(aliases) => aliases,
            Err(error) => {
                self.end_owned_record_state_block(token)?;
                return Err(error);
            }
        };
        Ok(Some(CaptureFrame {
            token,
            aliases: roots.into_iter().zip(aliases).collect(),
            before,
        }))
    }

    pub(super) fn leave_fn_mut_source_capture(
        &mut self,
        frame: Option<CaptureFrame>,
        success: bool,
        span: Option<Span>,
    ) -> Result<()> {
        let Some(frame) = frame else {
            return Ok(());
        };
        self.end_owned_record_state_block(frame.token)?;
        if !success {
            return Ok(());
        }
        let effects = self
            .owned_record_state
            .effects
            .as_ref()
            .expect("an entered callback owns its effect inventory");
        let names: HashSet<_> = effects
            .slots_since(frame.before)
            .map(|slot| slot.current_name.as_str())
            .collect();
        let mut captures = Vec::new();
        for (outer, alias) in frame.aliases {
            if !names.contains(alias.current_name.as_str()) {
                continue;
            }
            self.check_callback_state_root(&outer, span)?;
            let binder = if self.origins.collecting {
                None
            } else {
                let (name, location) = outer.callback_binder.as_ref().ok_or_else(|| {
                    fail(
                        "a mutable callback capture needs its actual source binder",
                        span,
                    )
                })?;
                Some(self.retain_source_state_binder(name, &outer.lexical, *location)?)
            };
            captures.push(SourceCapture {
                outer,
                alias,
                binder,
            });
        }
        self.fn_mut_source_capture_read
            .as_mut()
            .expect("an entered callback retains its reader")
            .captures = Some(captures);
        Ok(())
    }
}
