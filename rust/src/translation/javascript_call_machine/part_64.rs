// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySliceCall;
use super::JavaScriptArraySliceState;
use super::JavaScriptArraySliceTransition;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::accept_species;
use super::begin_java_script_array_species_selection;
use super::begin_java_script_scalar_protocol;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_like_length;
use super::read_java_script_array_slice_relative_index;
use super::request_call;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_scalar(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArraySliceState,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptArraySliceTransition {
    {
        let source_ml_source1_temporary_binding364 = state.clone();
        {
            let prior_call = match source_ml_source1_temporary_binding364.clone() {
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
                    source_ml_source2_data_property_value11755,
                ) => ((source_ml_source2_data_property_value11755
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,)))
                .0)
                    .clone(),
            };
            {
                let prior_conversion = match source_ml_source1_temporary_binding364.clone() {
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
                        source_ml_source2_data_property_value11756,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value11756
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,)))
                    .0)
                        .clone(),
                };
                {
                    let base = match source_ml_source1_temporary_binding364.clone() {
                        self::JavaScriptArraySliceState::ArraySliceState(
                            source_ml_source2_nominal_rest_field11757,
                            source_ml_source2_nominal_rest_field11758,
                            source_ml_source2_nominal_rest_field11759,
                            source_ml_source2_nominal_rest_field11760,
                            source_ml_source2_nominal_rest_field11761,
                            source_ml_source2_nominal_rest_field11762,
                            source_ml_source2_nominal_rest_field11763,
                            source_ml_source2_nominal_rest_field11764,
                            source_ml_source2_nominal_rest_field11765,
                            source_ml_source2_nominal_rest_field11766,
                            source_ml_source2_nominal_rest_field11767,
                            source_ml_source2_nominal_rest_field11768,
                            source_ml_source2_nominal_rest_field11769,
                            source_ml_source2_nominal_rest_field11770,
                            source_ml_source2_nominal_rest_field11771,
                            source_ml_source2_nominal_rest_field11772,
                            _,
                            source_ml_source2_nominal_rest_field11773,
                            source_ml_source2_nominal_rest_field11774,
                            source_ml_source2_nominal_rest_field11775,
                            source_ml_source2_nominal_rest_field11776,
                            source_ml_source2_nominal_rest_field11777,
                            _,
                        ) => {
                            let source_ml_source2_nominal_rest_field11757 =
                                *source_ml_source2_nominal_rest_field11757;
                            let source_ml_source2_nominal_rest_field11758 =
                                *source_ml_source2_nominal_rest_field11758;
                            let source_ml_source2_nominal_rest_field11759 =
                                *source_ml_source2_nominal_rest_field11759;
                            let source_ml_source2_nominal_rest_field11761 =
                                *source_ml_source2_nominal_rest_field11761;
                            let source_ml_source2_nominal_rest_field11768 =
                                *source_ml_source2_nominal_rest_field11768;
                            let source_ml_source2_nominal_rest_field11769 =
                                *source_ml_source2_nominal_rest_field11769;
                            let source_ml_source2_nominal_rest_field11770 =
                                *source_ml_source2_nominal_rest_field11770;
                            self::JavaScriptArraySliceState::ArraySliceState(
                                Box::new(source_ml_source2_nominal_rest_field11757.clone()),
                                Box::new(source_ml_source2_nominal_rest_field11758.clone()),
                                Box::new(source_ml_source2_nominal_rest_field11759.clone()),
                                source_ml_source2_nominal_rest_field11760,
                                Box::new(source_ml_source2_nominal_rest_field11761.clone()),
                                source_ml_source2_nominal_rest_field11762,
                                source_ml_source2_nominal_rest_field11763,
                                source_ml_source2_nominal_rest_field11764,
                                source_ml_source2_nominal_rest_field11765,
                                source_ml_source2_nominal_rest_field11766,
                                source_ml_source2_nominal_rest_field11767,
                                Box::new(source_ml_source2_nominal_rest_field11768.clone()),
                                Box::new(source_ml_source2_nominal_rest_field11769.clone()),
                                Box::new(source_ml_source2_nominal_rest_field11770.clone()),
                                source_ml_source2_nominal_rest_field11771.clone(),
                                source_ml_source2_nominal_rest_field11772.clone(),
                                None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                                source_ml_source2_nominal_rest_field11773.clone(),
                                source_ml_source2_nominal_rest_field11774.clone(),
                                source_ml_source2_nominal_rest_field11775.clone(),
                                source_ml_source2_nominal_rest_field11776.clone(),
                                source_ml_source2_nominal_rest_field11777.clone(),
                                None::<(Option<self::JavaScriptArraySliceCall>,)>,
                            )
                        }
                    };
                    match base.clone() {
                        self::JavaScriptArraySliceState::ArraySliceState(
                            shared_constructor_field24,
                            shared_constructor_field25,
                            shared_constructor_field26,
                            shared_constructor_field27,
                            shared_constructor_field28,
                            shared_constructor_field29,
                            shared_constructor_field30,
                            shared_constructor_field31,
                            shared_constructor_field32,
                            shared_constructor_field33,
                            shared_constructor_field34,
                            shared_constructor_field35,
                            shared_constructor_field36,
                            shared_constructor_field37,
                            shared_constructor_field38,
                            shared_constructor_field39,
                            shared_constructor_field40,
                            shared_constructor_field41,
                            shared_constructor_field42,
                            shared_constructor_field43,
                            shared_constructor_field44,
                            shared_constructor_field45,
                            shared_constructor_field46,
                        ) => {
                            let shared_constructor_field24 = *shared_constructor_field24;
                            let shared_constructor_field25 = *shared_constructor_field25;
                            let shared_constructor_field26 = *shared_constructor_field26;
                            let shared_constructor_field28 = *shared_constructor_field28;
                            let shared_constructor_field35 = *shared_constructor_field35;
                            let shared_constructor_field36 = *shared_constructor_field36;
                            let shared_constructor_field37 = *shared_constructor_field37;
                            match transition.clone() {
                                self::JavaScriptScalarConversionTransition::ScalarContinuation(state_2) => {
                                    let state_2 = *state_2;
                                    self::JavaScriptArraySliceTransition::ArraySliceContinue(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field11779 = shared_constructor_field24.clone();
                                        {
                                            let source_ml_source2_nominal_field11781 = shared_constructor_field25.clone();
                                            {
                                                let source_ml_source2_nominal_field11783 = shared_constructor_field26.clone();
                                                {
                                                    let source_ml_source2_nominal_field11785 = shared_constructor_field27;
                                                    {
                                                        let source_ml_source2_nominal_field11787 = shared_constructor_field28.clone();
                                                        {
                                                            let source_ml_source2_nominal_field11789 = shared_constructor_field29;
                                                            {
                                                                let source_ml_source2_nominal_field11791 = shared_constructor_field30;
                                                                {
                                                                    let source_ml_source2_nominal_field11793 = shared_constructor_field31;
                                                                    {
                                                                        let source_ml_source2_nominal_field11795 = shared_constructor_field32;
                                                                        {
                                                                            let source_ml_source2_nominal_field11797 = shared_constructor_field33;
                                                                            {
                                                                                let source_ml_source2_nominal_field11799 = shared_constructor_field34;
                                                                                {
                                                                                    let source_ml_source2_nominal_field11801 = shared_constructor_field35.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11803 = shared_constructor_field36.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11805 = shared_constructor_field37.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11807 = shared_constructor_field38.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11809 = shared_constructor_field39.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11811 = shared_constructor_field40.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11813 = shared_constructor_field41.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11815 = shared_constructor_field42.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11817 = shared_constructor_field43.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11819 = shared_constructor_field44.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11821 = shared_constructor_field45.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field11823 = shared_constructor_field46.clone();
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11779.clone()), Box::new(source_ml_source2_nominal_field11781.clone()), Box::new(source_ml_source2_nominal_field11783.clone()), source_ml_source2_nominal_field11785, Box::new(source_ml_source2_nominal_field11787.clone()), source_ml_source2_nominal_field11789, source_ml_source2_nominal_field11791, source_ml_source2_nominal_field11793, source_ml_source2_nominal_field11795, source_ml_source2_nominal_field11797, source_ml_source2_nominal_field11799, Box::new(source_ml_source2_nominal_field11801.clone()), Box::new(source_ml_source2_nominal_field11803.clone()), Box::new(source_ml_source2_nominal_field11805.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), source_ml_source2_nominal_field11809.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field11813.clone(), source_ml_source2_nominal_field11815.clone(), source_ml_source2_nominal_field11817.clone(), source_ml_source2_nominal_field11819.clone(), source_ml_source2_nominal_field11821.clone(), source_ml_source2_nominal_field11823.clone())
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
                                self::JavaScriptScalarConversionTransition::ScalarCall(state_3, callee, receiver, arguments) => {
                                    let state_3 = *state_3;
                                    let callee = *callee;
                                    let receiver = *receiver;
                                    self::request_call(heap.clone(), {
                                        let source_ml_source2_nominal_field11828 = shared_constructor_field24.clone();
                                        {
                                            let source_ml_source2_nominal_field11830 = shared_constructor_field25.clone();
                                            {
                                                let source_ml_source2_nominal_field11832 = shared_constructor_field26.clone();
                                                {
                                                    let source_ml_source2_nominal_field11834 = shared_constructor_field27;
                                                    {
                                                        let source_ml_source2_nominal_field11836 = shared_constructor_field28.clone();
                                                        {
                                                            let source_ml_source2_nominal_field11838 = shared_constructor_field29;
                                                            {
                                                                let source_ml_source2_nominal_field11840 = shared_constructor_field30;
                                                                {
                                                                    let source_ml_source2_nominal_field11842 = shared_constructor_field31;
                                                                    {
                                                                        let source_ml_source2_nominal_field11844 = shared_constructor_field32;
                                                                        {
                                                                            let source_ml_source2_nominal_field11846 = shared_constructor_field33;
                                                                            {
                                                                                let source_ml_source2_nominal_field11848 = shared_constructor_field34;
                                                                                {
                                                                                    let source_ml_source2_nominal_field11850 = shared_constructor_field35.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11852 = shared_constructor_field36.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11854 = shared_constructor_field37.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11856 = shared_constructor_field38.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11858 = shared_constructor_field39.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11860 = shared_constructor_field40.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11862 = shared_constructor_field41.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11864 = shared_constructor_field42.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11866 = shared_constructor_field43.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11868 = shared_constructor_field44.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11870 = shared_constructor_field45.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field11872 = shared_constructor_field46.clone();
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11828.clone()), Box::new(source_ml_source2_nominal_field11830.clone()), Box::new(source_ml_source2_nominal_field11832.clone()), source_ml_source2_nominal_field11834, Box::new(source_ml_source2_nominal_field11836.clone()), source_ml_source2_nominal_field11838, source_ml_source2_nominal_field11840, source_ml_source2_nominal_field11842, source_ml_source2_nominal_field11844, source_ml_source2_nominal_field11846, source_ml_source2_nominal_field11848, Box::new(source_ml_source2_nominal_field11850.clone()), Box::new(source_ml_source2_nominal_field11852.clone()), Box::new(source_ml_source2_nominal_field11854.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), source_ml_source2_nominal_field11858.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field11862.clone(), source_ml_source2_nominal_field11864.clone(), source_ml_source2_nominal_field11866.clone(), source_ml_source2_nominal_field11868.clone(), source_ml_source2_nominal_field11870.clone(), source_ml_source2_nominal_field11872.clone())
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
                                    }, callee.clone(), receiver.clone(), arguments.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarCompleted(primitive) => {
                                    let primitive = *primitive;
                                    match primitive.clone() {
                                        self::JavaScriptSemanticValue::Number(value) => {
                                            if ((match state.clone() {
                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11877, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value11877.clone()
                                                }
                                            }) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]))) {
                                                {
                                                    let next = {
                                                        let source_ml_source2_nominal_field11880 = shared_constructor_field24.clone();
                                                        {
                                                            let source_ml_source2_nominal_field11882 = shared_constructor_field25.clone();
                                                            {
                                                                let source_ml_source2_nominal_field11884 = shared_constructor_field26.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field11886 = shared_constructor_field27;
                                                                    {
                                                                        let source_ml_source2_nominal_field11888 = shared_constructor_field28.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field11890 = shared_constructor_field29;
                                                                            {
                                                                                let source_ml_source2_nominal_field11892 = shared_constructor_field30;
                                                                                {
                                                                                    let source_ml_source2_nominal_field11894 = shared_constructor_field31;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11896 = shared_constructor_field32;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11898 = shared_constructor_field33;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11900 = shared_constructor_field34;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11902 = shared_constructor_field35.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11904 = shared_constructor_field36.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11906 = shared_constructor_field37.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11908 = shared_constructor_field38.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11910 = shared_constructor_field39.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11912 = shared_constructor_field40.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11914 = shared_constructor_field41.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field11916 = shared_constructor_field42.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field11918 = shared_constructor_field43.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field11920 = shared_constructor_field44.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field11922 = shared_constructor_field45.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field11924 = shared_constructor_field46.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override11926 = self::read_java_script_array_like_length(value);
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_override11928 = match state.clone() {
                                                                                                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, source_ml_source2_data_property_value11878, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                let source_ml_source2_data_property_value11878 = *source_ml_source2_data_property_value11878;
                                                                                                                                                                source_ml_source2_data_property_value11878.clone()
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11880.clone()), Box::new(source_ml_source2_nominal_field11882.clone()), Box::new(source_ml_source2_nominal_field11884.clone()), source_ml_source2_nominal_field11886, Box::new(source_ml_source2_nominal_field11888.clone()), source_ml_source2_nominal_override11926, source_ml_source2_nominal_field11892, source_ml_source2_nominal_field11894, source_ml_source2_nominal_field11896, source_ml_source2_nominal_field11898, source_ml_source2_nominal_field11900, Box::new(source_ml_source2_nominal_field11902.clone()), Box::new(source_ml_source2_nominal_field11904.clone()), Box::new(source_ml_source2_nominal_override11928.clone()), source_ml_source2_nominal_field11908.clone(), self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116]), source_ml_source2_nominal_field11912.clone(), source_ml_source2_nominal_field11914.clone(), source_ml_source2_nominal_field11916.clone(), source_ml_source2_nominal_field11918.clone(), source_ml_source2_nominal_field11920.clone(), source_ml_source2_nominal_field11922.clone(), source_ml_source2_nominal_field11924.clone())
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
                                                    };
                                                    self::accept_scalar(heap.clone(), directory.clone(), next.clone(), self::begin_java_script_scalar_protocol(heap.clone(), match state.clone() {
                                                        self::JavaScriptArraySliceState::ArraySliceState(_, source_ml_source2_data_property_value11929, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value11929 = *source_ml_source2_data_property_value11929;
                                                            source_ml_source2_data_property_value11929.clone()
                                                        }
                                                    }, match state.clone() {
                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, source_ml_source2_data_property_value11930, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value11930 = *source_ml_source2_data_property_value11930;
                                                            source_ml_source2_data_property_value11930.clone()
                                                        }
                                                    }, &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                }
                                            } else {
                                                if ((match state.clone() {
                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11932, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value11932.clone()
                                                    }
                                                }) == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116]))) {
                                                    {
                                                        let start_index = self::read_java_script_array_slice_relative_index(value, match state.clone() {
                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, source_ml_source2_data_property_value11933, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value11933
                                                            }
                                                        });
                                                        {
                                                            let next_2 = {
                                                                let source_ml_source2_nominal_field11936 = shared_constructor_field24.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field11938 = shared_constructor_field25.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field11940 = shared_constructor_field26.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field11942 = shared_constructor_field27;
                                                                            {
                                                                                let source_ml_source2_nominal_field11944 = shared_constructor_field28.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field11946 = shared_constructor_field29;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11948 = shared_constructor_field30;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11950 = shared_constructor_field31;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11952 = shared_constructor_field32;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11954 = shared_constructor_field33;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11956 = shared_constructor_field34;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11958 = shared_constructor_field35.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11960 = shared_constructor_field36.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11962 = shared_constructor_field37.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11964 = shared_constructor_field38.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11966 = shared_constructor_field39.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field11968 = shared_constructor_field40.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field11970 = shared_constructor_field41.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field11972 = shared_constructor_field42.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field11974 = shared_constructor_field43.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field11976 = shared_constructor_field44.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field11978 = shared_constructor_field45.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field11980 = shared_constructor_field46.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_override11985 = match state.clone() {
                                                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, source_ml_source2_data_property_value11934, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                    let source_ml_source2_data_property_value11934 = *source_ml_source2_data_property_value11934;
                                                                                                                                                                    source_ml_source2_data_property_value11934.clone()
                                                                                                                                                                }
                                                                                                                                                            };
                                                                                                                                                            self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11936.clone()), Box::new(source_ml_source2_nominal_field11938.clone()), Box::new(source_ml_source2_nominal_field11940.clone()), source_ml_source2_nominal_field11942, Box::new(source_ml_source2_nominal_field11944.clone()), source_ml_source2_nominal_field11946, start_index, source_ml_source2_nominal_field11950, source_ml_source2_nominal_field11952, start_index, source_ml_source2_nominal_field11956, Box::new(source_ml_source2_nominal_field11958.clone()), Box::new(source_ml_source2_nominal_field11960.clone()), Box::new(source_ml_source2_nominal_override11985.clone()), source_ml_source2_nominal_field11964.clone(), self::ml_string::Text::from_units(vec![101, 110, 100]), source_ml_source2_nominal_field11968.clone(), source_ml_source2_nominal_field11970.clone(), source_ml_source2_nominal_field11972.clone(), source_ml_source2_nominal_field11974.clone(), source_ml_source2_nominal_field11976.clone(), source_ml_source2_nominal_field11978.clone(), source_ml_source2_nominal_field11980.clone())
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
                                                            };
                                                            match next_2.clone() {
                                                                self::JavaScriptArraySliceState::ArraySliceState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14, shared_constructor_field15, shared_constructor_field16, shared_constructor_field17, shared_constructor_field18, shared_constructor_field19, shared_constructor_field20, shared_constructor_field21, shared_constructor_field22, shared_constructor_field23) => {
                                                                    let shared_constructor_field1 = *shared_constructor_field1;
                                                                    let shared_constructor_field2 = *shared_constructor_field2;
                                                                    let shared_constructor_field3 = *shared_constructor_field3;
                                                                    let shared_constructor_field5 = *shared_constructor_field5;
                                                                    let shared_constructor_field12 = *shared_constructor_field12;
                                                                    let shared_constructor_field13 = *shared_constructor_field13;
                                                                    let shared_constructor_field14 = *shared_constructor_field14;
                                                                    if (match state.clone() {
                                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, source_ml_source2_data_property_value11986, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value11986
                                                                        }
                                                                    } && {
                                                                        let source_ml_source2_s11988 = match state.clone() {
                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, source_ml_source2_data_property_value11989, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value11989 = *source_ml_source2_data_property_value11989;
                                                                                source_ml_source2_data_property_value11989.clone()
                                                                            }
                                                                        };
                                                                        match source_ml_source2_s11988.clone() {
                                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                                false
                                                                            }
                                                                            _ => {
                                                                                true
                                                                            }
                                                                        }
                                                                    }) {
                                                                        self::accept_scalar(heap.clone(), directory.clone(), next_2.clone(), self::begin_java_script_scalar_protocol(heap.clone(), match state.clone() {
                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, source_ml_source2_data_property_value11990, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value11990 = *source_ml_source2_data_property_value11990;
                                                                                source_ml_source2_data_property_value11990.clone()
                                                                            }
                                                                        }, match state.clone() {
                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, source_ml_source2_data_property_value11991, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value11991 = *source_ml_source2_data_property_value11991;
                                                                                source_ml_source2_data_property_value11991.clone()
                                                                            }
                                                                        }, &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                                    } else {
                                                                        {
                                                                            let count = self::ml_math::max((match state.clone() {
                                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, source_ml_source2_data_property_value11992, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value11992
                                                                                }
                                                                            } - start_index), 0f64);
                                                                            {
                                                                                let selected = {
                                                                                    let source_ml_source2_nominal_field11995 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11997 = shared_constructor_field2.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11999 = shared_constructor_field3.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12001 = shared_constructor_field4;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12003 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12005 = shared_constructor_field6;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12007 = shared_constructor_field7;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12009 = shared_constructor_field8;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12011 = shared_constructor_field9;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12013 = shared_constructor_field10;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12015 = shared_constructor_field11;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12017 = shared_constructor_field12.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12019 = shared_constructor_field13.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12021 = shared_constructor_field14.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field12023 = shared_constructor_field15.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field12025 = shared_constructor_field16.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field12027 = shared_constructor_field17.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field12029 = shared_constructor_field18.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field12031 = shared_constructor_field19.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field12033 = shared_constructor_field20.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field12035 = shared_constructor_field21.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field12037 = shared_constructor_field22.clone();
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field12039 = shared_constructor_field23.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_override12041 = match state.clone() {
                                                                                                                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, source_ml_source2_data_property_value11993, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                                        source_ml_source2_data_property_value11993
                                                                                                                                                                                    }
                                                                                                                                                                                };
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_nominal_override12043 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                                                    self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11995.clone()), Box::new(source_ml_source2_nominal_field11997.clone()), Box::new(source_ml_source2_nominal_field11999.clone()), source_ml_source2_nominal_field12001, Box::new(source_ml_source2_nominal_field12003.clone()), source_ml_source2_nominal_field12005, source_ml_source2_nominal_field12007, source_ml_source2_nominal_override12041, count, source_ml_source2_nominal_field12013, source_ml_source2_nominal_field12015, Box::new(source_ml_source2_nominal_field12017.clone()), Box::new(source_ml_source2_nominal_field12019.clone()), Box::new(source_ml_source2_nominal_override12043.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field12025.clone(), source_ml_source2_nominal_field12027.clone(), source_ml_source2_nominal_field12029.clone(), source_ml_source2_nominal_field12031.clone(), source_ml_source2_nominal_field12033.clone(), source_ml_source2_nominal_field12035.clone(), source_ml_source2_nominal_field12037.clone(), source_ml_source2_nominal_field12039.clone())
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
                                                                                };
                                                                                self::accept_species(heap.clone(), selected.clone(), self::begin_java_script_array_species_selection(heap.clone(), directory.clone(), match state.clone() {
                                                                                    self::JavaScriptArraySliceState::ArraySliceState(source_ml_source2_data_property_value12045, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value12045 = *source_ml_source2_data_property_value12045;
                                                                                        source_ml_source2_data_property_value12045.clone()
                                                                                    }
                                                                                }, count))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let end_index = self::read_java_script_array_slice_relative_index(value, match state.clone() {
                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, source_ml_source2_data_property_value12046, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12046
                                                            }
                                                        });
                                                        {
                                                            let count_2 = self::ml_math::max((end_index - match state.clone() {
                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, source_ml_source2_data_property_value12047, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12047
                                                                }
                                                            }), 0f64);
                                                            {
                                                                let selected_2 = {
                                                                    let source_ml_source2_nominal_field12049 = shared_constructor_field24.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field12051 = shared_constructor_field25.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field12053 = shared_constructor_field26.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field12055 = shared_constructor_field27;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12057 = shared_constructor_field28.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12059 = shared_constructor_field29;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12061 = shared_constructor_field30;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12063 = shared_constructor_field31;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12065 = shared_constructor_field32;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12067 = shared_constructor_field33;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12069 = shared_constructor_field34;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12071 = shared_constructor_field35.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12073 = shared_constructor_field36.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12075 = shared_constructor_field37.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12077 = shared_constructor_field38.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12079 = shared_constructor_field39.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12081 = shared_constructor_field40.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12083 = shared_constructor_field41.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field12085 = shared_constructor_field42.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field12087 = shared_constructor_field43.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field12089 = shared_constructor_field44.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field12091 = shared_constructor_field45.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field12093 = shared_constructor_field46.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_override12097 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12049.clone()), Box::new(source_ml_source2_nominal_field12051.clone()), Box::new(source_ml_source2_nominal_field12053.clone()), source_ml_source2_nominal_field12055, Box::new(source_ml_source2_nominal_field12057.clone()), source_ml_source2_nominal_field12059, source_ml_source2_nominal_field12061, end_index, count_2, source_ml_source2_nominal_field12067, source_ml_source2_nominal_field12069, Box::new(source_ml_source2_nominal_field12071.clone()), Box::new(source_ml_source2_nominal_field12073.clone()), Box::new(source_ml_source2_nominal_override12097.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field12079.clone(), source_ml_source2_nominal_field12081.clone(), source_ml_source2_nominal_field12083.clone(), source_ml_source2_nominal_field12085.clone(), source_ml_source2_nominal_field12087.clone(), source_ml_source2_nominal_field12089.clone(), source_ml_source2_nominal_field12091.clone(), source_ml_source2_nominal_field12093.clone())
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
                                                                };
                                                                self::accept_species(heap.clone(), selected_2.clone(), self::begin_java_script_array_species_selection(heap.clone(), directory.clone(), match state.clone() {
                                                                    self::JavaScriptArraySliceState::ArraySliceState(source_ml_source2_data_property_value12099, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value12099 = *source_ml_source2_data_property_value12099;
                                                                        source_ml_source2_data_property_value12099.clone()
                                                                    }
                                                                }, count_2))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                                        }
                                    }
                                }
                                self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                                    let value_2 = *value_2;
                                    self::JavaScriptArraySliceTransition::ArraySliceThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
                                }
                                self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
                                    self::JavaScriptArraySliceTransition::ArraySliceError(Box::new(heap.clone()), error_class.clone(), reason.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                                    self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap.clone()), reason_2.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
                                    let value_3 = *value_3;
                                    self::JavaScriptArraySliceTransition::ArraySliceResourceLimit(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field12101 = shared_constructor_field24.clone();
                                        {
                                            let source_ml_source2_nominal_field12103 = shared_constructor_field25.clone();
                                            {
                                                let source_ml_source2_nominal_field12105 = shared_constructor_field26.clone();
                                                {
                                                    let source_ml_source2_nominal_field12107 = shared_constructor_field27;
                                                    {
                                                        let source_ml_source2_nominal_field12109 = shared_constructor_field28.clone();
                                                        {
                                                            let source_ml_source2_nominal_field12111 = shared_constructor_field29;
                                                            {
                                                                let source_ml_source2_nominal_field12113 = shared_constructor_field30;
                                                                {
                                                                    let source_ml_source2_nominal_field12115 = shared_constructor_field31;
                                                                    {
                                                                        let source_ml_source2_nominal_field12117 = shared_constructor_field32;
                                                                        {
                                                                            let source_ml_source2_nominal_field12119 = shared_constructor_field33;
                                                                            {
                                                                                let source_ml_source2_nominal_field12121 = shared_constructor_field34;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12123 = shared_constructor_field35.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12125 = shared_constructor_field36.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12127 = shared_constructor_field37.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12129 = shared_constructor_field38.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12131 = shared_constructor_field39.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12133 = shared_constructor_field40.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12135 = shared_constructor_field41.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12137 = shared_constructor_field42.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12139 = shared_constructor_field43.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12141 = shared_constructor_field44.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12143 = shared_constructor_field45.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12145 = shared_constructor_field46.clone();
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12101.clone()), Box::new(source_ml_source2_nominal_field12103.clone()), Box::new(source_ml_source2_nominal_field12105.clone()), source_ml_source2_nominal_field12107, Box::new(source_ml_source2_nominal_field12109.clone()), source_ml_source2_nominal_field12111, source_ml_source2_nominal_field12113, source_ml_source2_nominal_field12115, source_ml_source2_nominal_field12117, source_ml_source2_nominal_field12119, source_ml_source2_nominal_field12121, Box::new(source_ml_source2_nominal_field12123.clone()), Box::new(source_ml_source2_nominal_field12125.clone()), Box::new(value_3.clone()), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101]), source_ml_source2_nominal_field12131.clone(), source_ml_source2_nominal_field12133.clone(), source_ml_source2_nominal_field12135.clone(), source_ml_source2_nominal_field12137.clone(), source_ml_source2_nominal_field12139.clone(), source_ml_source2_nominal_field12141.clone(), source_ml_source2_nominal_field12143.clone(), source_ml_source2_nominal_field12145.clone())
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
                                    }), reason_3.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
