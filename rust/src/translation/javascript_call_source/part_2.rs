// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::ml_array;
use super::ml_math;
use super::read_java_script_semantic_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_constant_error(value: self::JavaScriptSemanticValue) -> String {
    self::read_java_script_semantic_value_error(value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_class(operation: &str) -> String {
    if (operation == "raiseJavaScriptError") {
        String::from("Error")
    } else {
        if (operation == "raiseJavaScriptTypeError") {
            String::from("TypeError")
        } else {
            if (operation == "raiseJavaScriptRangeError") {
                String::from("RangeError")
            } else {
                if (operation == "raiseJavaScriptSyntaxError") {
                    String::from("SyntaxError")
                } else {
                    if (operation == "raiseJavaScriptReferenceError") {
                        String::from("ReferenceError")
                    } else {
                        if (operation == "raiseJavaScriptURIError") {
                            String::from("URIError")
                        } else {
                            if (operation == "raiseJavaScriptEvalError") {
                                String::from("EvalError")
                            } else {
                                String::from("")
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
pub fn read_java_script_call_integer_arity(operation: &str) -> f64 {
    if (((operation == "negate integer") || (operation == "complement integer bits")) || (operation == "complement number bits")) {
        1f64
    } else {
        if self::ml_array::includes(&{ let ml_reference_value2 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("convert integer radix text"), String::from("wrap signed integer"), String::from("wrap unsigned integer"), String::from("add integers"), String::from("subtract integers"), String::from("multiply integers"), String::from("divide integers"), String::from("read integer remainder"), String::from("raise integer power"), String::from("compare integers"), String::from("compare number integer"), String::from("compare text integer"), String::from("conjoin integer bits"), String::from("disjoin integer bits"), String::from("exclude equal integer bits"), String::from("shift integer bits left"), String::from("shift integer bits right"), String::from("conjoin number bits"), String::from("disjoin number bits"), String::from("exclude equal number bits"), String::from("shift number bits left"), String::from("shift number bits right"), String::from("shift number bits right unsigned")]))).borrow().clone(); ml_reference_value2 }, &operation.to_owned(), 0f64, false, |a, b| a == b) {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_numeric_parsing_arity(operation: &str) -> f64 {
    if (operation == "parse integer text prefix") {
        2f64
    } else {
        if (operation == "parse float text prefix") {
            1f64
        } else {
            (-1f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_method_primitive_arity(operation: &str) -> f64 {
    if self::ml_array::includes(&{ let ml_reference_value3 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("test well formed String"), String::from("copy well formed String"), String::from("convert character code text"), String::from("convert code point text")]))).borrow().clone(); ml_reference_value3 }, &operation.to_owned(), 0f64, false, |a, b| a == b) {
        1f64
    } else {
        if self::ml_array::includes(&{ let ml_reference_value4 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("map lower String case"), String::from("map upper String case"), String::from("require String receiver"), String::from("read String code unit"), String::from("test String own key"), String::from("read String own key")]))).borrow().clone(); ml_reference_value4 }, &operation.to_owned(), 0f64, false, |a, b| a == b) {
            2f64
        } else {
            if self::ml_array::includes(&{ let ml_reference_value5 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("substring String units"), String::from("substr String units"), String::from("test String prefix at"), String::from("test String suffix at"), String::from("test String inclusion at")]))).borrow().clone(); ml_reference_value5 }, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                3f64
            } else {
                (-1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_unary_number_operation(operation: &str) -> bool {
    self::ml_array::includes(&{ let ml_reference_value6 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("test NaN number"), String::from("test finite number"), String::from("negate number"), String::from("truncate number"), String::from("read absolute number"), String::from("floor number"), String::from("ceil number"), String::from("round number"), String::from("read number sign"), String::from("read square root"), String::from("test integer number"), String::from("test safe integer number")]))).borrow().clone(); ml_reference_value6 }, &operation.to_owned(), 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_primitive_arity_error(operation: &str, count: f64) -> String {
    {
        let parsing_arity = self::read_java_script_numeric_parsing_arity(operation);
        if (parsing_arity >= (0f64)) {
            if (count == parsing_arity) {
                String::from("")
            } else {
                String::from("numeric parsing retains every converted operand")
            }
        } else {
            {
                let string_arity = self::read_java_script_string_method_primitive_arity(operation);
                if (string_arity >= (0f64)) {
                    if (count == string_arity) {
                        String::from("")
                    } else {
                        String::from("String operations retain every actual converted operand")
                    }
                } else {
                    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (4096f64))) {
                        String::from("primitive operands exceed their explicit call register budget")
                    } else {
                        if (operation == "throw restricted Function access") {
                            if (count == (0f64)) {
                                String::from("")
                            } else {
                                String::from("restricted Function access ignores receiver and arguments")
                            }
                        } else {
                            if ((self::read_java_script_source_exception_class(operation)) != "") {
                                if (count == (2f64)) {
                                    String::from("")
                                } else {
                                    String::from("a source Error requires its actual text message and own-message presence")
                                }
                            } else {
                                {
                                    let integer_arity = self::read_java_script_call_integer_arity(operation);
                                    if (integer_arity != (0f64)) {
                                        if (count == integer_arity) {
                                            String::from("")
                                        } else {
                                            String::from("primitive instructions require every actual operand coordinate")
                                        }
                                    } else {
                                        if (((operation == "read minimum number") || (operation == "read maximum number")) || (operation == "construct text units")) {
                                            String::from("")
                                        } else {
                                            if self::test_java_script_call_unary_number_operation(operation) {
                                                if (count == (1f64)) {
                                                    String::from("")
                                                } else {
                                                    String::from("primitive instructions require every actual operand coordinate")
                                                }
                                            } else {
                                                {
                                                    let unary = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("require Array creation length"), String::from("require V8 array join length"), String::from("construct Symbol descriptive text"), String::from("round binary32 number"), String::from("count leading zero bits"), String::from("read integer index"), String::from("require callable target"), String::from("require array like arguments Object"), String::from("require Object assign target"), String::from("require Object reflection target"), String::from("require nonnull Object input"), String::from("require object for in"), String::from("require ordinary property object"), String::from("require array method object"), String::from("read array like length"), String::from("require delete success"), String::from("require collection adder"), String::from("require Map entry object"), String::from("require object coercible"), String::from("test object value"), String::from("read semantic type"), String::from("read text length"), String::from("trim text"), String::from("trim text start"), String::from("trim text end"), String::from("convert explicit number"), String::from("coerce primitive number"), String::from("coerce primitive numeric"), String::from("convert primitive value"), String::from("convert number primitive value"), String::from("increment numeric value"), String::from("decrement numeric value"), String::from("negate numeric value"), String::from("convert primitive text"), String::from("convert explicit integer"), String::from("coerce primitive integer")]));
                                                    {
                                                        let binary = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("read array deletion count"), String::from("read checked array replacement index"), String::from("multiply signed integer bits"), String::from("read relative array index"), String::from("read checked array append length"), String::from("compare relational values"), String::from("subtract numeric values"), String::from("multiply numeric values"), String::from("divide numeric values"), String::from("read numeric remainder"), String::from("raise numeric power"), String::from("repeat text"), String::from("read text code point"), String::from("read text character"), String::from("read relative text character"), String::from("concatenate text"), String::from("compare text"), String::from("test text prefix"), String::from("test text suffix"), String::from("test text inclusion"), String::from("add primitive values")]));
                                                        {
                                                            let ternary = std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("pad text start"), String::from("pad text end"), String::from("slice text"), String::from("find text"), String::from("find last text"), String::from("replace text"), String::from("replace all text")]));
                                                            {
                                                                let arity = if self::ml_array::includes(&{ let ml_reference_value7 = unary.borrow().clone(); ml_reference_value7 }, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                                                    1f64
                                                                } else {
                                                                    if self::ml_array::includes(&{ let ml_reference_value8 = binary.borrow().clone(); ml_reference_value8 }, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                                                        2f64
                                                                    } else {
                                                                        if self::ml_array::includes(&{ let ml_reference_value9 = ternary.borrow().clone(); ml_reference_value9 }, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                                                            3f64
                                                                        } else {
                                                                            (-1f64)
                                                                        }
                                                                    }
                                                                };
                                                                if (arity < (0f64)) {
                                                                    String::from("primitive instructions require their actual registered operation")
                                                                } else {
                                                                    if (count != arity) {
                                                                        String::from("primitive instructions require every actual operand coordinate")
                                                                    } else {
                                                                        String::from("")
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
pub fn read_java_script_call_source_accessor_kind(property: self::JavaScriptCallSourceProperty) -> String {
    match property.clone() {
        self::JavaScriptCallSourceProperty::SourceObjectAccessor(_, _, kind, _) => {
            kind.clone()
        }
        _ => {
            String::from("")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_call_source(builder: self::JavaScriptCallSourceBuilder, error: &str) -> self::JavaScriptCallSourceResult {
    self::JavaScriptCallSourceResult::SourceResult(Box::new({
        let source_ml_source2_nominal_field721 = match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value722, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value722.clone()
            }
        };
        {
            let source_ml_source2_nominal_field723 = match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value724, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value724
                }
            };
            {
                let source_ml_source2_nominal_field725 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value726, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value726
                    }
                };
                {
                    let source_ml_source2_nominal_field727 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value728, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value728
                        }
                    };
                    {
                        let source_ml_source2_nominal_field729 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value730, _, _, _, _, _) => {
                                source_ml_source2_data_property_value730
                            }
                        };
                        {
                            let source_ml_source2_nominal_field731 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value732, _, _, _, _) => {
                                    source_ml_source2_data_property_value732.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field733 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value734, _, _, _) => {
                                        source_ml_source2_data_property_value734.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field735 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value736, _, _) => {
                                            source_ml_source2_data_property_value736.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field737 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value738, _) => {
                                                source_ml_source2_data_property_value738.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field739 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value740) => {
                                                    source_ml_source2_data_property_value740.clone()
                                                }
                                            };
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field721.clone(), source_ml_source2_nominal_field723, source_ml_source2_nominal_field725, source_ml_source2_nominal_field727, source_ml_source2_nominal_field729, error.to_owned(), source_ml_source2_nominal_field733.clone(), source_ml_source2_nominal_field735.clone(), source_ml_source2_nominal_field737.clone(), source_ml_source2_nominal_field739.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }), 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_call_source_instruction(builder: self::JavaScriptCallSourceBuilder, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallSourceBuilder {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value743, _, _, _, _) => {
            source_ml_source2_data_property_value743.clone()
        }
    }) != "") {
        builder.clone()
    } else {
        if (({ let ml_reference_array10 = match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value745, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value745.clone()
            }
        }; let ml_reference_projection10 = ml_reference_array10.borrow().len() as f64; ml_reference_projection10 }) >= (65536f64)) {
            {
                let source_ml_source2_nominal_field747 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value748, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value748.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field749 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value750, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value750
                        }
                    };
                    {
                        let source_ml_source2_nominal_field751 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value752, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value752
                            }
                        };
                        {
                            let source_ml_source2_nominal_field753 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value754, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value754
                                }
                            };
                            {
                                let source_ml_source2_nominal_field755 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value756, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value756
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field757 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value758, _, _, _, _) => {
                                            source_ml_source2_data_property_value758.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field759 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value760, _, _, _) => {
                                                source_ml_source2_data_property_value760.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field761 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value762, _, _) => {
                                                    source_ml_source2_data_property_value762.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field763 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value764, _) => {
                                                        source_ml_source2_data_property_value764.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field765 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value766) => {
                                                            source_ml_source2_data_property_value766.clone()
                                                        }
                                                    };
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field747.clone(), source_ml_source2_nominal_field749, source_ml_source2_nominal_field751, source_ml_source2_nominal_field753, source_ml_source2_nominal_field755, String::from("source lowering exceeds its instruction budget"), source_ml_source2_nominal_field759.clone(), source_ml_source2_nominal_field761.clone(), source_ml_source2_nominal_field763.clone(), source_ml_source2_nominal_field765.clone())
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
        } else {
            {
                let source_ml_source2_nominal_field772 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value773, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value773.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field774 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value775, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value775
                        }
                    };
                    {
                        let source_ml_source2_nominal_field776 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value777, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value777
                            }
                        };
                        {
                            let source_ml_source2_nominal_field778 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value779, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value779
                                }
                            };
                            {
                                let source_ml_source2_nominal_field780 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value781, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value781
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field782 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value783, _, _, _, _) => {
                                            source_ml_source2_data_property_value783.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field784 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value785, _, _, _) => {
                                                source_ml_source2_data_property_value785.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field786 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value787, _, _) => {
                                                    source_ml_source2_data_property_value787.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field788 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value789, _) => {
                                                        source_ml_source2_data_property_value789.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field790 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value791) => {
                                                            source_ml_source2_data_property_value791.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override792 = std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_concat_value769 = match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value768, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value768.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_concat_value770 = std::rc::Rc::new(std::cell::RefCell::new(vec![instruction.clone()]));
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), { let ml_reference_value11 = source_ml_source2_concat_value769.borrow().clone(); ml_reference_value11 }), { let ml_reference_value12 = source_ml_source2_concat_value770.borrow().clone(); ml_reference_value12 })
                                                            }
                                                        }));
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override792.clone(), source_ml_source2_nominal_field774, source_ml_source2_nominal_field776, source_ml_source2_nominal_field778, source_ml_source2_nominal_field780, source_ml_source2_nominal_field782.clone(), source_ml_source2_nominal_field784.clone(), source_ml_source2_nominal_field786.clone(), source_ml_source2_nominal_field788.clone(), source_ml_source2_nominal_field790.clone())
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
pub fn allocate_java_script_call_source_register(builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value794, _, _, _, _) => {
            source_ml_source2_data_property_value794.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value796, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value796
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_field799 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value800, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value800.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field801 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value802, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value802
                        }
                    };
                    {
                        let source_ml_source2_nominal_field803 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value804, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value804
                            }
                        };
                        {
                            let source_ml_source2_nominal_field805 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value806, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value806
                                }
                            };
                            {
                                let source_ml_source2_nominal_field807 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value808, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value808
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field809 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value810, _, _, _, _) => {
                                            source_ml_source2_data_property_value810.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field811 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value812, _, _, _) => {
                                                source_ml_source2_data_property_value812.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field813 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value814, _, _) => {
                                                    source_ml_source2_data_property_value814.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field815 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value816, _) => {
                                                        source_ml_source2_data_property_value816.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field817 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value818) => {
                                                            source_ml_source2_data_property_value818.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override819 = (match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value797, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value797
                                                            }
                                                        } + 1f64);
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field799.clone(), source_ml_source2_nominal_override819, source_ml_source2_nominal_field803, source_ml_source2_nominal_field805, source_ml_source2_nominal_field807, source_ml_source2_nominal_field809.clone(), source_ml_source2_nominal_field811.clone(), source_ml_source2_nominal_field813.clone(), source_ml_source2_nominal_field815.clone(), source_ml_source2_nominal_field817.clone())
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
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value820, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value820
                }
            })
        }
    }
}
