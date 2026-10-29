// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result;
use super::lower_java_script_call_source_branch;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_constant_error;
use super::read_java_script_call_primitive_arity_error;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop102;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103;
use super::source_ml_source1_temporary_lower_java_script_call_source_expression_loop104;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_call_source(builder: self::JavaScriptCallSourceBuilder, error: &self::ml_string::Text) -> self::JavaScriptCallSourceResult {
    match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_field319 = shared_constructor_field1;
                {
                    let source_ml_source2_nominal_field321 = shared_constructor_field2;
                    {
                        let source_ml_source2_nominal_field323 = shared_constructor_field3;
                        {
                            let source_ml_source2_nominal_field325 = shared_constructor_field4;
                            {
                                let source_ml_source2_nominal_field327 = shared_constructor_field5;
                                {
                                    let source_ml_source2_nominal_field329 = shared_constructor_field6.clone();
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field319, source_ml_source2_nominal_field321, source_ml_source2_nominal_field323, source_ml_source2_nominal_field325, source_ml_source2_nominal_field327, error.clone())
                                }
                            }
                        }
                    }
                }
            }), 0f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_call_source_instruction(builder: self::JavaScriptCallSourceBuilder, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallSourceBuilder {
    match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
            if (shared_constructor_field6 != (self::ml_string::Text::from_units(vec![]))) {
                builder.clone()
            } else {
                if (((shared_constructor_field1.len() as f64)) >= (65536f64)) {
                    {
                        let source_ml_source2_nominal_field337 = shared_constructor_field1;
                        {
                            let source_ml_source2_nominal_field339 = shared_constructor_field2;
                            {
                                let source_ml_source2_nominal_field341 = shared_constructor_field3;
                                {
                                    let source_ml_source2_nominal_field343 = shared_constructor_field4;
                                    {
                                        let source_ml_source2_nominal_field345 = shared_constructor_field5;
                                        {
                                            let source_ml_source2_nominal_field347 = shared_constructor_field6.clone();
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field337, source_ml_source2_nominal_field339, source_ml_source2_nominal_field341, source_ml_source2_nominal_field343, source_ml_source2_nominal_field345, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116]))
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source2_nominal_field354 = shared_constructor_field1.clone();
                        {
                            let source_ml_source2_nominal_field356 = shared_constructor_field2;
                            {
                                let source_ml_source2_nominal_field358 = shared_constructor_field3;
                                {
                                    let source_ml_source2_nominal_field360 = shared_constructor_field4;
                                    {
                                        let source_ml_source2_nominal_field362 = shared_constructor_field5;
                                        {
                                            let source_ml_source2_nominal_field364 = shared_constructor_field6.clone();
                                            {
                                                let source_ml_source2_nominal_override366 = {
                                                    let source_ml_source2_concat_value351 = shared_constructor_field1;
                                                    {
                                                        let source_ml_source2_concat_value352 = vec![instruction.clone()];
                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), source_ml_source2_concat_value351), source_ml_source2_concat_value352)
                                                    }
                                                };
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override366, source_ml_source2_nominal_field356, source_ml_source2_nominal_field358, source_ml_source2_nominal_field360, source_ml_source2_nominal_field362, source_ml_source2_nominal_field364.clone())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_call_source_register(builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
            if (shared_constructor_field6 != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
            } else {
                if (shared_constructor_field2 >= (4096f64)) {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 98, 117, 100, 103, 101, 116])))
                } else {
                    self::JavaScriptCallSourceResult::SourceResult(Box::new({
                        let source_ml_source2_nominal_field373 = shared_constructor_field1;
                        {
                            let source_ml_source2_nominal_field375 = shared_constructor_field2;
                            {
                                let source_ml_source2_nominal_field377 = shared_constructor_field3;
                                {
                                    let source_ml_source2_nominal_field379 = shared_constructor_field4;
                                    {
                                        let source_ml_source2_nominal_field381 = shared_constructor_field5;
                                        {
                                            let source_ml_source2_nominal_field383 = shared_constructor_field6.clone();
                                            {
                                                let source_ml_source2_nominal_override385 = (shared_constructor_field2 + 1f64);
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field373, source_ml_source2_nominal_override385, source_ml_source2_nominal_field377, source_ml_source2_nominal_field379, source_ml_source2_nominal_field381, source_ml_source2_nominal_field383.clone())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }), shared_constructor_field2)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14, shared_constructor_field15, shared_constructor_field16) => {
            if (shared_constructor_field16 != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
            } else {
                if ((depth > (128f64)) || (shared_constructor_field13 >= (65536f64))) {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 32, 98, 117, 100, 103, 101, 116])))
                } else {
                    {
                        let state = {
                            let source_ml_source2_nominal_field393 = shared_constructor_field11;
                            {
                                let source_ml_source2_nominal_field395 = shared_constructor_field12;
                                {
                                    let source_ml_source2_nominal_field397 = shared_constructor_field13;
                                    {
                                        let source_ml_source2_nominal_field399 = shared_constructor_field14;
                                        {
                                            let source_ml_source2_nominal_field401 = shared_constructor_field15;
                                            {
                                                let source_ml_source2_nominal_field403 = shared_constructor_field16.clone();
                                                {
                                                    let source_ml_source2_nominal_override405 = (shared_constructor_field13 + 1f64);
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field393, source_ml_source2_nominal_field395, source_ml_source2_nominal_override405, source_ml_source2_nominal_field399, source_ml_source2_nominal_field401, source_ml_source2_nominal_field403.clone())
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        match state.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, shared_constructor_field9, shared_constructor_field10, _) => {
                                match source.clone() {
                                    self::JavaScriptCallSource::SourceConstant(value) => {
                                        let value = *value;
                                        {
                                            let error = self::read_java_script_call_constant_error(value.clone());
                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                self::refuse_java_script_call_source(state.clone(), &(error))
                                            } else {
                                                {
                                                    let result = self::allocate_java_script_call_source_register(state.clone());
                                                    match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1, shared_constructor_field2) => {
                                                            let shared_constructor_field1 = *shared_constructor_field1;
                                                            {
                                                                let source_ml_source2_nominal_field416 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field418 = shared_constructor_field2;
                                                                    {
                                                                        let source_ml_source2_nominal_override420 = self::append_java_script_call_source_instruction(shared_constructor_field1.clone(), self::JavaScriptCallInstruction::Constant(shared_constructor_field2, Box::new(value.clone())));
                                                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override420.clone()), source_ml_source2_nominal_field418)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceReceiver => {
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), 0f64)
                                    }
                                    self::JavaScriptCallSource::SourceArgument(index) => {
                                        if !(self::test_java_script_call_code_coordinate(index, 1048576f64)) {
                                            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 97, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 110, 111, 110, 110, 101, 103, 97, 116, 105, 118, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])))
                                        } else {
                                            {
                                                let result_2 = self::allocate_java_script_call_source_register(state.clone());
                                                match result_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1_2, shared_constructor_field2_2) => {
                                                        let shared_constructor_field1_2 = *shared_constructor_field1_2;
                                                        {
                                                            let source_ml_source2_nominal_field424 = shared_constructor_field1_2.clone();
                                                            {
                                                                let source_ml_source2_nominal_field426 = shared_constructor_field2_2;
                                                                {
                                                                    let source_ml_source2_nominal_override428 = self::append_java_script_call_source_instruction(shared_constructor_field1_2.clone(), self::JavaScriptCallInstruction::Argument(shared_constructor_field2_2, index));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override428.clone()), source_ml_source2_nominal_field426)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceThrow(source_value) => {
                                        let source_value = *source_value;
                                        {
                                            let value_2 = self::lower_java_script_call_source_expression(source_value.clone(), bindings, state.clone(), (depth + 1f64));
                                            match value_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1_3, shared_constructor_field2_3) => {
                                                    let shared_constructor_field1_3 = *shared_constructor_field1_3;
                                                    {
                                                        let source_ml_source2_nominal_field432 = shared_constructor_field1_3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field434 = shared_constructor_field2_3;
                                                            {
                                                                let source_ml_source2_nominal_override436 = self::append_java_script_call_source_instruction(shared_constructor_field1_3.clone(), self::JavaScriptCallInstruction::Throw(shared_constructor_field2_3));
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override436.clone()), source_ml_source2_nominal_field434)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceTry(_, _, _) => {
                                        self::lower_java_script_call_source_exception(source.clone(), bindings, state.clone(), depth)
                                    }
                                    self::JavaScriptCallSource::SourceBinding(identity) => {
                                        if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                                            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 97, 99, 116, 32, 110, 111, 110, 110, 101, 103, 97, 116, 105, 118, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115])))
                                        } else {
                                            {
                                                let index_2 = ((bindings.len() as f64) - 1f64);
                                                {
                                                    let source_ml_source2_s437 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(source.clone(), bindings, state.clone(), index_2);
                                                    match source_ml_source2_s437.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Done => {
                                                            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 104, 97, 115, 32, 110, 111, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 111, 119, 110, 101, 114])))
                                                        }
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Return(source_ml_source1_temporary_result100) => {
                                                            let source_ml_source1_temporary_result100 = *source_ml_source1_temporary_result100;
                                                            source_ml_source1_temporary_result100.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceLet(identity_2, source_value_2, body) => {
                                        let source_value_2 = *source_value_2;
                                        let body = *body;
                                        if (!(self::test_java_script_call_code_coordinate(identity_2, 65536f64)) || { let ml_values1 = bindings.clone(); let ml_callback1 = { let identity_2 = identity_2; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value439, _) => {
                                                source_ml_source2_data_property_value439
                                            }
                                        }) == identity_2) }) }; ml_values1.iter().cloned().enumerate().any(|(ml_index1, ml_value1)| ml_callback1(ml_value1)) }) {
                                            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 105, 110, 32, 105, 116, 115, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 99, 111, 112, 101])))
                                        } else {
                                            {
                                                let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                                match value_3.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1_4, shared_constructor_field2_4) => {
                                                        let shared_constructor_field1_4 = *shared_constructor_field1_4;
                                                        self::lower_java_script_call_source_expression(body.clone(), {
                                                            let source_ml_source2_concat_value442 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity_2, shared_constructor_field2_4)];
                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), bindings), source_ml_source2_concat_value442)
                                                        }, shared_constructor_field1_4.clone(), (depth + 1f64))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceIf(_, _, _) => {
                                        self::lower_java_script_call_source_branch(source.clone(), bindings, state.clone(), depth)
                                    }
                                    self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f412, source_left, source_right) => {
                                        let source_left = *source_left;
                                        let source_right = *source_right;
                                        {
                                            let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                            match left.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field5, shared_constructor_field6) => {
                                                    let shared_constructor_field5 = *shared_constructor_field5;
                                                    {
                                                        let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings, shared_constructor_field5.clone(), (depth + 1f64));
                                                        match right.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field3, shared_constructor_field4) => {
                                                                let shared_constructor_field3 = *shared_constructor_field3;
                                                                {
                                                                    let result_3 = self::allocate_java_script_call_source_register(shared_constructor_field3.clone());
                                                                    match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1_5, shared_constructor_field2_5) => {
                                                                            let shared_constructor_field1_5 = *shared_constructor_field1_5;
                                                                            if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 100, 100]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]), self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])], &source_ml_source2_f412.clone(), 0f64, false, |a, b| a == b)) {
                                                                                self::refuse_java_script_call_source(shared_constructor_field1_5.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 78, 117, 109, 98, 101, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 111, 114])))
                                                                            } else {
                                                                                {
                                                                                    let source_ml_source2_nominal_field482 = shared_constructor_field1_5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field484 = shared_constructor_field2_5;
                                                                                        {
                                                                                            let source_ml_source2_nominal_override486 = self::append_java_script_call_source_instruction(shared_constructor_field1_5.clone(), self::JavaScriptCallInstruction::NumberBinary(shared_constructor_field2_5, shared_constructor_field6, shared_constructor_field4, source_ml_source2_f412.clone()));
                                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override486.clone()), source_ml_source2_nominal_field484)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceSameValue(source_left_2, source_right_2) => {
                                        let source_left_2 = *source_left_2;
                                        let source_right_2 = *source_right_2;
                                        {
                                            let left_2 = self::lower_java_script_call_source_expression(source_left_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                            match left_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field5_2, shared_constructor_field6_2) => {
                                                    let shared_constructor_field5_2 = *shared_constructor_field5_2;
                                                    {
                                                        let right_2 = self::lower_java_script_call_source_expression(source_right_2.clone(), bindings, shared_constructor_field5_2.clone(), (depth + 1f64));
                                                        match right_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field3_2, shared_constructor_field4_2) => {
                                                                let shared_constructor_field3_2 = *shared_constructor_field3_2;
                                                                {
                                                                    let result_4 = self::allocate_java_script_call_source_register(shared_constructor_field3_2.clone());
                                                                    match result_4.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1_6, shared_constructor_field2_6) => {
                                                                            let shared_constructor_field1_6 = *shared_constructor_field1_6;
                                                                            {
                                                                                let source_ml_source2_nominal_field524 = shared_constructor_field1_6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field526 = shared_constructor_field2_6;
                                                                                    {
                                                                                        let source_ml_source2_nominal_override528 = self::append_java_script_call_source_instruction(shared_constructor_field1_6.clone(), self::JavaScriptCallInstruction::SameValue(shared_constructor_field2_6, shared_constructor_field6_2, shared_constructor_field4_2));
                                                                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override528.clone()), source_ml_source2_nominal_field526)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceStrictEqual(source_left_3, source_right_3) => {
                                        let source_left_3 = *source_left_3;
                                        let source_right_3 = *source_right_3;
                                        {
                                            let left_3 = self::lower_java_script_call_source_expression(source_left_3.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                            match left_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field5_3, shared_constructor_field6_3) => {
                                                    let shared_constructor_field5_3 = *shared_constructor_field5_3;
                                                    {
                                                        let right_3 = self::lower_java_script_call_source_expression(source_right_3.clone(), bindings, shared_constructor_field5_3.clone(), (depth + 1f64));
                                                        match right_3.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field3_3, shared_constructor_field4_3) => {
                                                                let shared_constructor_field3_3 = *shared_constructor_field3_3;
                                                                {
                                                                    let result_5 = self::allocate_java_script_call_source_register(shared_constructor_field3_3.clone());
                                                                    match result_5.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1_7, shared_constructor_field2_7) => {
                                                                            let shared_constructor_field1_7 = *shared_constructor_field1_7;
                                                                            {
                                                                                let source_ml_source2_nominal_field566 = shared_constructor_field1_7.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field568 = shared_constructor_field2_7;
                                                                                    {
                                                                                        let source_ml_source2_nominal_override570 = self::append_java_script_call_source_instruction(shared_constructor_field1_7.clone(), self::JavaScriptCallInstruction::StrictEqual(shared_constructor_field2_7, shared_constructor_field6_3, shared_constructor_field4_3));
                                                                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override570.clone()), source_ml_source2_nominal_field568)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourcePrimitive(operation, arguments) => {
                                        {
                                            let error_2 = self::read_java_script_call_primitive_arity_error(&(operation), (arguments.len() as f64));
                                            if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                self::refuse_java_script_call_source(state.clone(), &(error_2))
                                            } else {
                                                {
                                                    let current = state.clone();
                                                    {
                                                        let arguments_ = Vec::<f64>::new();
                                                        {
                                                            let source_ml_source1_temporary_index97 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s571 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(bindings, depth, current.clone(), arguments_, arguments, source_ml_source1_temporary_index97.clone());
                                                                match source_ml_source2_s571.clone() {
                                                                    self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(current_2, arguments__2) => {
                                                                        let current_2 = *current_2;
                                                                        {
                                                                            let result_6 = self::allocate_java_script_call_source_register(current_2.clone());
                                                                            match result_6.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1_8, shared_constructor_field2_8) => {
                                                                                    let shared_constructor_field1_8 = *shared_constructor_field1_8;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field575 = shared_constructor_field1_8.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field577 = shared_constructor_field2_8;
                                                                                            {
                                                                                                let source_ml_source2_nominal_override579 = self::append_java_script_call_source_instruction(shared_constructor_field1_8.clone(), self::JavaScriptCallInstruction::Primitive(shared_constructor_field2_8, operation.clone(), arguments__2));
                                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override579.clone()), source_ml_source2_nominal_field577)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceRecur(arguments_2) => {
                                        if (shared_constructor_field10 != (0f64)) {
                                            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 114, 101, 99, 117, 114, 114, 101, 110, 99, 101, 32, 116, 104, 114, 111, 117, 103, 104, 32, 97, 110, 32, 97, 99, 116, 105, 118, 101, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 104, 97, 110, 100, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 117, 110, 119, 105, 110, 100, 105, 110, 103])))
                                        } else {
                                            if (((arguments_2.len() as f64)) != shared_constructor_field9) {
                                                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 117, 112, 100, 97, 116, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100])))
                                            } else {
                                                {
                                                    let current_3 = state.clone();
                                                    {
                                                        let arguments__3 = Vec::<f64>::new();
                                                        {
                                                            let source_ml_source1_temporary_index98 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s584 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop102(bindings, depth, current_3.clone(), arguments__3, arguments_2, source_ml_source1_temporary_index98.clone());
                                                                match source_ml_source2_s584.clone() {
                                                                    self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Done(current_4, arguments__4) => {
                                                                        let current_4 = *current_4;
                                                                        {
                                                                            let index_3 = 0f64;
                                                                            {
                                                                                let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(current_4.clone(), &(arguments__4), index_3);
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(current_5.clone(), self::JavaScriptCallInstruction::Jump(0f64))), 0f64)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceSet(source_object, source_key, source_value_3) => {
                                        let source_object = *source_object;
                                        let source_key = *source_key;
                                        let source_value_3 = *source_value_3;
                                        {
                                            let object = self::lower_java_script_call_source_expression(source_object.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                            match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field7, shared_constructor_field8) => {
                                                    let shared_constructor_field7 = *shared_constructor_field7;
                                                    {
                                                        let key = self::lower_java_script_call_source_expression(source_key.clone(), bindings.clone(), shared_constructor_field7.clone(), (depth + 1f64));
                                                        match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field5_4, shared_constructor_field6_4) => {
                                                                let shared_constructor_field5_4 = *shared_constructor_field5_4;
                                                                {
                                                                    let value_4 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings, shared_constructor_field5_4.clone(), (depth + 1f64));
                                                                    match value_4.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field3_4, shared_constructor_field4_4) => {
                                                                            let shared_constructor_field3_4 = *shared_constructor_field3_4;
                                                                            {
                                                                                let result_7 = self::allocate_java_script_call_source_register(shared_constructor_field3_4.clone());
                                                                                match result_7.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1_9, shared_constructor_field2_9) => {
                                                                                        let shared_constructor_field1_9 = *shared_constructor_field1_9;
                                                                                        {
                                                                                            let write = self::append_java_script_call_source_instruction(shared_constructor_field1_9.clone(), self::JavaScriptCallInstruction::OrdinarySet(shared_constructor_field2_9, shared_constructor_field8, shared_constructor_field6_4, shared_constructor_field4_4, shared_constructor_field8));
                                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(write.clone(), self::JavaScriptCallInstruction::RequireWrite(shared_constructor_field2_9))), shared_constructor_field4_4)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceGet(source_object_2, source_key_2) => {
                                        let source_object_2 = *source_object_2;
                                        let source_key_2 = *source_key_2;
                                        {
                                            let object_2 = self::lower_java_script_call_source_expression(source_object_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                            match object_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field5_5, shared_constructor_field6_5) => {
                                                    let shared_constructor_field5_5 = *shared_constructor_field5_5;
                                                    {
                                                        let key_2 = self::lower_java_script_call_source_expression(source_key_2.clone(), bindings, shared_constructor_field5_5.clone(), (depth + 1f64));
                                                        match key_2.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field3_5, shared_constructor_field4_5) => {
                                                                let shared_constructor_field3_5 = *shared_constructor_field3_5;
                                                                {
                                                                    let result_8 = self::allocate_java_script_call_source_register(shared_constructor_field3_5.clone());
                                                                    match result_8.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1_10, shared_constructor_field2_10) => {
                                                                            let shared_constructor_field1_10 = *shared_constructor_field1_10;
                                                                            {
                                                                                let source_ml_source2_nominal_field604 = shared_constructor_field1_10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field606 = shared_constructor_field2_10;
                                                                                    {
                                                                                        let source_ml_source2_nominal_override608 = self::append_java_script_call_source_instruction(shared_constructor_field1_10.clone(), self::JavaScriptCallInstruction::OrdinaryGet(shared_constructor_field2_10, shared_constructor_field6_5, shared_constructor_field4_5, shared_constructor_field6_5));
                                                                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override608.clone()), source_ml_source2_nominal_field606)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallSource::SourceCall(source_callee, source_receiver, arguments_3) => {
                                        let source_callee = *source_callee;
                                        let source_receiver = *source_receiver;
                                        if (((arguments_3.len() as f64)) > (4096f64)) {
                                            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 98, 117, 100, 103, 101, 116])))
                                        } else {
                                            {
                                                let callee = self::lower_java_script_call_source_expression(source_callee.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                                                match callee.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field5_6, shared_constructor_field6_6) => {
                                                        let shared_constructor_field5_6 = *shared_constructor_field5_6;
                                                        {
                                                            let receiver = self::lower_java_script_call_source_expression(source_receiver.clone(), bindings.clone(), shared_constructor_field5_6.clone(), (depth + 1f64));
                                                            match receiver.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field3_6, shared_constructor_field4_6) => {
                                                                    let shared_constructor_field3_6 = *shared_constructor_field3_6;
                                                                    {
                                                                        let current_6 = shared_constructor_field3_6.clone();
                                                                        {
                                                                            let arguments__5 = Vec::<f64>::new();
                                                                            {
                                                                                let source_ml_source1_temporary_index99 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let source_ml_source2_s611 = self::source_ml_source1_temporary_lower_java_script_call_source_expression_loop104(bindings, depth, current_6.clone(), arguments__5, arguments_3, source_ml_source1_temporary_index99.clone());
                                                                                    match source_ml_source2_s611.clone() {
                                                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Done(current_7, arguments__6) => {
                                                                                            let current_7 = *current_7;
                                                                                            {
                                                                                                let result_9 = self::allocate_java_script_call_source_register(current_7.clone());
                                                                                                match result_9.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1_11, shared_constructor_field2_11) => {
                                                                                                        let shared_constructor_field1_11 = *shared_constructor_field1_11;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field617 = shared_constructor_field1_11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field619 = shared_constructor_field2_11;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override621 = self::append_java_script_call_source_instruction(shared_constructor_field1_11.clone(), self::JavaScriptCallInstruction::Call(shared_constructor_field2_11, shared_constructor_field6_6, shared_constructor_field4_6, arguments__6));
                                                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override621.clone()), source_ml_source2_nominal_field619)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_exception(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, shared_constructor_field15, _) => {
            match source.clone() {
                self::JavaScriptCallSource::SourceConstant(_) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceReceiver => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceArgument(_) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceThrow(_) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceTry(identity, source_body, source_handler) => {
                    let source_body = *source_body;
                    let source_handler = *source_handler;
                    if (!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || { let ml_values2 = bindings.clone(); let ml_callback2 = { let identity = identity; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value651, _) => {
                            source_ml_source2_data_property_value651
                        }
                    }) == identity) }) }; ml_values2.iter().cloned().enumerate().any(|(ml_index2, ml_value2)| ml_callback2(ml_value2)) }) {
                        self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 105, 110, 32, 105, 116, 115, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 99, 111, 112, 101])))
                    } else {
                        if (shared_constructor_field15 >= (1024f64)) {
                            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114, 32, 98, 117, 100, 103, 101, 116])))
                        } else {
                            {
                                let caught = self::allocate_java_script_call_source_register(builder.clone());
                                match caught.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field13, shared_constructor_field14) => {
                                        let shared_constructor_field13 = *shared_constructor_field13;
                                        {
                                            let result = self::allocate_java_script_call_source_register(shared_constructor_field13.clone());
                                            match result.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field11, shared_constructor_field12) => {
                                                    let shared_constructor_field11 = *shared_constructor_field11;
                                                    {
                                                        let position = ((match &shared_constructor_field11 { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value656, _, _, _, _, _) => source_ml_source2_data_property_value656.as_slice() }).len() as f64);
                                                        {
                                                            let protected_builder = self::append_java_script_call_source_instruction({
                                                                let source_ml_source2_nominal_spread659 = shared_constructor_field11.clone();
                                                                match source_ml_source2_nominal_spread659.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                                                        {
                                                                            let source_ml_source2_nominal_field660 = shared_constructor_field1;
                                                                            {
                                                                                let source_ml_source2_nominal_field662 = shared_constructor_field2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field664 = shared_constructor_field3;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field666 = shared_constructor_field4;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field668 = shared_constructor_field5;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field670 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override672 = (shared_constructor_field15 + 1f64);
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field660, source_ml_source2_nominal_field662, source_ml_source2_nominal_field664, source_ml_source2_nominal_field666, source_ml_source2_nominal_override672, source_ml_source2_nominal_field670.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }, self::JavaScriptCallInstruction::PushHandler(shared_constructor_field14, 0f64));
                                                            {
                                                                let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), protected_builder.clone(), (depth + 1f64));
                                                                match body.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field9, shared_constructor_field10) => {
                                                                        let shared_constructor_field9 = *shared_constructor_field9;
                                                                        {
                                                                            let popped = self::append_java_script_call_source_instruction({
                                                                                let source_ml_source2_nominal_spread676 = shared_constructor_field9.clone();
                                                                                match source_ml_source2_nominal_spread676.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                                                                        {
                                                                                            let source_ml_source2_nominal_field677 = shared_constructor_field1_2;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field679 = shared_constructor_field2_2;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field681 = shared_constructor_field3_2;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field683 = shared_constructor_field4_2;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field685 = shared_constructor_field5_2;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field687 = shared_constructor_field6_2.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override689 = shared_constructor_field15;
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field677, source_ml_source2_nominal_field679, source_ml_source2_nominal_field681, source_ml_source2_nominal_field683, source_ml_source2_nominal_override689, source_ml_source2_nominal_field687.clone())
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }, self::JavaScriptCallInstruction::PopHandler);
                                                                            {
                                                                                let copied = self::append_java_script_call_source_instruction(popped.clone(), self::JavaScriptCallInstruction::Copy(shared_constructor_field12, shared_constructor_field10));
                                                                                {
                                                                                    let jump = ((match &copied { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value692, _, _, _, _, _) => source_ml_source2_data_property_value692.as_slice() }).len() as f64);
                                                                                    {
                                                                                        let alternative = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                                                        {
                                                                                            let target = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value693, _, _, _, _, _) => source_ml_source2_data_property_value693.as_slice() }).len() as f64);
                                                                                            {
                                                                                                let handler = self::lower_java_script_call_source_expression(source_handler.clone(), {
                                                                                                    let source_ml_source2_concat_value696 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, shared_constructor_field14)];
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), bindings), source_ml_source2_concat_value696)
                                                                                                }, alternative.clone(), (depth + 1f64));
                                                                                                match handler.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field7, shared_constructor_field8) => {
                                                                                                        let shared_constructor_field7 = *shared_constructor_field7;
                                                                                                        {
                                                                                                            let finished = self::append_java_script_call_source_instruction(shared_constructor_field7.clone(), self::JavaScriptCallInstruction::Copy(shared_constructor_field12, shared_constructor_field8));
                                                                                                            match finished.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(shared_constructor_field1_3, shared_constructor_field2_3, shared_constructor_field3_3, shared_constructor_field4_3, shared_constructor_field5_3, shared_constructor_field6_3) => {
                                                                                                                    {
                                                                                                                        let end = (shared_constructor_field1_3.len() as f64);
                                                                                                                        if (shared_constructor_field6_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(finished.clone()), 0f64)
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let instructions = { let ml_values3 = shared_constructor_field1_3.clone(); let ml_callback3 = { let position = position; let caught = caught.clone(); let target = target; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                                                                                    self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value704) => {
                                                                                                                                            source_ml_source2_data_property_value704
                                                                                                                                        }
                                                                                                                                    }, target)
                                                                                                                                } else {
                                                                                                                                    if (index == jump) {
                                                                                                                                        self::JavaScriptCallInstruction::Jump(end)
                                                                                                                                    } else {
                                                                                                                                        entry.clone()
                                                                                                                                    }
                                                                                                                                } }) }; ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3, (ml_index3 as f64))).collect::<Vec<_>>() };
                                                                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                                                                                                    let source_ml_source2_nominal_field706 = shared_constructor_field1_3;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field708 = shared_constructor_field2_3;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field710 = shared_constructor_field3_3;
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field712 = shared_constructor_field4_3;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field714 = shared_constructor_field5_3;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field716 = shared_constructor_field6_3.clone();
                                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(instructions, source_ml_source2_nominal_field708, source_ml_source2_nominal_field710, source_ml_source2_nominal_field712, source_ml_source2_nominal_field714, source_ml_source2_nominal_field716.clone())
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }), shared_constructor_field12)
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptCallSource::SourceBinding(_) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceLet(_, _, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceIf(_, _, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceSameValue(_, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceRecur(_) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceSet(_, _, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceGet(_, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
                self::JavaScriptCallSource::SourceCall(_, _, _) => {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
                }
            }
        }
    }
}
