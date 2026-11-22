// Generated from complete js/src/translation/javascript-collection-iterator-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCollectionIteratorCallCompilation;
use super::JavaScriptCollectionIteratorCallableDirectory;
use super::JavaScriptCollectionIteratorMethodOwner;
use super::JavaScriptSemanticValue;
use super::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_collection_iterator_operation_kind;
use super::read_java_script_collection_iterator_operation_strategy;
use super::test_java_script_collection_iterator_method_alias;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop15(
    mut directory: self::JavaScriptCollectionIteratorCallableDirectory,
    mut empty: self::JavaScriptCallProgram,
    mut functions: Vec<self::JavaScriptCallFunction>,
    mut owners: Vec<self::JavaScriptCollectionIteratorMethodOwner>,
    mut source_ml_temporary_values13: Vec<self::JavaScriptCollectionIteratorMethodOwner>,
    mut source_ml_temporary_index13: self::ml::Big,
) -> self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result {
    loop {
        return if source_ml_temporary_index13
            < (self::ml::Big::from_u128(source_ml_temporary_values13.len() as u128))
        {
            {
                let owner = self::ml_array::at(
                    &source_ml_temporary_values13,
                    source_ml_temporary_index13.to_index(),
                );
                {
                    let kind = self::read_java_script_collection_iterator_operation_kind(&(match owner.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, source_ml_source1_data_property_value713, _, _, _) => {
                            source_ml_source1_data_property_value713.clone()
                        }
                    }));
                    {
                        let strategy = self::read_java_script_collection_iterator_operation_strategy(&(match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, source_ml_source1_data_property_value714, _, _, _) => {
                                source_ml_source1_data_property_value714.clone()
                            }
                        }));
                        if ((((kind == (self::ml_string::Text::from_units(vec![]))) || (({
                            let source_ml_source1_type_query_operand718 = match owner.clone() {
                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value717, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value717.clone()
                                }
                            };
                            self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                        }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])))) || ((match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value720, _, _, _, _, _) => {
                                source_ml_source1_data_property_value720.clone()
                            }
                        }) <= (self::ml::Big::from_i128(0)))) || ((match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value722, _, _, _, _, _) => {
                                source_ml_source1_data_property_value722.clone()
                            }
                        }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615)))) || !(self::ml_array::includes((match &directory { self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(source_ml_source1_data_property_value724, _, _) => source_ml_source1_data_property_value724.as_slice() }), &match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value725, _, _, _, _) => {
                                source_ml_source1_data_property_value725.clone()
                            }
                        }, 0f64, false, |a, b| a == b)) {
                            self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 111, 108, 101]))))
                        } else {
                            {
                                let prior = { let ml_values1 = owners.clone(); let ml_callback1 = { let owner = owner.clone(); std::rc::Rc::new(move |value: self::JavaScriptCollectionIteratorMethodOwner| -> bool { (match value.clone() {
                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value728, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value728.clone()
                                    }
                                }) == (match owner.clone() {
                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value729, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value729.clone()
                                    }
                                }) }) }; ml_values1.iter().cloned().enumerate().find_map(|(ml_index1, ml_value1)| if ml_callback1(ml_value1.clone()) { Some(ml_value1) } else { None }) };
                                if (match (prior.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    {
                                        let owners_2 = {
                                            let source_ml_source1_concat_value730 = owners;
                                            {
                                                let source_ml_source1_concat_value731 = vec![owner.clone()];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCollectionIteratorMethodOwner>::new(), source_ml_source1_concat_value730), source_ml_source1_concat_value731)
                                            }
                                        };
                                        if kind == (self::ml_string::Text::from_units(vec![115, 101, 108, 102])) {
                                            {
                                                let functions_2 = {
                                                    let source_ml_source1_concat_value742 = functions;
                                                    {
                                                        let source_ml_source1_concat_value743 = vec![{
                                                            let source_ml_source1_constructor_field733 = match owner.clone() {
                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value732, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value732.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_constructor_field735 = match owner.clone() {
                                                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value734, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value734.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_constructor_field736 = self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]);
                                                                    {
                                                                        let source_ml_source1_constructor_field737 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_constructor_field738 = 0f64;
                                                                            {
                                                                                let source_ml_source1_constructor_field739 = 2f64;
                                                                                {
                                                                                    let source_ml_source1_constructor_field740 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                    {
                                                                                        let source_ml_source1_constructor_field741 = vec![self::JavaScriptCallInstruction::Copy(1f64, 0f64), self::JavaScriptCallInstruction::Return(1f64)];
                                                                                        self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field733, source_ml_source1_constructor_field735, source_ml_source1_constructor_field736, Box::new(source_ml_source1_constructor_field737), source_ml_source1_constructor_field738, source_ml_source1_constructor_field739, source_ml_source1_constructor_field740, source_ml_source1_constructor_field741, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }];
                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), source_ml_source1_concat_value742), source_ml_source1_concat_value743)
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index13_3 = source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (functions, owners, source_ml_temporary_index13) = (functions_2, owners_2, source_ml_temporary_index13_3);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
{
                                            if (kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) && (strategy == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) {
                                                if match {
                                                    let source_ml_source1_logical758 = {
                                                        let source_ml_source1_s745 = match owner.clone() {
                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value755, _, _) => {
                                                                let source_ml_source1_data_property_value755 = *source_ml_source1_data_property_value755;
                                                                source_ml_source1_data_property_value755.clone()
                                                            }
                                                        };
                                                        match source_ml_source1_s745 {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                true
                                                            }
                                                            self::JavaScriptSemanticValue::Null => {
                                                                true
                                                            }
                                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f746) => {
                                                                true
                                                            }
                                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f747) => {
                                                                true
                                                            }
                                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f748) => {
                                                                true
                                                            }
                                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f749) => {
                                                                true
                                                            }
                                                            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f750, source_ml_source1_f751) => {
                                                                true
                                                            }
                                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f752) => {
                                                                true
                                                            }
                                                            self::JavaScriptSemanticValue::Object(source_ml_source1_f753, source_ml_source1_f754) => {
                                                                false
                                                            }
                                                        }
                                                    };
                                                    if source_ml_source1_logical758 {
                                                        Some(source_ml_source1_logical758)
                                                    } else {
                                                        match match owner.clone() {
                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value756, _, _) => {
                                                                let source_ml_source1_data_property_value756 = *source_ml_source1_data_property_value756;
                                                                source_ml_source1_data_property_value756.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Null => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Number(_) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Text(_) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value757) => {
                                                                Some(source_ml_source1_data_property_value757)
                                                            }
                                                        }
                                                    }
                                                } { Some(ml_present) => ml_present, None => false } {
                                                    self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))))
                                                } else {
                                                    {
                                                        let functions_3 = {
                                                            let source_ml_source1_concat_value775 = functions;
                                                            {
                                                                let source_ml_source1_concat_value776 = vec![{
                                                                    let source_ml_source1_constructor_field760 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value759, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value759.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_constructor_field762 = match owner.clone() {
                                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value761, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value761.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_constructor_field763 = self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]);
                                                                            {
                                                                                let source_ml_source1_constructor_field764 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source1_constructor_field765 = 0f64;
                                                                                    {
                                                                                        let source_ml_source1_constructor_field766 = 4f64;
                                                                                        {
                                                                                            let source_ml_source1_constructor_field767 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                            {
                                                                                                let source_ml_source1_constructor_field774 = vec![self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 99, 111, 101, 114, 99, 105, 98, 108, 101]), vec![0f64]), self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), vec![0f64]), self::JavaScriptCallInstruction::Constant(2f64, Box::new(match owner.clone() {
                                                                                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value768, _, _) => {
                                                                                                        let source_ml_source1_data_property_value768 = *source_ml_source1_data_property_value768;
                                                                                                        source_ml_source1_data_property_value768.clone()
                                                                                                    }
                                                                                                })), {
                                                                                                    let source_ml_source1_constructor_field769 = 3f64;
                                                                                                    {
                                                                                                        let source_ml_source1_constructor_field770 = 2f64;
                                                                                                        {
                                                                                                            let source_ml_source1_constructor_field771 = self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]);
                                                                                                            {
                                                                                                                let source_ml_source1_constructor_field772 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]);
                                                                                                                {
                                                                                                                    let source_ml_source1_constructor_field773 = Some(1f64);
                                                                                                                    self::JavaScriptCallInstruction::CollectionIteratorAllocate(source_ml_source1_constructor_field769, source_ml_source1_constructor_field770, source_ml_source1_constructor_field771, source_ml_source1_constructor_field772, Some(source_ml_source1_constructor_field773,))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }, self::JavaScriptCallInstruction::Return(3f64)];
                                                                                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field760, source_ml_source1_constructor_field762, source_ml_source1_constructor_field763, Box::new(source_ml_source1_constructor_field764), source_ml_source1_constructor_field765, source_ml_source1_constructor_field766, source_ml_source1_constructor_field767, source_ml_source1_constructor_field774, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }];
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), source_ml_source1_concat_value775), source_ml_source1_concat_value776)
                                                            }
                                                        };
                                                        {
                                                            let source_ml_temporary_index13_4 = source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                (functions, owners, source_ml_temporary_index13) = (functions_3, owners_2, source_ml_temporary_index13_4);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
let ml_library_inequality_1 = strategy != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                    if match {
                                                        let source_ml_source1_logical791 = {
                                                            let source_ml_source1_s778 = match owner.clone() {
                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value788, _, _) => {
                                                                    let source_ml_source1_data_property_value788 = *source_ml_source1_data_property_value788;
                                                                    source_ml_source1_data_property_value788.clone()
                                                                }
                                                            };
                                                            match source_ml_source1_s778 {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    true
                                                                }
                                                                self::JavaScriptSemanticValue::Null => {
                                                                    true
                                                                }
                                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f779) => {
                                                                    true
                                                                }
                                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f780) => {
                                                                    true
                                                                }
                                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f781) => {
                                                                    true
                                                                }
                                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f782) => {
                                                                    true
                                                                }
                                                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f783, source_ml_source1_f784) => {
                                                                    true
                                                                }
                                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f785) => {
                                                                    true
                                                                }
                                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f786, source_ml_source1_f787) => {
                                                                    false
                                                                }
                                                            }
                                                        };
                                                        if source_ml_source1_logical791 {
                                                            Some(source_ml_source1_logical791)
                                                        } else {
                                                            match match owner.clone() {
                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value789, _, _) => {
                                                                    let source_ml_source1_data_property_value789 = *source_ml_source1_data_property_value789;
                                                                    source_ml_source1_data_property_value789.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Null => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Number(_) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Text(_) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value790) => {
                                                                    Some(source_ml_source1_data_property_value790)
                                                                }
                                                            }
                                                        }
                                                    } { Some(ml_present) => ml_present, None => false } {
                                                        self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))))
                                                    } else {
                                                        {
                                                            let functions_4 = {
                                                                let source_ml_source1_concat_value807 = functions;
                                                                {
                                                                    let source_ml_source1_concat_value808 = vec![{
                                                                        let source_ml_source1_constructor_field793 = match owner.clone() {
                                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value792, _, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value792.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_constructor_field795 = match owner.clone() {
                                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value794, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value794.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_constructor_field796 = self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]);
                                                                                {
                                                                                    let source_ml_source1_constructor_field797 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source1_constructor_field798 = 0f64;
                                                                                        {
                                                                                            let source_ml_source1_constructor_field799 = 3f64;
                                                                                            {
                                                                                                let source_ml_source1_constructor_field800 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                {
                                                                                                    let source_ml_source1_constructor_field806 = vec![self::JavaScriptCallInstruction::Constant(1f64, Box::new(match owner.clone() {
                                                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value801, _, _) => {
                                                                                                            let source_ml_source1_data_property_value801 = *source_ml_source1_data_property_value801;
                                                                                                            source_ml_source1_data_property_value801.clone()
                                                                                                        }
                                                                                                    })), {
                                                                                                        let source_ml_source1_constructor_field802 = 2f64;
                                                                                                        {
                                                                                                            let source_ml_source1_constructor_field803 = 1f64;
                                                                                                            {
                                                                                                                let source_ml_source1_constructor_field804 = kind.clone();
                                                                                                                {
                                                                                                                    let source_ml_source1_constructor_field805 = strategy.clone();
                                                                                                                    self::JavaScriptCallInstruction::CollectionIteratorAllocate(source_ml_source1_constructor_field802, source_ml_source1_constructor_field803, source_ml_source1_constructor_field804, source_ml_source1_constructor_field805, None::<(Option<f64>,)>)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }, self::JavaScriptCallInstruction::Return(2f64)];
                                                                                                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field793, source_ml_source1_constructor_field795, source_ml_source1_constructor_field796, Box::new(source_ml_source1_constructor_field797), source_ml_source1_constructor_field798, source_ml_source1_constructor_field799, source_ml_source1_constructor_field800, source_ml_source1_constructor_field806, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), source_ml_source1_concat_value807), source_ml_source1_concat_value808)
                                                                }
                                                            };
                                                            {
                                                                let source_ml_temporary_index13_5 = source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    (functions, owners, source_ml_temporary_index13) = (functions_4, owners_2, source_ml_temporary_index13_5);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if match {
                                                        let source_ml_source1_logical839 = {
                                                            let source_ml_source1_logical836 = {
                                                                let source_ml_source1_logical823 = {
                                                                    let source_ml_source1_s810 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value820, _) => {
                                                                            let source_ml_source1_data_property_value820 = *source_ml_source1_data_property_value820;
                                                                            source_ml_source1_data_property_value820.clone()
                                                                        }
                                                                    };
                                                                    match source_ml_source1_s810 {
                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Null => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f811) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Number(source_ml_source1_f812) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f813) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Text(source_ml_source1_f814) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f815, source_ml_source1_f816) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f817) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Object(source_ml_source1_f818, source_ml_source1_f819) => {
                                                                            false
                                                                        }
                                                                    }
                                                                };
                                                                if source_ml_source1_logical823 {
                                                                    Some(source_ml_source1_logical823)
                                                                } else {
                                                                    match match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value821, _) => {
                                                                            let source_ml_source1_data_property_value821 = *source_ml_source1_data_property_value821;
                                                                            source_ml_source1_data_property_value821.clone()
                                                                        }
                                                                    } {
                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                            None::<bool>
                                                                        }
                                                                        self::JavaScriptSemanticValue::Null => {
                                                                            None::<bool>
                                                                        }
                                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                                            None::<bool>
                                                                        }
                                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                                            None::<bool>
                                                                        }
                                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                                            None::<bool>
                                                                        }
                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                            None::<bool>
                                                                        }
                                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                            None::<bool>
                                                                        }
                                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                                            None::<bool>
                                                                        }
                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value822) => {
                                                                            Some(source_ml_source1_data_property_value822)
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            if (source_ml_source1_logical836.clone()).unwrap_or_default() {
                                                                source_ml_source1_logical836.clone().expect("a proven present value")
                                                            } else {
                                                                {
                                                                    let source_ml_source1_s825 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value835) => {
                                                                            let source_ml_source1_data_property_value835 = *source_ml_source1_data_property_value835;
                                                                            source_ml_source1_data_property_value835.clone()
                                                                        }
                                                                    };
                                                                    match source_ml_source1_s825 {
                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Null => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f826) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Number(source_ml_source1_f827) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f828) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Text(source_ml_source1_f829) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f830, source_ml_source1_f831) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f832) => {
                                                                            true
                                                                        }
                                                                        self::JavaScriptSemanticValue::Object(source_ml_source1_f833, source_ml_source1_f834) => {
                                                                            false
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        if source_ml_source1_logical839 {
                                                            Some(source_ml_source1_logical839)
                                                        } else {
                                                            match match owner.clone() {
                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value837) => {
                                                                    let source_ml_source1_data_property_value837 = *source_ml_source1_data_property_value837;
                                                                    source_ml_source1_data_property_value837.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Null => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Number(_) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Text(_) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                    None::<bool>
                                                                }
                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value838) => {
                                                                    Some(source_ml_source1_data_property_value838)
                                                                }
                                                            }
                                                        }
                                                    } { Some(ml_present) => ml_present, None => false } {
                                                        self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115]))))
                                                    } else {
                                                        {
                                                            let functions_5 = {
                                                                let source_ml_source1_concat_value852 = functions;
                                                                {
                                                                    let source_ml_source1_concat_value853 = vec![{
                                                                        let source_ml_source1_constructor_field841 = match owner.clone() {
                                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value840, _, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value840.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_constructor_field843 = match owner.clone() {
                                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value842, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value842.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_constructor_field844 = self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]);
                                                                                {
                                                                                    let source_ml_source1_constructor_field845 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source1_constructor_field846 = 0f64;
                                                                                        {
                                                                                            let source_ml_source1_constructor_field847 = 4f64;
                                                                                            {
                                                                                                let source_ml_source1_constructor_field848 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                {
                                                                                                    let source_ml_source1_constructor_field851 = vec![self::JavaScriptCallInstruction::Constant(1f64, Box::new(match owner.clone() {
                                                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value849, _) => {
                                                                                                            let source_ml_source1_data_property_value849 = *source_ml_source1_data_property_value849;
                                                                                                            source_ml_source1_data_property_value849.clone()
                                                                                                        }
                                                                                                    })), self::JavaScriptCallInstruction::Constant(2f64, Box::new(match owner.clone() {
                                                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value850) => {
                                                                                                            let source_ml_source1_data_property_value850 = *source_ml_source1_data_property_value850;
                                                                                                            source_ml_source1_data_property_value850.clone()
                                                                                                        }
                                                                                                    })), self::JavaScriptCallInstruction::CollectionIteratorResult(3f64, kind.clone(), 1f64, 2f64), self::JavaScriptCallInstruction::Return(3f64)];
                                                                                                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field841, source_ml_source1_constructor_field843, source_ml_source1_constructor_field844, Box::new(source_ml_source1_constructor_field845), source_ml_source1_constructor_field846, source_ml_source1_constructor_field847, source_ml_source1_constructor_field848, source_ml_source1_constructor_field851, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), source_ml_source1_concat_value852), source_ml_source1_concat_value853)
                                                                }
                                                            };
                                                            {
                                                                let source_ml_temporary_index13_6 = source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    (functions, owners, source_ml_temporary_index13) = (functions_5, owners_2, source_ml_temporary_index13_6);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
}
                                        }
                                    }
                                } else {
                                    if (self::test_java_script_collection_iterator_method_alias(prior.clone().expect("a proven present value"), owner.clone())) {
                                        {
                                            let source_ml_temporary_index13_2 = source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_temporary_index13 = source_ml_temporary_index13_2;
                                                continue;
                                            }
                                        }
                                    }
{
                                        self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![111, 110, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 99, 97, 110, 110, 111, 116, 32, 97, 99, 113, 117, 105, 114, 101, 32, 99, 111, 110, 102, 108, 105, 99, 116, 105, 110, 103, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115]))))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Done(functions, owners)
        };
    }
}
