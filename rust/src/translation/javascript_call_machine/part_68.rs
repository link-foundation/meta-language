// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArraySliceCall;
use super::JavaScriptArraySliceState;
use super::JavaScriptArraySliceTransition;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::accept_final_set;
use super::accept_scalar;
use super::accept_species;
use super::allocate_java_script_array;
use super::allocation_request;
use super::begin_java_script_scalar_protocol;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_slice_state_error;
use super::read_java_script_ordinary_value_error;
use super::resume_java_script_array_species_getter;
use super::resume_java_script_object_spread;
use super::resume_java_script_scalar_protocol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_slice_call(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArraySliceState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArraySliceTransition {
    match state.clone() {
        self::JavaScriptArraySliceState::ArraySliceState(
            _,
            _,
            _,
            _,
            shared_constructor_field24,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field25,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field24 = *shared_constructor_field24;
            {
                let error = self::read_java_script_array_slice_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if (match (
                        match state.clone() {
                            self::JavaScriptArraySliceState::ArraySliceState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value12762,
                            ) => ((source_ml_source2_data_property_value12762
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || !(self::ml_array::includes(
                        &{
                            let ml_reference_value780 =
                                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    self::ml_string::Text::from_units(vec![
                                        110, 111, 114, 109, 97, 108, 67, 111, 109, 112, 108, 101,
                                        116, 105, 111, 110,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        116, 104, 114, 111, 119, 67, 111, 109, 112, 108, 101, 116,
                                        105, 111, 110,
                                    ]),
                                ])))
                                .borrow()
                                .clone();
                            ml_reference_value780
                        },
                        &match completion.clone() {
                            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(_) => {
                                self::ml_string::Text::from_units(vec![
                                    110, 111, 114, 109, 97, 108, 67, 111, 109, 112, 108, 101, 116,
                                    105, 111, 110,
                                ])
                            }
                            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(_) => {
                                self::ml_string::Text::from_units(vec![
                                    116, 104, 114, 111, 119, 67, 111, 109, 112, 108, 101, 116, 105,
                                    111, 110,
                                ])
                            }
                        },
                        0f64,
                        false,
                        |a, b| a == b,
                    ))) {
                        self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                115, 108, 105, 99, 101, 32, 114, 101, 115, 117, 109, 112, 116, 105,
                                111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                                32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116,
                                101, 100, 32, 99, 97, 108, 108, 32, 97, 110, 100, 32, 99, 111, 109,
                                112, 108, 101, 116, 105, 111, 110, 32, 107, 105, 110, 100,
                            ]),
                        )
                    } else {
                        {
                            let invalid = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value12763) => {
                                    let source_ml_source2_data_property_value12763 = *source_ml_source2_data_property_value12763;
                                    source_ml_source2_data_property_value12763.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value12764) => {
                                    let source_ml_source2_data_property_value12764 = *source_ml_source2_data_property_value12764;
                                    source_ml_source2_data_property_value12764.clone()
                                }
                            });
                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                                    Box::new(heap.clone()),
                                    invalid.clone(),
                                )
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        {
                                            let prior_call = match state.clone() {
                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12765) => {
                                                    ((source_ml_source2_data_property_value12765.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                }
                                            };
                                            {
                                                let base = match state.clone() {
                                                    self::JavaScriptArraySliceState::ArraySliceState(source_ml_source2_nominal_rest_field12766, source_ml_source2_nominal_rest_field12767, source_ml_source2_nominal_rest_field12768, source_ml_source2_nominal_rest_field12769, source_ml_source2_nominal_rest_field12770, source_ml_source2_nominal_rest_field12771, source_ml_source2_nominal_rest_field12772, source_ml_source2_nominal_rest_field12773, source_ml_source2_nominal_rest_field12774, source_ml_source2_nominal_rest_field12775, source_ml_source2_nominal_rest_field12776, source_ml_source2_nominal_rest_field12777, source_ml_source2_nominal_rest_field12778, source_ml_source2_nominal_rest_field12779, source_ml_source2_nominal_rest_field12780, source_ml_source2_nominal_rest_field12781, source_ml_source2_nominal_rest_field12782, source_ml_source2_nominal_rest_field12783, source_ml_source2_nominal_rest_field12784, source_ml_source2_nominal_rest_field12785, source_ml_source2_nominal_rest_field12786, source_ml_source2_nominal_rest_field12787, _) => {
                                                        let source_ml_source2_nominal_rest_field12766 = *source_ml_source2_nominal_rest_field12766;
                                                        let source_ml_source2_nominal_rest_field12767 = *source_ml_source2_nominal_rest_field12767;
                                                        let source_ml_source2_nominal_rest_field12768 = *source_ml_source2_nominal_rest_field12768;
                                                        let source_ml_source2_nominal_rest_field12770 = *source_ml_source2_nominal_rest_field12770;
                                                        let source_ml_source2_nominal_rest_field12777 = *source_ml_source2_nominal_rest_field12777;
                                                        let source_ml_source2_nominal_rest_field12778 = *source_ml_source2_nominal_rest_field12778;
                                                        let source_ml_source2_nominal_rest_field12779 = *source_ml_source2_nominal_rest_field12779;
                                                        self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_rest_field12766.clone()), Box::new(source_ml_source2_nominal_rest_field12767.clone()), Box::new(source_ml_source2_nominal_rest_field12768.clone()), source_ml_source2_nominal_rest_field12769, Box::new(source_ml_source2_nominal_rest_field12770.clone()), source_ml_source2_nominal_rest_field12771, source_ml_source2_nominal_rest_field12772, source_ml_source2_nominal_rest_field12773, source_ml_source2_nominal_rest_field12774, source_ml_source2_nominal_rest_field12775, source_ml_source2_nominal_rest_field12776, Box::new(source_ml_source2_nominal_rest_field12777.clone()), Box::new(source_ml_source2_nominal_rest_field12778.clone()), Box::new(source_ml_source2_nominal_rest_field12779.clone()), source_ml_source2_nominal_rest_field12780.clone(), source_ml_source2_nominal_rest_field12781.clone(), source_ml_source2_nominal_rest_field12782.clone(), source_ml_source2_nominal_rest_field12783.clone(), source_ml_source2_nominal_rest_field12784.clone(), source_ml_source2_nominal_rest_field12785.clone(), source_ml_source2_nominal_rest_field12786.clone(), source_ml_source2_nominal_rest_field12787.clone(), None::<(Option<self::JavaScriptArraySliceCall>,)>)
                                                    }
                                                };
                                                match base.clone() {
                                                    self::JavaScriptArraySliceState::ArraySliceState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14, shared_constructor_field15, shared_constructor_field16, shared_constructor_field17, shared_constructor_field18, shared_constructor_field19, shared_constructor_field20, shared_constructor_field21, shared_constructor_field22, shared_constructor_field23) => {
                                                        let shared_constructor_field1 = *shared_constructor_field1;
                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                        let shared_constructor_field3 = *shared_constructor_field3;
                                                        let shared_constructor_field5 = *shared_constructor_field5;
                                                        let shared_constructor_field12 = *shared_constructor_field12;
                                                        let shared_constructor_field13 = *shared_constructor_field13;
                                                        let shared_constructor_field14 = *shared_constructor_field14;
                                                        if (shared_constructor_field25 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]))) {
                                                            self::accept_scalar(heap.clone(), directory.clone(), {
                                                                let source_ml_source2_nominal_field12791 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field12793 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field12795 = shared_constructor_field3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field12797 = shared_constructor_field4;
                                                                            {
                                                                                let source_ml_source2_nominal_field12799 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field12801 = shared_constructor_field6;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12803 = shared_constructor_field7;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12805 = shared_constructor_field8;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12807 = shared_constructor_field9;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12809 = shared_constructor_field10;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12811 = shared_constructor_field11;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12813 = shared_constructor_field12.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12815 = shared_constructor_field13.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12817 = shared_constructor_field14.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12819 = shared_constructor_field15.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12821 = shared_constructor_field16.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12823 = shared_constructor_field17.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12825 = shared_constructor_field18.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12827 = shared_constructor_field19.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field12829 = shared_constructor_field20.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field12831 = shared_constructor_field21.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field12833 = shared_constructor_field22.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field12835 = shared_constructor_field23.clone();
                                                                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12791.clone()), Box::new(source_ml_source2_nominal_field12793.clone()), Box::new(source_ml_source2_nominal_field12795.clone()), source_ml_source2_nominal_field12797, Box::new(source_ml_source2_nominal_field12799.clone()), source_ml_source2_nominal_field12801, source_ml_source2_nominal_field12803, source_ml_source2_nominal_field12805, source_ml_source2_nominal_field12807, source_ml_source2_nominal_field12809, source_ml_source2_nominal_field12811, Box::new(source_ml_source2_nominal_field12813.clone()), Box::new(source_ml_source2_nominal_field12815.clone()), Box::new(value.clone()), source_ml_source2_nominal_field12819.clone(), source_ml_source2_nominal_field12821.clone(), source_ml_source2_nominal_field12823.clone(), source_ml_source2_nominal_field12825.clone(), source_ml_source2_nominal_field12827.clone(), source_ml_source2_nominal_field12829.clone(), source_ml_source2_nominal_field12831.clone(), source_ml_source2_nominal_field12833.clone(), source_ml_source2_nominal_field12835.clone())
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
                                                            }, self::begin_java_script_scalar_protocol(heap.clone(), value.clone(), shared_constructor_field24.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                        } else {
                                                            if ((shared_constructor_field25 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && !(match (match state.clone() {
                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12841, _, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value12841.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                self::accept_scalar(heap.clone(), directory.clone(), base.clone(), self::resume_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12842, _, _, _, _, _, _) => {
                                                                        ((source_ml_source2_data_property_value12842.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value"), completion.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                            } else {
                                                                if ((shared_constructor_field25 == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && !(match (match state.clone() {
                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12845, _, _, _, _, _) => {
                                                                        ((source_ml_source2_data_property_value12845.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                    self::accept_species(heap.clone(), base.clone(), self::resume_java_script_array_species_getter(heap.clone(), directory.clone(), (match state.clone() {
                                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12846, _, _, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value12846.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value"), completion.clone()))
                                                                } else {
                                                                    if ((shared_constructor_field25 == (self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104]))) && !(match (match state.clone() {
                                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12849, _) => {
                                                                            ((source_ml_source2_data_property_value12849.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                        self::accept_final_set(base.clone(), self::resume_java_script_object_spread(heap.clone(), (match state.clone() {
                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12850, _) => {
                                                                                ((source_ml_source2_data_property_value12850.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value"), completion.clone()))
                                                                    } else {
                                                                        if (shared_constructor_field25 == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]))) {
                                                                            self::JavaScriptArraySliceTransition::ArraySliceContinue(Box::new(heap.clone()), Box::new({
                                                                                let source_ml_source2_nominal_field12854 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field12856 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12858 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12860 = shared_constructor_field4;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12862 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12864 = shared_constructor_field6;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12866 = shared_constructor_field7;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12868 = shared_constructor_field8;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12870 = shared_constructor_field9;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12872 = shared_constructor_field10;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12874 = shared_constructor_field11;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12876 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12878 = shared_constructor_field13.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12880 = shared_constructor_field14.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12882 = shared_constructor_field15.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field12884 = shared_constructor_field16.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field12886 = shared_constructor_field17.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field12888 = shared_constructor_field18.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field12890 = shared_constructor_field19.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field12892 = shared_constructor_field20.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field12894 = shared_constructor_field21.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field12896 = shared_constructor_field22.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field12898 = shared_constructor_field23.clone();
                                                                                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12854.clone()), Box::new(source_ml_source2_nominal_field12856.clone()), Box::new(source_ml_source2_nominal_field12858.clone()), source_ml_source2_nominal_field12860, Box::new(source_ml_source2_nominal_field12862.clone()), source_ml_source2_nominal_field12864, source_ml_source2_nominal_field12866, source_ml_source2_nominal_field12868, source_ml_source2_nominal_field12870, source_ml_source2_nominal_field12872, source_ml_source2_nominal_field12874, Box::new(source_ml_source2_nominal_field12876.clone()), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field12880.clone()), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), source_ml_source2_nominal_field12884.clone(), source_ml_source2_nominal_field12886.clone(), source_ml_source2_nominal_field12888.clone(), source_ml_source2_nominal_field12890.clone(), source_ml_source2_nominal_field12892.clone(), source_ml_source2_nominal_field12894.clone(), source_ml_source2_nominal_field12896.clone(), source_ml_source2_nominal_field12898.clone())
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
                                                                            }))
                                                                        } else {
                                                                            self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 104, 97, 115, 101]))
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
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptArraySliceTransition::ArraySliceThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
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
pub fn resume_java_script_array_slice_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArraySliceState,
    identity: self::ml::Big,
) -> self::JavaScriptArraySliceTransition {
    match state.clone() {
        self::JavaScriptArraySliceState::ArraySliceState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
            shared_constructor_field18,
            shared_constructor_field19,
            shared_constructor_field20,
            shared_constructor_field21,
            shared_constructor_field22,
            shared_constructor_field23,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            let shared_constructor_field14 = *shared_constructor_field14;
            {
                let error = self::read_java_script_array_slice_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if ((shared_constructor_field15
                        != (self::ml_string::Text::from_units(vec![
                            97, 108, 108, 111, 99, 97, 116, 101,
                        ])))
                        || match (
                            match state.clone() {
                                self::JavaScriptArraySliceState::ArraySliceState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value12904,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value12904
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            },
                            None::<()>,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        })
                    {
                        self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                115, 108, 105, 99, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                111, 110, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32,
                                114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 114,
                                101, 113, 117, 101, 115, 116,
                            ]),
                        )
                    } else {
                        {
                            let preflight = self::allocation_request(heap.clone(), state.clone());
                            match preflight.clone() {
                                self::JavaScriptArraySliceTransition::ArraySliceAllocate(
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let result = self::allocate_java_script_array(
                                        heap.clone(),
                                        identity.clone(),
                                        (match state.clone() {
                                            self::JavaScriptArraySliceState::ArraySliceState(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source2_data_property_value12909,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => ((source_ml_source2_data_property_value12909
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    (None::<self::JavaScriptSemanticValue>,)
                                                }))
                                            .0)
                                                .clone(),
                                        })
                                        .clone()
                                        .expect("a proven present value"),
                                        shared_constructor_field9,
                                    );
                                    match result.clone() {
                                            self::JavaScriptArrayCompletion::ArrayNormal(heap_2, value) => {
                                                let heap_2 = *heap_2;
                                                let value = *value;
                                                self::JavaScriptArraySliceTransition::ArraySliceContinue(Box::new(heap_2.clone()), Box::new({
                                                    let source_ml_source2_nominal_field12919 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field12921 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field12923 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field12925 = shared_constructor_field4;
                                                                {
                                                                    let source_ml_source2_nominal_field12927 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field12929 = shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source2_nominal_field12931 = shared_constructor_field7;
                                                                            {
                                                                                let source_ml_source2_nominal_field12933 = shared_constructor_field8;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12935 = shared_constructor_field9;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12937 = shared_constructor_field10;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12939 = shared_constructor_field11;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12941 = shared_constructor_field12.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12943 = shared_constructor_field13.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12945 = shared_constructor_field14.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12947 = shared_constructor_field15.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12949 = shared_constructor_field16.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12951 = shared_constructor_field17.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12953 = shared_constructor_field18.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12955 = shared_constructor_field19.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12957 = shared_constructor_field20.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12959 = shared_constructor_field21.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12961 = shared_constructor_field22.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field12963 = shared_constructor_field23.clone();
                                                                                                                                            self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12919.clone()), Box::new(source_ml_source2_nominal_field12921.clone()), Box::new(source_ml_source2_nominal_field12923.clone()), source_ml_source2_nominal_field12925, Box::new(source_ml_source2_nominal_field12927.clone()), source_ml_source2_nominal_field12929, source_ml_source2_nominal_field12931, source_ml_source2_nominal_field12933, source_ml_source2_nominal_field12935, source_ml_source2_nominal_field12937, source_ml_source2_nominal_field12939, Box::new(value.clone()), Box::new(source_ml_source2_nominal_field12943.clone()), Box::new(source_ml_source2_nominal_field12945.clone()), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field12949.clone(), source_ml_source2_nominal_field12951.clone(), source_ml_source2_nominal_field12953.clone(), source_ml_source2_nominal_field12955.clone(), source_ml_source2_nominal_field12957.clone(), source_ml_source2_nominal_field12959.clone(), source_ml_source2_nominal_field12961.clone(), source_ml_source2_nominal_field12963.clone())
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
                                                }))
                                            }
                                            self::JavaScriptArrayCompletion::ArrayPropertyComplete(heap_3, _, _) => {
                                                let heap_3 = *heap_3;
                                                self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptArrayCompletion::ArrayNumberRequest(heap_4, _, _) => {
                                                let heap_4 = *heap_4;
                                                self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap_4.clone()), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptArrayCompletion::ArrayDefinitionContinue(heap_5, _) => {
                                                let heap_5 = *heap_5;
                                                self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap_5.clone()), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptArrayCompletion::ArrayInvalid(heap_6, source_ml_source2_f12916) => {
                                                let heap_6 = *heap_6;
                                                self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap_6.clone()), source_ml_source2_f12916.clone())
                                            }
                                            self::JavaScriptArrayCompletion::ArrayError(heap_7, error_class, reason) => {
                                                let heap_7 = *heap_7;
                                                self::JavaScriptArraySliceTransition::ArraySliceError(Box::new(heap_7.clone()), error_class.clone(), reason.clone())
                                            }
                                            self::JavaScriptArrayCompletion::ArrayResourceLimit(heap_8, reason_2, _) => {
                                                let heap_8 = *heap_8;
                                                self::JavaScriptArraySliceTransition::ArraySliceResourceLimit(Box::new(heap_8.clone()), Box::new(state.clone()), reason_2.clone())
                                            }
                                        }
                                }
                                _ => preflight.clone(),
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
pub fn resume_java_script_array_slice_construction(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArraySliceState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArraySliceTransition {
    match state.clone() {
        self::JavaScriptArraySliceState::ArraySliceState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
            shared_constructor_field18,
            shared_constructor_field19,
            shared_constructor_field20,
            shared_constructor_field21,
            shared_constructor_field22,
            shared_constructor_field23,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            let shared_constructor_field14 = *shared_constructor_field14;
            {
                let error = self::read_java_script_array_slice_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if ((shared_constructor_field15
                        != (self::ml_string::Text::from_units(vec![
                            99, 111, 110, 115, 116, 114, 117, 99, 116,
                        ])))
                        || !(self::ml_array::includes(
                            &{
                                let ml_reference_value781 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            110, 111, 114, 109, 97, 108, 67, 111, 109, 112, 108,
                                            101, 116, 105, 111, 110,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            116, 104, 114, 111, 119, 67, 111, 109, 112, 108, 101,
                                            116, 105, 111, 110,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value781
                            },
                            &match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                    _,
                                ) => self::ml_string::Text::from_units(vec![
                                    110, 111, 114, 109, 97, 108, 67, 111, 109, 112, 108, 101, 116,
                                    105, 111, 110,
                                ]),
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                    _,
                                ) => self::ml_string::Text::from_units(vec![
                                    116, 104, 114, 111, 119, 67, 111, 109, 112, 108, 101, 116, 105,
                                    111, 110,
                                ]),
                            },
                            0f64,
                            false,
                            |a, b| a == b,
                        )))
                    {
                        self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                115, 108, 105, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99,
                                116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 113, 117,
                                101, 115, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101,
                                116, 105, 111, 110, 32, 107, 105, 110, 100,
                            ]),
                        )
                    } else {
                        {
                            let invalid = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value12973) => {
                                    let source_ml_source2_data_property_value12973 = *source_ml_source2_data_property_value12973;
                                    source_ml_source2_data_property_value12973.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value12974) => {
                                    let source_ml_source2_data_property_value12974 = *source_ml_source2_data_property_value12974;
                                    source_ml_source2_data_property_value12974.clone()
                                }
                            });
                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                                    Box::new(heap.clone()),
                                    invalid.clone(),
                                )
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        if match value.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        } {
                                            self::JavaScriptArraySliceTransition::ArraySliceContinue(Box::new(heap.clone()), Box::new({
                                                let source_ml_source2_nominal_field12978 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field12980 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field12982 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field12984 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_nominal_field12986 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field12988 = shared_constructor_field6;
                                                                    {
                                                                        let source_ml_source2_nominal_field12990 = shared_constructor_field7;
                                                                        {
                                                                            let source_ml_source2_nominal_field12992 = shared_constructor_field8;
                                                                            {
                                                                                let source_ml_source2_nominal_field12994 = shared_constructor_field9;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12996 = shared_constructor_field10;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12998 = shared_constructor_field11;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field13000 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field13002 = shared_constructor_field13.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field13004 = shared_constructor_field14.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field13006 = shared_constructor_field15.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field13008 = shared_constructor_field16.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field13010 = shared_constructor_field17.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field13012 = shared_constructor_field18.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field13014 = shared_constructor_field19.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field13016 = shared_constructor_field20.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field13018 = shared_constructor_field21.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field13020 = shared_constructor_field22.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field13022 = shared_constructor_field23.clone();
                                                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12978.clone()), Box::new(source_ml_source2_nominal_field12980.clone()), Box::new(source_ml_source2_nominal_field12982.clone()), source_ml_source2_nominal_field12984, Box::new(source_ml_source2_nominal_field12986.clone()), source_ml_source2_nominal_field12988, source_ml_source2_nominal_field12990, source_ml_source2_nominal_field12992, source_ml_source2_nominal_field12994, source_ml_source2_nominal_field12996, source_ml_source2_nominal_field12998, Box::new(value.clone()), Box::new(source_ml_source2_nominal_field13002.clone()), Box::new(source_ml_source2_nominal_field13004.clone()), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field13008.clone(), source_ml_source2_nominal_field13010.clone(), source_ml_source2_nominal_field13012.clone(), source_ml_source2_nominal_field13014.clone(), source_ml_source2_nominal_field13016.clone(), source_ml_source2_nominal_field13018.clone(), source_ml_source2_nominal_field13020.clone(), source_ml_source2_nominal_field13022.clone())
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
                                            }))
                                        } else {
                                            self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 67, 111, 110, 115, 116, 114, 117, 99, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 115, 117, 108, 116]))
                                        }
                                    }
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptArraySliceTransition::ArraySliceThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
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
