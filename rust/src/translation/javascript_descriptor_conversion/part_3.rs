// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result;
use super::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result;
use super::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result;
use super::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result;
use super::create_java_script_property_descriptor;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_find_java_script_ordinary_object_loop5;
use super::source_ml_temporary_find_java_script_ordinary_property_loop7;
use super::source_ml_temporary_locate_java_script_ordinary_property_loop18;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop11;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop12;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop13;
use super::source_ml_temporary_test_java_script_ordinary_symbol_loop6;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_object(
    objects: Vec<self::JavaScriptOrdinaryObject>,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = objects.len() as f64;
            {
                let source_ml_source1_s151 =
                    self::source_ml_temporary_find_java_script_ordinary_object_loop5(
                        objects,
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s151 {
                    self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done => {
                        -1f64
                    }
                    self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(source_ml_temporary_result5) => {
                        source_ml_temporary_result5
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_symbol(
    symbols: Vec<self::ml::Big>,
    identity: self::ml::Big,
) -> bool {
    {
        let low = 0f64;
        {
            let high = symbols.len() as f64;
            {
                let source_ml_source1_s152 =
                    self::source_ml_temporary_test_java_script_ordinary_symbol_loop6(
                        symbols,
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s152 {
                    self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(source_ml_temporary_result6) => {
                        source_ml_temporary_result6
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_value_error(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> String {
    {
        let error = self::read_java_script_semantic_value_error(value.clone());
        if !(error).is_empty() {
            error.clone()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => String::new(),
                self::JavaScriptSemanticValue::Null => String::new(),
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f153) => {
                    let source_ml_source1_f169 = source_ml_source1_f153;
                    String::new()
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f154) => {
                    let source_ml_source1_f175 = source_ml_source1_f154;
                    String::new()
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f155) => {
                    let source_ml_source1_f181 = source_ml_source1_f155.clone();
                    String::new()
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f156) => {
                    let source_ml_source1_f187 = source_ml_source1_f156;
                    String::new()
                }
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    if self::test_java_script_ordinary_symbol(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                source_ml_source1_data_property_value189,
                            ) => source_ml_source1_data_property_value189,
                        },
                        identity.clone(),
                    ) {
                        String::new()
                    } else {
                        String::from(
                            "a semantic Symbol must belong to the complete global identity directory",
                        )
                    }
                }
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f157,
                    source_ml_source1_f158,
                ) => {
                    let identity_2 = source_ml_source1_f157.clone();
                    {
                        let callable = source_ml_source1_f158;
                        {
                            let at = self::find_java_script_ordinary_object(
                                match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source1_data_property_value195,
                                        _,
                                    ) => source_ml_source1_data_property_value195,
                                },
                                identity_2,
                            );
                            if at < 0f64 {
                                String::from(
                                    "a semantic Object must belong to the complete global identity directory",
                                )
                            } else {
                                {
                                    let object = self::ml_array::optional_at(
                                        (match &heap {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                source_ml_source1_data_property_value196,
                                                _,
                                            ) => {
                                                source_ml_source1_data_property_value196.as_slice()
                                            }
                                        }),
                                        self::ml_array::number_index(at),
                                    );
                                    if !(match (object.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) && (({
                                        let source_ml_source1_nullish_field_receiver200 =
                                            (object.clone(),);
                                        if match (source_ml_source1_nullish_field_receiver200
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            {
                                                let source_ml_source1_nullish_field_present201 =
                                                    ((source_ml_source1_nullish_field_receiver200
                                                        .clone())
                                                    .0)
                                                        .clone()
                                                        .expect("a proven present value");
                                                match source_ml_source1_nullish_field_present201 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value202, _) => {
                                                            source_ml_source1_data_property_value202
                                                        }
                                                    }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'callable')")).encode_utf16().collect(), message_present: true }).raise()
                                        }
                                    }) == callable)
                                    {
                                        String::new()
                                    } else {
                                        String::from(
                                            "Object callability must agree with its intrinsic global identity",
                                        )
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
pub fn read_java_script_ordinary_key_error(
    heap: self::JavaScriptOrdinaryHeap,
    key: self::JavaScriptSemanticValue,
) -> String {
    if match key.clone() {
        self::JavaScriptSemanticValue::Undefined => true,
        self::JavaScriptSemanticValue::Null => true,
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f203) => true,
        self::JavaScriptSemanticValue::Number(source_ml_source1_f204) => true,
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f205) => true,
        self::JavaScriptSemanticValue::Text(source_ml_source1_f206) => false,
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f207) => true,
        self::JavaScriptSemanticValue::Object(source_ml_source1_f208, source_ml_source1_f209) => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Undefined => true,
        self::JavaScriptSemanticValue::Null => true,
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f210) => true,
        self::JavaScriptSemanticValue::Number(source_ml_source1_f211) => true,
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f212) => true,
        self::JavaScriptSemanticValue::Text(source_ml_source1_f213) => true,
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f214) => false,
        self::JavaScriptSemanticValue::Object(source_ml_source1_f215, source_ml_source1_f216) => {
            true
        }
    } {
        String::from("an ordinary property key must be text or Symbol without implicit coercion")
    } else {
        self::read_java_script_ordinary_value_error(heap.clone(), key.clone())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_property(
    properties: Vec<self::JavaScriptOrdinaryProperty>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s217 =
                self::source_ml_temporary_find_java_script_ordinary_property_loop7(
                    properties,
                    key.clone(),
                    at,
                );
            match source_ml_source1_s217 {
                self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Done => {
                    -1f64
                }
                self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Return(source_ml_temporary_result7) => {
                    source_ml_temporary_result7
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_heap_error(heap: self::JavaScriptOrdinaryHeap) -> String {
    if (((match &heap {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value219, _) => {
            source_ml_source1_data_property_value219.as_slice()
        }
    })
    .len() as f64)
        > 4096f64)
        || (((match &heap {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                _,
                source_ml_source1_data_property_value221,
            ) => source_ml_source1_data_property_value221.as_slice(),
        })
        .len() as f64)
            > 16_384f64)
    {
        String::from("ordinary heap identity directory exceeds its explicit resource budget")
    } else {
        {
            let previous = self::ml::Big::from_i128(0);
            {
                let source_ml_temporary_values8 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value222,
                        _,
                    ) => source_ml_source1_data_property_value222,
                };
                {
                    let source_ml_temporary_index8 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s223 =
                            self::source_ml_temporary_read_java_script_ordinary_heap_error_loop11(
                                previous,
                                source_ml_temporary_values8,
                                source_ml_temporary_index8,
                            );
                        match source_ml_source1_s223 {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(previous_2) => {
                                {
                                    let previous_3 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_temporary_values9 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value224) => {
                                                source_ml_source1_data_property_value224
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index9 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s225 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop12(heap.clone(), previous_3, source_ml_temporary_values9, source_ml_temporary_index9);
                                                match source_ml_source1_s225 {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done => {
                                                        {
                                                            let property_count = 0f64;
                                                            {
                                                                let source_ml_temporary_values10 = match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value226, _) => {
                                                                        source_ml_source1_data_property_value226
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_temporary_index10 = self::ml::Big::from_i128(0);
                                                                    {
                                                                        let source_ml_source1_s227 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop13(heap.clone(), property_count, source_ml_temporary_values10, source_ml_temporary_index10);
                                                                        match source_ml_source1_s227 {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done => {
                                                                                String::new()
                                                                            }
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_temporary_result13) => {
                                                                                source_ml_temporary_result13.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(source_ml_temporary_result17) => {
                                                        source_ml_temporary_result17.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(source_ml_temporary_result17_2) => {
                                source_ml_temporary_result17_2.clone()
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
pub fn replace_java_script_ordinary_object(
    heap: self::JavaScriptOrdinaryHeap,
    replacement: self::JavaScriptOrdinaryObject,
) -> self::JavaScriptOrdinaryHeap {
    {
        let objects = {
            let ml_values1 = match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    source_ml_source1_data_property_value228,
                    _,
                ) => source_ml_source1_data_property_value228,
            };
            let ml_callback1 = {
                let replacement = replacement.clone();
                std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if (match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value231, _, _, _, _) => {
                source_ml_source1_data_property_value231.clone()
            }
        }) == (match replacement.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value232, _, _, _, _) => {
                source_ml_source1_data_property_value232.clone()
            }
        }) {
            replacement.clone()
        } else {
            object.clone()
        } })
            };
            ml_values1
                .iter()
                .cloned()
                .enumerate()
                .map(|(ml_index1, ml_value1)| ml_callback1(ml_value1))
                .collect::<Vec<_>>()
        };
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            objects,
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    _,
                    source_ml_source1_data_property_value233,
                ) => source_ml_source1_data_property_value233,
            },
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn locate_java_script_ordinary_property(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
) -> (
    (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
    bool,
    self::ml::Big,
) {
    {
        let current = identity.clone();
        {
            let depth = 0f64;
            {
                let source_ml_source1_s234 =
                    self::source_ml_temporary_locate_java_script_ordinary_property_loop18(
                        heap.clone(),
                        key.clone(),
                        current,
                        depth,
                    );
                match source_ml_source1_s234 {
                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done => {
                        {
                            let source_ml_source1_field235 = false;
                            {
                                let source_ml_source1_field236 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_field237 = self::create_java_script_property_descriptor("data", false, false);
                                    (source_ml_source1_field237, source_ml_source1_field235, source_ml_source1_field236)
                                }
                            }
                        }
                    }
                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Return(source_ml_temporary_result18) => {
                        source_ml_temporary_result18.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_get(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
) -> (
    String,
    Vec<self::JavaScriptSemanticValue>,
    self::JavaScriptSemanticValue,
    String,
    self::JavaScriptSemanticValue,
    self::JavaScriptSemanticValue,
) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let receiver_error =
                    self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                {
                    let error = if !(heap_error).is_empty() {
                        heap_error.clone()
                    } else if !(key_error).is_empty() {
                        key_error.clone()
                    } else if !(receiver_error).is_empty() {
                        receiver_error.clone()
                    } else if (self::find_java_script_ordinary_object(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value239,
                                _,
                            ) => source_ml_source1_data_property_value239,
                        },
                        identity.clone(),
                    )) < 0f64
                    {
                        String::from("an ordinary Get needs its actual Object identity")
                    } else {
                        String::new()
                    };
                    {
                        let ml_library_inequality_1 = error != "";
                        if ml_library_inequality_1 {
                            {
                                let source_ml_source1_field240 = String::from("error");
                                {
                                    let source_ml_source1_field241 = error.clone();
                                    {
                                        let source_ml_source1_field242 =
                                            self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source1_field243 =
                                                self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field244 = receiver.clone();
                                                {
                                                    let source_ml_source1_field245 =
                                                        Vec::<self::JavaScriptSemanticValue>::new();
                                                    (
                                                        source_ml_source1_field240,
                                                        source_ml_source1_field245,
                                                        source_ml_source1_field243,
                                                        source_ml_source1_field241,
                                                        source_ml_source1_field244,
                                                        source_ml_source1_field242,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let property = self::locate_java_script_ordinary_property(
                                    heap.clone(),
                                    identity.clone(),
                                    key.clone(),
                                );
                                if ((property.clone()).1) {
                                    if ((property.clone()).0).9 {
                                        {
                                            let source_ml_source1_field252 = String::from("value");
                                            {
                                                let source_ml_source1_field253 = String::new();
                                                {
                                                    let source_ml_source1_field254 =
                                                        ((property.clone()).0).8;
                                                    {
                                                        let source_ml_source1_field255 = self::JavaScriptSemanticValue::Undefined;
                                                        {
                                                            let source_ml_source1_field256 =
                                                                receiver.clone();
                                                            {
                                                                let source_ml_source1_field257 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                (
                                                                    source_ml_source1_field252,
                                                                    source_ml_source1_field257,
                                                                    source_ml_source1_field255,
                                                                    source_ml_source1_field253,
                                                                    source_ml_source1_field256,
                                                                    source_ml_source1_field254,
                                                                )
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let getter = ((property.clone()).0).4;
                                            match getter.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    let source_ml_source1_field265 =
                                                        String::from("value");
                                                    {
                                                        let source_ml_source1_field266 =
                                                            String::new();
                                                        {
                                                            let source_ml_source1_field267 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field268 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field269 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field270 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field265, source_ml_source1_field270, source_ml_source1_field268, source_ml_source1_field266, source_ml_source1_field269, source_ml_source1_field267)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    let source_ml_source1_field271 =
                                                        String::from("call");
                                                    {
                                                        let source_ml_source1_field272 =
                                                            String::new();
                                                        {
                                                            let source_ml_source1_field273 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field274 =
                                                                    getter.clone();
                                                                {
                                                                    let source_ml_source1_field275 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field276 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field271, source_ml_source1_field276, source_ml_source1_field274, source_ml_source1_field272, source_ml_source1_field275, source_ml_source1_field273)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(
                                                    source_ml_source1_f258,
                                                ) => {
                                                    let source_ml_source1_field277 =
                                                        String::from("call");
                                                    {
                                                        let source_ml_source1_field278 =
                                                            String::new();
                                                        {
                                                            let source_ml_source1_field279 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field280 =
                                                                    getter.clone();
                                                                {
                                                                    let source_ml_source1_field281 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field282 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field277, source_ml_source1_field282, source_ml_source1_field280, source_ml_source1_field278, source_ml_source1_field281, source_ml_source1_field279)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(
                                                    source_ml_source1_f259,
                                                ) => {
                                                    let source_ml_source1_field283 =
                                                        String::from("call");
                                                    {
                                                        let source_ml_source1_field284 =
                                                            String::new();
                                                        {
                                                            let source_ml_source1_field285 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field286 =
                                                                    getter.clone();
                                                                {
                                                                    let source_ml_source1_field287 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field288 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field283, source_ml_source1_field288, source_ml_source1_field286, source_ml_source1_field284, source_ml_source1_field287, source_ml_source1_field285)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(
                                                    source_ml_source1_f260,
                                                ) => {
                                                    let source_ml_source1_field289 =
                                                        String::from("call");
                                                    {
                                                        let source_ml_source1_field290 =
                                                            String::new();
                                                        {
                                                            let source_ml_source1_field291 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field292 =
                                                                    getter.clone();
                                                                {
                                                                    let source_ml_source1_field293 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field294 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field289, source_ml_source1_field294, source_ml_source1_field292, source_ml_source1_field290, source_ml_source1_field293, source_ml_source1_field291)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(
                                                    source_ml_source1_f261,
                                                ) => {
                                                    let source_ml_source1_field295 =
                                                        String::from("call");
                                                    {
                                                        let source_ml_source1_field296 =
                                                            String::new();
                                                        {
                                                            let source_ml_source1_field297 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field298 =
                                                                    getter.clone();
                                                                {
                                                                    let source_ml_source1_field299 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field300 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field295, source_ml_source1_field300, source_ml_source1_field298, source_ml_source1_field296, source_ml_source1_field299, source_ml_source1_field297)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(
                                                    source_ml_source1_f262,
                                                ) => {
                                                    let source_ml_source1_field301 =
                                                        String::from("call");
                                                    {
                                                        let source_ml_source1_field302 =
                                                            String::new();
                                                        {
                                                            let source_ml_source1_field303 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field304 =
                                                                    getter.clone();
                                                                {
                                                                    let source_ml_source1_field305 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field306 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field301, source_ml_source1_field306, source_ml_source1_field304, source_ml_source1_field302, source_ml_source1_field305, source_ml_source1_field303)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(
                                                    source_ml_source1_f263,
                                                    source_ml_source1_f264,
                                                ) => {
                                                    let source_ml_source1_field307 =
                                                        String::from("call");
                                                    {
                                                        let source_ml_source1_field308 =
                                                            String::new();
                                                        {
                                                            let source_ml_source1_field309 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_field310 =
                                                                    getter.clone();
                                                                {
                                                                    let source_ml_source1_field311 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field312 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field307, source_ml_source1_field312, source_ml_source1_field310, source_ml_source1_field308, source_ml_source1_field311, source_ml_source1_field309)
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
                                        let source_ml_source1_field246 = String::from("value");
                                        {
                                            let source_ml_source1_field247 = String::new();
                                            {
                                                let source_ml_source1_field248 =
                                                    self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source1_field249 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source1_field250 =
                                                            receiver.clone();
                                                        {
                                                            let source_ml_source1_field251 = Vec::<
                                                                self::JavaScriptSemanticValue,
                                                            >::new(
                                                            );
                                                            (
                                                                source_ml_source1_field246,
                                                                source_ml_source1_field251,
                                                                source_ml_source1_field249,
                                                                source_ml_source1_field247,
                                                                source_ml_source1_field250,
                                                                source_ml_source1_field248,
                                                            )
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
