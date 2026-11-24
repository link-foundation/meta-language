// Generated from js/src/translation/frontend-rules.js by
// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.
use super::*;

pub fn source_ml_source1_temporary_regular_expression_end_loop11(
    mut units: Vec<f64>,
    mut index: f64,
    mut escaped: bool,
    mut character_class: bool,
) -> crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndLoop11Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let unit = crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if ((({
                    let source_ml_source2_comparison_operand187 = unit.clone();
                    {
                        let source_ml_source2_comparison_operand188 = 10f64;
                        match (
                            source_ml_source2_comparison_operand187.clone(),
                            source_ml_source2_comparison_operand188,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } || {
                    let source_ml_source2_comparison_operand189 = unit.clone();
                    {
                        let source_ml_source2_comparison_operand190 = 13f64;
                        match (
                            source_ml_source2_comparison_operand189.clone(),
                            source_ml_source2_comparison_operand190,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                }) || {
                    let source_ml_source2_comparison_operand191 = unit.clone();
                    {
                        let source_ml_source2_comparison_operand192 = 8232f64;
                        match (
                            source_ml_source2_comparison_operand191.clone(),
                            source_ml_source2_comparison_operand192,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                }) || {
                    let source_ml_source2_comparison_operand193 = unit.clone();
                    {
                        let source_ml_source2_comparison_operand194 = 8233f64;
                        match (
                            source_ml_source2_comparison_operand193.clone(),
                            source_ml_source2_comparison_operand194,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                }) {
                    crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndLoop11Result::MlSource1TemporaryRegularExpressionEndLoop11Return((-1f64))
                } else {
                    {
                        let source_ml_source2_s203 = if escaped {
                            {
                                let escaped_2 = false;
                                crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndJoin12::MlSource1TemporaryRegularExpressionEndJoin12Next(index, escaped_2, character_class)
                            }
                        } else {
                            if {
                                let source_ml_source2_comparison_operand204 = unit.clone();
                                {
                                    let source_ml_source2_comparison_operand205 = 92f64;
                                    match (
                                        source_ml_source2_comparison_operand204.clone(),
                                        source_ml_source2_comparison_operand205,
                                    ) {
                                        (Some(ml_left), ml_right) => ml_left == ml_right,
                                        _ => false,
                                    }
                                }
                            } {
                                {
                                    let escaped_3 = true;
                                    crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndJoin12::MlSource1TemporaryRegularExpressionEndJoin12Next(index, escaped_3, character_class)
                                }
                            } else {
                                if {
                                    let source_ml_source2_comparison_operand206 = unit.clone();
                                    {
                                        let source_ml_source2_comparison_operand207 = 91f64;
                                        match (
                                            source_ml_source2_comparison_operand206.clone(),
                                            source_ml_source2_comparison_operand207,
                                        ) {
                                            (Some(ml_left), ml_right) => ml_left == ml_right,
                                            _ => false,
                                        }
                                    }
                                } {
                                    {
                                        let character_class_2 = true;
                                        crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndJoin12::MlSource1TemporaryRegularExpressionEndJoin12Next(index, escaped, character_class_2)
                                    }
                                } else {
                                    if {
                                        let source_ml_source2_comparison_operand208 = unit.clone();
                                        {
                                            let source_ml_source2_comparison_operand209 = 93f64;
                                            match (
                                                source_ml_source2_comparison_operand208.clone(),
                                                source_ml_source2_comparison_operand209,
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            }
                                        }
                                    } {
                                        {
                                            let character_class_3 = false;
                                            crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndJoin12::MlSource1TemporaryRegularExpressionEndJoin12Next(index, escaped, character_class_3)
                                        }
                                    } else {
                                        if ({
                                            let source_ml_source2_comparison_operand210 =
                                                unit.clone();
                                            {
                                                let source_ml_source2_comparison_operand211 = 47f64;
                                                match (
                                                    source_ml_source2_comparison_operand210.clone(),
                                                    source_ml_source2_comparison_operand211,
                                                ) {
                                                    (Some(ml_left), ml_right) => {
                                                        ml_left == ml_right
                                                    }
                                                    _ => false,
                                                }
                                            }
                                        } && !character_class)
                                        {
                                            {
                                                let index_2 = (index + 1f64);
                                                {
                                                    let index_3 = crate::translation::frontend_rules::source_ml_source1_temporary_regular_expression_end_loop13(units.to_vec(), index_2);
                                                    crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndJoin12::MlSource1TemporaryRegularExpressionEndJoin12Return(index_3)
                                                }
                                            }
                                        } else {
                                            crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndJoin12::MlSource1TemporaryRegularExpressionEndJoin12Next(index, escaped, character_class)
                                        }
                                    }
                                }
                            }
                        };
                        match source_ml_source2_s203.clone() {
                            crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndJoin12::MlSource1TemporaryRegularExpressionEndJoin12Next(index_4, escaped_4, character_class_4) => {
                                {
                                    let index_5 = (index_4 + 1f64);
                                    {
                                        (index, escaped, character_class) = (index_5, escaped_4, character_class_4);
                                        continue;
                                    }
                                }
                            }
                            crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndJoin12::MlSource1TemporaryRegularExpressionEndJoin12Return(source_ml_source1_temporary_result12) => {
                                crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndLoop11Result::MlSource1TemporaryRegularExpressionEndLoop11Return(source_ml_source1_temporary_result12)
                            }
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryRegularExpressionEndLoop11Result::MlSource1TemporaryRegularExpressionEndLoop11Done
        };
    }
}

pub fn source_ml_source1_temporary_find_binding_run_end_loop14(
    mut terms: Vec<String>,
    mut end: f64,
) -> f64 {
    loop {
        return if (end < (terms.len() as f64)) {
            {
                let term = crate::translation::frontend_rules::ml_array::optional_at(
                    &terms,
                    crate::translation::frontend_rules::ml_array::number_index(end),
                );
                if (({
                    let source_ml_source2_comparison_operand212 = term.clone();
                    {
                        let source_ml_source2_comparison_operand213 =
                            String::from("function_declaration");
                        !(match (
                            source_ml_source2_comparison_operand212.clone(),
                            source_ml_source2_comparison_operand213.clone(),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                } && {
                    let source_ml_source2_comparison_operand214 = term.clone();
                    {
                        let source_ml_source2_comparison_operand215 =
                            String::from("lexical_declaration");
                        !(match (
                            source_ml_source2_comparison_operand214.clone(),
                            source_ml_source2_comparison_operand215.clone(),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                }) && {
                    let source_ml_source2_comparison_operand216 = term.clone();
                    {
                        let source_ml_source2_comparison_operand217 =
                            String::from("export_statement");
                        !(match (
                            source_ml_source2_comparison_operand216.clone(),
                            source_ml_source2_comparison_operand217.clone(),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                }) {
                    end
                } else {
                    {
                        let end_2 = (end + 1f64);
                        {
                            end = end_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptBindingScopeLoop15Result {
    MlSource1TemporaryAcceptBindingScopeLoop15Done,
    MlSource1TemporaryAcceptBindingScopeLoop15Return(bool),
}

pub fn source_ml_source1_temporary_accept_binding_scope_loop15(
    mut statuses: Vec<String>,
    mut reasons: Vec<String>,
    mut index: f64,
) -> crate::translation::frontend_rules::MlSource1TemporaryAcceptBindingScopeLoop15Result {
    loop {
        return if (index < (statuses.len() as f64)) {
            if ({
                let source_ml_source2_comparison_operand218 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &statuses,
                        crate::translation::frontend_rules::ml_array::number_index(index),
                    );
                {
                    let source_ml_source2_comparison_operand219 = String::from("carried");
                    match (
                        source_ml_source2_comparison_operand218.clone(),
                        source_ml_source2_comparison_operand219.clone(),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } && {
                let source_ml_source2_comparison_operand220 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &reasons,
                        crate::translation::frontend_rules::ml_array::number_index(index),
                    );
                {
                    let source_ml_source2_comparison_operand221 = String::from("type");
                    match (
                        source_ml_source2_comparison_operand220.clone(),
                        source_ml_source2_comparison_operand221.clone(),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            }) {
                crate::translation::frontend_rules::MlSource1TemporaryAcceptBindingScopeLoop15Result::MlSource1TemporaryAcceptBindingScopeLoop15Return(true)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryAcceptBindingScopeLoop15Result::MlSource1TemporaryAcceptBindingScopeLoop15Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryAcceptModuleBindingScopeLoop16Result {
    MlSource1TemporaryAcceptModuleBindingScopeLoop16Done(f64),
    MlSource1TemporaryAcceptModuleBindingScopeLoop16Return(bool),
}

pub fn source_ml_source1_temporary_accept_module_binding_scope_loop16(
    mut terms: Vec<String>,
    mut declarations: f64,
    mut index: f64,
) -> crate::translation::frontend_rules::MlSource1TemporaryAcceptModuleBindingScopeLoop16Result {
    loop {
        return if (index < (terms.len() as f64)) {
            {
                let term = crate::translation::frontend_rules::ml_array::optional_at(
                    &terms,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                );
                if {
                    let source_ml_source2_comparison_operand222 = term.clone();
                    {
                        let source_ml_source2_comparison_operand223 = String::from("");
                        !(match (
                            source_ml_source2_comparison_operand222.clone(),
                            source_ml_source2_comparison_operand223.clone(),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                } {
                    if (({
                        let source_ml_source2_comparison_operand224 = term.clone();
                        {
                            let source_ml_source2_comparison_operand225 =
                                String::from("function_declaration");
                            !(match (
                                source_ml_source2_comparison_operand224.clone(),
                                source_ml_source2_comparison_operand225.clone(),
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    } && {
                        let source_ml_source2_comparison_operand226 = term.clone();
                        {
                            let source_ml_source2_comparison_operand227 =
                                String::from("lexical_declaration");
                            !(match (
                                source_ml_source2_comparison_operand226.clone(),
                                source_ml_source2_comparison_operand227.clone(),
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    }) && {
                        let source_ml_source2_comparison_operand228 = term.clone();
                        {
                            let source_ml_source2_comparison_operand229 =
                                String::from("export_statement");
                            !(match (
                                source_ml_source2_comparison_operand228.clone(),
                                source_ml_source2_comparison_operand229.clone(),
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    }) {
                        crate::translation::frontend_rules::MlSource1TemporaryAcceptModuleBindingScopeLoop16Result::MlSource1TemporaryAcceptModuleBindingScopeLoop16Return(false)
                    } else {
                        {
                            let declarations_2 = (declarations + 1f64);
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (declarations, index) = (declarations_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            crate::translation::frontend_rules::MlSource1TemporaryAcceptModuleBindingScopeLoop16Result::MlSource1TemporaryAcceptModuleBindingScopeLoop16Done(declarations)
        };
    }
}

pub fn source_ml_source1_temporary_find_documentation_parameter_range_loop17(
    mut units: Vec<f64>,
    mut start: f64,
) -> f64 {
    loop {
        return if ((start < (units.len() as f64))
            && ({
                let source_ml_source2_comparison_operand230 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(start),
                    );
                {
                    let source_ml_source2_comparison_operand231 = 32f64;
                    match (
                        source_ml_source2_comparison_operand230.clone(),
                        source_ml_source2_comparison_operand231,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } || {
                let source_ml_source2_comparison_operand232 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(start),
                    );
                {
                    let source_ml_source2_comparison_operand233 = 9f64;
                    match (
                        source_ml_source2_comparison_operand232.clone(),
                        source_ml_source2_comparison_operand233,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            })) {
            {
                let start_2 = (start + 1f64);
                {
                    start = start_2;
                    continue;
                }
            }
        } else {
            start
        };
    }
}

pub fn source_ml_source1_temporary_find_documentation_parameter_range_loop19(
    mut units: Vec<f64>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        return if (end < (units.len() as f64)) {
            {
                let code = crate::translation::frontend_rules::ml_array::optional_at(
                    &units,
                    crate::translation::frontend_rules::ml_array::number_index(end),
                );
                if ((((({
                    let source_ml_source2_comparison_operand234 = code.clone();
                    {
                        let source_ml_source2_comparison_operand235 = 65f64;
                        if match (source_ml_source2_comparison_operand234.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) >= source_ml_source2_comparison_operand235)
                        } else {
                            ((source_ml_source2_comparison_operand234
                                .clone()
                                .expect("a proven present value"))
                                >= source_ml_source2_comparison_operand235)
                        }
                    }
                } && {
                    let source_ml_source2_comparison_operand236 = code.clone();
                    {
                        let source_ml_source2_comparison_operand237 = 90f64;
                        if match (source_ml_source2_comparison_operand236.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) <= source_ml_source2_comparison_operand237)
                        } else {
                            ((source_ml_source2_comparison_operand236
                                .clone()
                                .expect("a proven present value"))
                                <= source_ml_source2_comparison_operand237)
                        }
                    }
                }) || ({
                    let source_ml_source2_comparison_operand238 = code.clone();
                    {
                        let source_ml_source2_comparison_operand239 = 97f64;
                        if match (source_ml_source2_comparison_operand238.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) >= source_ml_source2_comparison_operand239)
                        } else {
                            ((source_ml_source2_comparison_operand238
                                .clone()
                                .expect("a proven present value"))
                                >= source_ml_source2_comparison_operand239)
                        }
                    }
                } && {
                    let source_ml_source2_comparison_operand240 = code.clone();
                    {
                        let source_ml_source2_comparison_operand241 = 122f64;
                        if match (source_ml_source2_comparison_operand240.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) <= source_ml_source2_comparison_operand241)
                        } else {
                            ((source_ml_source2_comparison_operand240
                                .clone()
                                .expect("a proven present value"))
                                <= source_ml_source2_comparison_operand241)
                        }
                    }
                })) || {
                    let source_ml_source2_comparison_operand242 = code.clone();
                    {
                        let source_ml_source2_comparison_operand243 = 95f64;
                        match (
                            source_ml_source2_comparison_operand242.clone(),
                            source_ml_source2_comparison_operand243,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                }) || {
                    let source_ml_source2_comparison_operand244 = code.clone();
                    {
                        let source_ml_source2_comparison_operand245 = 36f64;
                        match (
                            source_ml_source2_comparison_operand244.clone(),
                            source_ml_source2_comparison_operand245,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                }) || (((end > start) && {
                    let source_ml_source2_comparison_operand246 = code.clone();
                    {
                        let source_ml_source2_comparison_operand247 = 48f64;
                        if match (source_ml_source2_comparison_operand246.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) >= source_ml_source2_comparison_operand247)
                        } else {
                            ((source_ml_source2_comparison_operand246
                                .clone()
                                .expect("a proven present value"))
                                >= source_ml_source2_comparison_operand247)
                        }
                    }
                }) && {
                    let source_ml_source2_comparison_operand248 = code.clone();
                    {
                        let source_ml_source2_comparison_operand249 = 57f64;
                        if match (source_ml_source2_comparison_operand248.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            ((f64::NAN) <= source_ml_source2_comparison_operand249)
                        } else {
                            ((source_ml_source2_comparison_operand248
                                .clone()
                                .expect("a proven present value"))
                                <= source_ml_source2_comparison_operand249)
                        }
                    }
                })) {
                    {
                        let end_2 = (end + 1f64);
                        {
                            end = end_2;
                            continue;
                        }
                    }
                }
                end
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindDocumentationParameterRangeJoin20 {
    MlSource1TemporaryFindDocumentationParameterRangeJoin20Next,
    MlSource1TemporaryFindDocumentationParameterRangeJoin20Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_find_documentation_parameter_range_loop21(
    mut units: Vec<f64>,
    mut after: f64,
) -> f64 {
    loop {
        return if ((after < (units.len() as f64))
            && ({
                let source_ml_source2_comparison_operand250 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(after),
                    );
                {
                    let source_ml_source2_comparison_operand251 = 32f64;
                    match (
                        source_ml_source2_comparison_operand250.clone(),
                        source_ml_source2_comparison_operand251,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } || {
                let source_ml_source2_comparison_operand252 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(after),
                    );
                {
                    let source_ml_source2_comparison_operand253 = 9f64;
                    match (
                        source_ml_source2_comparison_operand252.clone(),
                        source_ml_source2_comparison_operand253,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            })) {
            {
                let after_2 = (after + 1f64);
                {
                    after = after_2;
                    continue;
                }
            }
        } else {
            after
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindDocumentationParameterRangeJoin18 {
    MlSource1TemporaryFindDocumentationParameterRangeJoin18Next(f64, bool),
}

pub fn source_ml_source1_temporary_read_rust_number_bounds_loop22(
    mut units: Vec<f64>,
    mut at: f64,
) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && (({
                let source_ml_source2_comparison_operand254 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand255 = 48f64;
                    if match (source_ml_source2_comparison_operand254.clone(), None::<f64>) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    } {
                        ((f64::NAN) >= source_ml_source2_comparison_operand255)
                    } else {
                        ((source_ml_source2_comparison_operand254
                            .clone()
                            .expect("a proven present value"))
                            >= source_ml_source2_comparison_operand255)
                    }
                }
            } && {
                let source_ml_source2_comparison_operand256 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand257 = 57f64;
                    if match (source_ml_source2_comparison_operand256.clone(), None::<f64>) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    } {
                        ((f64::NAN) <= source_ml_source2_comparison_operand257)
                    } else {
                        ((source_ml_source2_comparison_operand256
                            .clone()
                            .expect("a proven present value"))
                            <= source_ml_source2_comparison_operand257)
                    }
                }
            }) || {
                let source_ml_source2_comparison_operand258 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand259 = 95f64;
                    match (
                        source_ml_source2_comparison_operand258.clone(),
                        source_ml_source2_comparison_operand259,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            })) {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

pub fn source_ml_source1_temporary_read_rust_number_bounds_loop25(
    mut units: Vec<f64>,
    mut at: f64,
) -> f64 {
    loop {
        return if ((at < (units.len() as f64))
            && (({
                let source_ml_source2_comparison_operand260 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand261 = 97f64;
                    if match (source_ml_source2_comparison_operand260.clone(), None::<f64>) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    } {
                        ((f64::NAN) >= source_ml_source2_comparison_operand261)
                    } else {
                        ((source_ml_source2_comparison_operand260
                            .clone()
                            .expect("a proven present value"))
                            >= source_ml_source2_comparison_operand261)
                    }
                }
            } && {
                let source_ml_source2_comparison_operand262 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand263 = 122f64;
                    if match (source_ml_source2_comparison_operand262.clone(), None::<f64>) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    } {
                        ((f64::NAN) <= source_ml_source2_comparison_operand263)
                    } else {
                        ((source_ml_source2_comparison_operand262
                            .clone()
                            .expect("a proven present value"))
                            <= source_ml_source2_comparison_operand263)
                    }
                }
            }) || ({
                let source_ml_source2_comparison_operand264 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand265 = 48f64;
                    if match (source_ml_source2_comparison_operand264.clone(), None::<f64>) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    } {
                        ((f64::NAN) >= source_ml_source2_comparison_operand265)
                    } else {
                        ((source_ml_source2_comparison_operand264
                            .clone()
                            .expect("a proven present value"))
                            >= source_ml_source2_comparison_operand265)
                    }
                }
            } && {
                let source_ml_source2_comparison_operand266 =
                    crate::translation::frontend_rules::ml_array::optional_at(
                        &units,
                        crate::translation::frontend_rules::ml_array::number_index(at),
                    );
                {
                    let source_ml_source2_comparison_operand267 = 57f64;
                    if match (source_ml_source2_comparison_operand266.clone(), None::<f64>) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    } {
                        ((f64::NAN) <= source_ml_source2_comparison_operand267)
                    } else {
                        ((source_ml_source2_comparison_operand266
                            .clone()
                            .expect("a proven present value"))
                            <= source_ml_source2_comparison_operand267)
                    }
                }
            }))) {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustNumberBoundsJoin24 {
    MlSource1TemporaryReadRustNumberBoundsJoin24Next(f64, bool),
}
