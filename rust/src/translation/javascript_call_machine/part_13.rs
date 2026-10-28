// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_set_java_script_ordinary_prototype_loop112;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn set_java_script_ordinary_prototype(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, prototype: self::JavaScriptSemanticValue) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source2_field565 = heap.clone();
                {
                    let source_ml_source2_field566 = false;
                    {
                        let source_ml_source2_field567 = heap_error.clone();
                        (source_ml_source2_field567.clone(), source_ml_source2_field565.clone(), source_ml_source2_field566)
                    }
                }
            }
        } else {
            if (match prototype.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    true
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f568) => {
                    true
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f569) => {
                    true
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f570) => {
                    true
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f571) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f572) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f573, source_ml_source2_f574) => {
                    true
                }
            } && match prototype.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    true
                }
                self::JavaScriptSemanticValue::Null => {
                    true
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f575) => {
                    true
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f576) => {
                    true
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f577) => {
                    true
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f578) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f579) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f580, source_ml_source2_f581) => {
                    false
                }
            }) {
                {
                    let source_ml_source2_field582 = heap.clone();
                    {
                        let source_ml_source2_field583 = false;
                        {
                            let source_ml_source2_field584 = self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]);
                            (source_ml_source2_field584.clone(), source_ml_source2_field582.clone(), source_ml_source2_field583)
                        }
                    }
                }
            } else {
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let source_ml_source2_field585 = heap.clone();
                            {
                                let source_ml_source2_field586 = false;
                                {
                                    let source_ml_source2_field587 = error.clone();
                                    (source_ml_source2_field587.clone(), source_ml_source2_field585.clone(), source_ml_source2_field586)
                                }
                            }
                        }
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value588, _) => {
                                    source_ml_source2_data_property_value588
                                }
                            }, identity.clone());
                            {
                                let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value589, _) => source_ml_source2_data_property_value589.as_slice() }), self::ml_array::number_index(index));
                                if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    {
                                        let source_ml_source2_field590 = heap.clone();
                                        {
                                            let source_ml_source2_field591 = false;
                                            {
                                                let source_ml_source2_field592 = self::ml_string::Text::from_units(vec![83, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                                (source_ml_source2_field592.clone(), source_ml_source2_field590.clone(), source_ml_source2_field591)
                                            }
                                        }
                                    }
                                } else {
                                    if self::test_java_script_semantic_same_value(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value593, _, _, _) => {
                                            let source_ml_source2_data_property_value593 = *source_ml_source2_data_property_value593;
                                            source_ml_source2_data_property_value593.clone()
                                        }
                                    }, prototype.clone()) {
                                        {
                                            let source_ml_source2_field594 = heap.clone();
                                            {
                                                let source_ml_source2_field595 = true;
                                                {
                                                    let source_ml_source2_field596 = self::ml_string::Text::from_units(vec![]);
                                                    (source_ml_source2_field596.clone(), source_ml_source2_field594.clone(), source_ml_source2_field595)
                                                }
                                            }
                                        }
                                    } else {
                                        if !(match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value597, _, _) => {
                                                source_ml_source2_data_property_value597
                                            }
                                        }) {
                                            {
                                                let source_ml_source2_field598 = heap.clone();
                                                {
                                                    let source_ml_source2_field599 = false;
                                                    {
                                                        let source_ml_source2_field600 = self::ml_string::Text::from_units(vec![]);
                                                        (source_ml_source2_field600.clone(), source_ml_source2_field598.clone(), source_ml_source2_field599)
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let next = prototype.clone();
                                                {
                                                    let depth = 0f64;
                                                    {
                                                        let source_ml_source2_s601 = self::source_ml_source1_temporary_set_java_script_ordinary_prototype_loop112(heap.clone(), identity.clone(), next.clone(), depth);
                                                        match source_ml_source2_s601.clone() {
                                                            self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Done => {
                                                                {
                                                                    let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value602, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value602.clone()
                                                                        }
                                                                    }, Box::new(prototype.clone()), match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value603, _, _) => {
                                                                            source_ml_source2_data_property_value603
                                                                        }
                                                                    }, match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value604, _) => {
                                                                            source_ml_source2_data_property_value604
                                                                        }
                                                                    }, match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value605) => {
                                                                            source_ml_source2_data_property_value605
                                                                        }
                                                                    });
                                                                    {
                                                                        let source_ml_source2_field606 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                        {
                                                                            let source_ml_source2_field607 = true;
                                                                            {
                                                                                let source_ml_source2_field608 = self::ml_string::Text::from_units(vec![]);
                                                                                (source_ml_source2_field608.clone(), source_ml_source2_field606.clone(), source_ml_source2_field607)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Return(source_ml_source1_temporary_result112) => {
                                                                source_ml_source1_temporary_result112.clone()
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
pub fn create_java_script_ordinary_set_descriptor(value: self::JavaScriptSemanticValue, creation: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field609 = creation;
        {
            let source_ml_source2_field610 = creation;
            {
                let source_ml_source2_field611 = creation;
                {
                    let source_ml_source2_field612 = creation;
                    {
                        let source_ml_source2_field613 = true;
                        {
                            let source_ml_source2_field614 = value.clone();
                            {
                                let source_ml_source2_field615 = creation;
                                {
                                    let source_ml_source2_field616 = creation;
                                    {
                                        let source_ml_source2_field617 = false;
                                        {
                                            let source_ml_source2_field618 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field619 = false;
                                                {
                                                    let source_ml_source2_field620 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field610, source_ml_source2_field609, source_ml_source2_field612, source_ml_source2_field611, source_ml_source2_field618.clone(), source_ml_source2_field617, source_ml_source2_field620.clone(), source_ml_source2_field619, source_ml_source2_field614.clone(), source_ml_source2_field613, source_ml_source2_field616, source_ml_source2_field615)
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
pub fn plan_java_script_ordinary_set(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue, value: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>, self::JavaScriptSemanticValue, self::ml_string::Text, self::JavaScriptOrdinaryHeap, self::JavaScriptSemanticValue, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let value_error = self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                    {
                        let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                            heap_error.clone()
                        } else {
                            if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                key_error.clone()
                            } else {
                                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                    value_error.clone()
                                } else {
                                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                        receiver_error.clone()
                                    } else {
                                        if ((self::find_java_script_ordinary_object(match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value622, _) => {
                                                source_ml_source2_data_property_value622
                                            }
                                        }, identity.clone())) < (0f64)) {
                                            self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                    }
                                }
                            }
                        };
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            {
                                let source_ml_source2_field623 = heap.clone();
                                {
                                    let source_ml_source2_field624 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                    {
                                        let source_ml_source2_field625 = error.clone();
                                        {
                                            let source_ml_source2_field626 = false;
                                            {
                                                let source_ml_source2_field627 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source2_field628 = receiver.clone();
                                                    {
                                                        let source_ml_source2_field629 = Vec::<self::JavaScriptSemanticValue>::new();
                                                        (source_ml_source2_field624.clone(), source_ml_source2_field629, source_ml_source2_field627.clone(), source_ml_source2_field625.clone(), source_ml_source2_field623.clone(), source_ml_source2_field628.clone(), source_ml_source2_field626)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let located = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                {
                                    let own = if (located.clone()).1 {
                                        (located.clone()).0
                                    } else {
                                        self::create_java_script_ordinary_set_descriptor(self::JavaScriptSemanticValue::Undefined, true)
                                    };
                                    if (own.clone()).5 {
                                        {
                                            let setter = (own.clone()).6;
                                            match setter.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    {
                                                        let source_ml_source2_field637 = heap.clone();
                                                        {
                                                            let source_ml_source2_field638 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field639 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field640 = false;
                                                                    {
                                                                        let source_ml_source2_field641 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field642 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field643 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field638.clone(), source_ml_source2_field643, source_ml_source2_field641.clone(), source_ml_source2_field639.clone(), source_ml_source2_field637.clone(), source_ml_source2_field642.clone(), source_ml_source2_field640)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    {
                                                        let source_ml_source2_field644 = heap.clone();
                                                        {
                                                            let source_ml_source2_field645 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field646 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field647 = false;
                                                                    {
                                                                        let source_ml_source2_field648 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field649 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field650 = vec![value.clone()];
                                                                                (source_ml_source2_field645.clone(), source_ml_source2_field650, source_ml_source2_field648.clone(), source_ml_source2_field646.clone(), source_ml_source2_field644.clone(), source_ml_source2_field649.clone(), source_ml_source2_field647)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f630) => {
                                                    {
                                                        let source_ml_source2_field651 = heap.clone();
                                                        {
                                                            let source_ml_source2_field652 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field653 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field654 = false;
                                                                    {
                                                                        let source_ml_source2_field655 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field656 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field657 = vec![value.clone()];
                                                                                (source_ml_source2_field652.clone(), source_ml_source2_field657, source_ml_source2_field655.clone(), source_ml_source2_field653.clone(), source_ml_source2_field651.clone(), source_ml_source2_field656.clone(), source_ml_source2_field654)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source2_f631) => {
                                                    {
                                                        let source_ml_source2_field658 = heap.clone();
                                                        {
                                                            let source_ml_source2_field659 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field660 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field661 = false;
                                                                    {
                                                                        let source_ml_source2_field662 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field663 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field664 = vec![value.clone()];
                                                                                (source_ml_source2_field659.clone(), source_ml_source2_field664, source_ml_source2_field662.clone(), source_ml_source2_field660.clone(), source_ml_source2_field658.clone(), source_ml_source2_field663.clone(), source_ml_source2_field661)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f632) => {
                                                    {
                                                        let source_ml_source2_field665 = heap.clone();
                                                        {
                                                            let source_ml_source2_field666 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field667 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field668 = false;
                                                                    {
                                                                        let source_ml_source2_field669 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field670 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field671 = vec![value.clone()];
                                                                                (source_ml_source2_field666.clone(), source_ml_source2_field671, source_ml_source2_field669.clone(), source_ml_source2_field667.clone(), source_ml_source2_field665.clone(), source_ml_source2_field670.clone(), source_ml_source2_field668)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f633) => {
                                                    {
                                                        let source_ml_source2_field672 = heap.clone();
                                                        {
                                                            let source_ml_source2_field673 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field674 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field675 = false;
                                                                    {
                                                                        let source_ml_source2_field676 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field677 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field678 = vec![value.clone()];
                                                                                (source_ml_source2_field673.clone(), source_ml_source2_field678, source_ml_source2_field676.clone(), source_ml_source2_field674.clone(), source_ml_source2_field672.clone(), source_ml_source2_field677.clone(), source_ml_source2_field675)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f634) => {
                                                    {
                                                        let source_ml_source2_field679 = heap.clone();
                                                        {
                                                            let source_ml_source2_field680 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field681 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field682 = false;
                                                                    {
                                                                        let source_ml_source2_field683 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field684 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field685 = vec![value.clone()];
                                                                                (source_ml_source2_field680.clone(), source_ml_source2_field685, source_ml_source2_field683.clone(), source_ml_source2_field681.clone(), source_ml_source2_field679.clone(), source_ml_source2_field684.clone(), source_ml_source2_field682)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source2_f635, source_ml_source2_f636) => {
                                                    {
                                                        let source_ml_source2_field686 = heap.clone();
                                                        {
                                                            let source_ml_source2_field687 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source2_field688 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field689 = false;
                                                                    {
                                                                        let source_ml_source2_field690 = setter.clone();
                                                                        {
                                                                            let source_ml_source2_field691 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field692 = vec![value.clone()];
                                                                                (source_ml_source2_field687.clone(), source_ml_source2_field692, source_ml_source2_field690.clone(), source_ml_source2_field688.clone(), source_ml_source2_field686.clone(), source_ml_source2_field691.clone(), source_ml_source2_field689)
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
                                        if !((own.clone()).10) {
                                            {
                                                let source_ml_source2_field693 = heap.clone();
                                                {
                                                    let source_ml_source2_field694 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                    {
                                                        let source_ml_source2_field695 = self::ml_string::Text::from_units(vec![]);
                                                        {
                                                            let source_ml_source2_field696 = false;
                                                            {
                                                                let source_ml_source2_field697 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field698 = receiver.clone();
                                                                    {
                                                                        let source_ml_source2_field699 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source2_field694.clone(), source_ml_source2_field699, source_ml_source2_field697.clone(), source_ml_source2_field695.clone(), source_ml_source2_field693.clone(), source_ml_source2_field698.clone(), source_ml_source2_field696)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            match receiver.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    {
                                                        let source_ml_source2_field706 = heap.clone();
                                                        {
                                                            let source_ml_source2_field707 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field708 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field709 = false;
                                                                    {
                                                                        let source_ml_source2_field710 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field711 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field712 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field707.clone(), source_ml_source2_field712, source_ml_source2_field710.clone(), source_ml_source2_field708.clone(), source_ml_source2_field706.clone(), source_ml_source2_field711.clone(), source_ml_source2_field709)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    {
                                                        let source_ml_source2_field713 = heap.clone();
                                                        {
                                                            let source_ml_source2_field714 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field715 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field716 = false;
                                                                    {
                                                                        let source_ml_source2_field717 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field718 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field719 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field714.clone(), source_ml_source2_field719, source_ml_source2_field717.clone(), source_ml_source2_field715.clone(), source_ml_source2_field713.clone(), source_ml_source2_field718.clone(), source_ml_source2_field716)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f700) => {
                                                    {
                                                        let source_ml_source2_field720 = heap.clone();
                                                        {
                                                            let source_ml_source2_field721 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field722 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field723 = false;
                                                                    {
                                                                        let source_ml_source2_field724 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field725 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field726 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field721.clone(), source_ml_source2_field726, source_ml_source2_field724.clone(), source_ml_source2_field722.clone(), source_ml_source2_field720.clone(), source_ml_source2_field725.clone(), source_ml_source2_field723)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source2_f701) => {
                                                    {
                                                        let source_ml_source2_field727 = heap.clone();
                                                        {
                                                            let source_ml_source2_field728 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field729 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field730 = false;
                                                                    {
                                                                        let source_ml_source2_field731 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field732 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field733 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field728.clone(), source_ml_source2_field733, source_ml_source2_field731.clone(), source_ml_source2_field729.clone(), source_ml_source2_field727.clone(), source_ml_source2_field732.clone(), source_ml_source2_field730)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f702) => {
                                                    {
                                                        let source_ml_source2_field734 = heap.clone();
                                                        {
                                                            let source_ml_source2_field735 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field736 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field737 = false;
                                                                    {
                                                                        let source_ml_source2_field738 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field739 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field740 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field735.clone(), source_ml_source2_field740, source_ml_source2_field738.clone(), source_ml_source2_field736.clone(), source_ml_source2_field734.clone(), source_ml_source2_field739.clone(), source_ml_source2_field737)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f703) => {
                                                    {
                                                        let source_ml_source2_field741 = heap.clone();
                                                        {
                                                            let source_ml_source2_field742 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field743 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field744 = false;
                                                                    {
                                                                        let source_ml_source2_field745 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field746 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field747 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field742.clone(), source_ml_source2_field747, source_ml_source2_field745.clone(), source_ml_source2_field743.clone(), source_ml_source2_field741.clone(), source_ml_source2_field746.clone(), source_ml_source2_field744)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f704) => {
                                                    {
                                                        let source_ml_source2_field748 = heap.clone();
                                                        {
                                                            let source_ml_source2_field749 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                            {
                                                                let source_ml_source2_field750 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source2_field751 = false;
                                                                    {
                                                                        let source_ml_source2_field752 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field753 = receiver.clone();
                                                                            {
                                                                                let source_ml_source2_field754 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source2_field749.clone(), source_ml_source2_field754, source_ml_source2_field752.clone(), source_ml_source2_field750.clone(), source_ml_source2_field748.clone(), source_ml_source2_field753.clone(), source_ml_source2_field751)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f705) => {
                                                    {
                                                        let index = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value755, _) => {
                                                                source_ml_source2_data_property_value755
                                                            }
                                                        }, identity_2.clone());
                                                        {
                                                            let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value756, _) => source_ml_source2_data_property_value756.as_slice() }), self::ml_array::number_index(index));
                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                {
                                                                    let source_ml_source2_field757 = heap.clone();
                                                                    {
                                                                        let source_ml_source2_field758 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                        {
                                                                            let source_ml_source2_field759 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                            {
                                                                                let source_ml_source2_field760 = false;
                                                                                {
                                                                                    let source_ml_source2_field761 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source2_field762 = receiver.clone();
                                                                                        {
                                                                                            let source_ml_source2_field763 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                            (source_ml_source2_field758.clone(), source_ml_source2_field763, source_ml_source2_field761.clone(), source_ml_source2_field759.clone(), source_ml_source2_field757.clone(), source_ml_source2_field762.clone(), source_ml_source2_field760)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value764) => {
                                                                            source_ml_source2_data_property_value764
                                                                        }
                                                                    }, key.clone());
                                                                    {
                                                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value765) => {
                                                                                source_ml_source2_data_property_value765
                                                                            }
                                                                        }, self::ml_array::number_index(at));
                                                                        if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                            let source_ml_source2_nullish_field_receiver766 = (property.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver766.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present767 = ((source_ml_source2_nullish_field_receiver766.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source2_nullish_field_present767.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value768) => {
                                                                                            source_ml_source2_data_property_value768.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).9) || !(({
                                                                            let source_ml_source2_nullish_field_receiver769 = (property.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver769.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present770 = ((source_ml_source2_nullish_field_receiver769.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source2_nullish_field_present770.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value771) => {
                                                                                            source_ml_source2_data_property_value771.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).10))) {
                                                                            {
                                                                                let source_ml_source2_field772 = heap.clone();
                                                                                {
                                                                                    let source_ml_source2_field773 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                    {
                                                                                        let source_ml_source2_field774 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source2_field775 = false;
                                                                                            {
                                                                                                let source_ml_source2_field776 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source2_field777 = receiver.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_field778 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                        (source_ml_source2_field773.clone(), source_ml_source2_field778, source_ml_source2_field776.clone(), source_ml_source2_field774.clone(), source_ml_source2_field772.clone(), source_ml_source2_field777.clone(), source_ml_source2_field775)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let proposed = self::create_java_script_ordinary_set_descriptor(value.clone(), match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false });
                                                                                {
                                                                                    let transition = self::define_java_script_ordinary_property(heap.clone(), identity_2.clone(), key.clone(), proposed.clone());
                                                                                    {
                                                                                        let source_ml_source2_field779 = (transition.clone()).1;
                                                                                        {
                                                                                            let source_ml_source2_field780 = if (((transition.clone()).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_field781 = (transition.clone()).0;
                                                                                                {
                                                                                                    let source_ml_source2_field782 = (transition.clone()).2;
                                                                                                    {
                                                                                                        let source_ml_source2_field783 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source2_field784 = receiver.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_field785 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                (source_ml_source2_field780.clone(), source_ml_source2_field785, source_ml_source2_field783.clone(), source_ml_source2_field781.clone(), source_ml_source2_field779.clone(), source_ml_source2_field784.clone(), source_ml_source2_field782)
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
