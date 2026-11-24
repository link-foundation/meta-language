// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptArraySpeciesTransition;
use super::JavaScriptArrayTransformCall;
use super::JavaScriptArrayTransformState;
use super::JavaScriptArrayTransformTransition;
use super::JavaScriptOrdinaryHeap;
use super::copy_java_script_array_constructor_fact;
use super::ml_string;
use super::request_java_script_array_transform_allocation;
use super::request_java_script_array_transform_call;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_transform_species(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayTransformState,
    transition: self::JavaScriptArraySpeciesTransition,
) -> self::JavaScriptArrayTransformTransition {
    {
        let source_ml_source1_temporary_binding241 = state.clone();
        {
            let prior_call = match source_ml_source1_temporary_binding241.clone() {
                self::JavaScriptArrayTransformState::ArrayTransformState(
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
                    source_ml_source2_data_property_value4786,
                ) => ((source_ml_source2_data_property_value4786
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,)))
                .0)
                    .clone(),
            };
            {
                let base = match source_ml_source1_temporary_binding241.clone() {
                    self::JavaScriptArrayTransformState::ArrayTransformState(
                        source_ml_source2_nominal_rest_field4787,
                        source_ml_source2_nominal_rest_field4788,
                        source_ml_source2_nominal_rest_field4789,
                        source_ml_source2_nominal_rest_field4790,
                        source_ml_source2_nominal_rest_field4791,
                        source_ml_source2_nominal_rest_field4792,
                        source_ml_source2_nominal_rest_field4793,
                        source_ml_source2_nominal_rest_field4794,
                        source_ml_source2_nominal_rest_field4795,
                        source_ml_source2_nominal_rest_field4796,
                        source_ml_source2_nominal_rest_field4797,
                        source_ml_source2_nominal_rest_field4798,
                        source_ml_source2_nominal_rest_field4799,
                        source_ml_source2_nominal_rest_field4800,
                        source_ml_source2_nominal_rest_field4801,
                        source_ml_source2_nominal_rest_field4802,
                        source_ml_source2_nominal_rest_field4803,
                        source_ml_source2_nominal_rest_field4804,
                        _,
                    ) => {
                        let source_ml_source2_nominal_rest_field4788 =
                            *source_ml_source2_nominal_rest_field4788;
                        let source_ml_source2_nominal_rest_field4789 =
                            *source_ml_source2_nominal_rest_field4789;
                        let source_ml_source2_nominal_rest_field4790 =
                            *source_ml_source2_nominal_rest_field4790;
                        let source_ml_source2_nominal_rest_field4791 =
                            *source_ml_source2_nominal_rest_field4791;
                        let source_ml_source2_nominal_rest_field4795 =
                            *source_ml_source2_nominal_rest_field4795;
                        let source_ml_source2_nominal_rest_field4796 =
                            *source_ml_source2_nominal_rest_field4796;
                        let source_ml_source2_nominal_rest_field4797 =
                            *source_ml_source2_nominal_rest_field4797;
                        let source_ml_source2_nominal_rest_field4798 =
                            *source_ml_source2_nominal_rest_field4798;
                        self::JavaScriptArrayTransformState::ArrayTransformState(
                            source_ml_source2_nominal_rest_field4787.clone(),
                            Box::new(source_ml_source2_nominal_rest_field4788.clone()),
                            Box::new(source_ml_source2_nominal_rest_field4789.clone()),
                            Box::new(source_ml_source2_nominal_rest_field4790.clone()),
                            Box::new(source_ml_source2_nominal_rest_field4791.clone()),
                            source_ml_source2_nominal_rest_field4792,
                            source_ml_source2_nominal_rest_field4793,
                            source_ml_source2_nominal_rest_field4794,
                            Box::new(source_ml_source2_nominal_rest_field4795.clone()),
                            Box::new(source_ml_source2_nominal_rest_field4796.clone()),
                            Box::new(source_ml_source2_nominal_rest_field4797.clone()),
                            Box::new(source_ml_source2_nominal_rest_field4798.clone()),
                            source_ml_source2_nominal_rest_field4799.clone(),
                            source_ml_source2_nominal_rest_field4800.clone(),
                            source_ml_source2_nominal_rest_field4801.clone(),
                            source_ml_source2_nominal_rest_field4802.clone(),
                            source_ml_source2_nominal_rest_field4803.clone(),
                            source_ml_source2_nominal_rest_field4804.clone(),
                            None::<(Option<self::JavaScriptArrayTransformCall>,)>,
                        )
                    }
                };
                match base.clone() {
                    self::JavaScriptArrayTransformState::ArrayTransformState(
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
                    ) => {
                        let shared_constructor_field27 = *shared_constructor_field27;
                        let shared_constructor_field28 = *shared_constructor_field28;
                        let shared_constructor_field29 = *shared_constructor_field29;
                        let shared_constructor_field30 = *shared_constructor_field30;
                        let shared_constructor_field34 = *shared_constructor_field34;
                        let shared_constructor_field35 = *shared_constructor_field35;
                        let shared_constructor_field36 = *shared_constructor_field36;
                        let shared_constructor_field37 = *shared_constructor_field37;
                        match transition.clone() {
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesContinue(
                                heap_2,
                                state_2,
                            ) => {
                                let heap_2 = *heap_2;
                                let state_2 = *state_2;
                                self::JavaScriptArrayTransformTransition::ArrayTransformContinue(
                                    Box::new(heap_2.clone()),
                                    Box::new({
                                        let source_ml_source2_nominal_field4806 =
                                            shared_constructor_field26.clone();
                                        {
                                            let source_ml_source2_nominal_field4808 =
                                                shared_constructor_field27.clone();
                                            {
                                                let source_ml_source2_nominal_field4810 =
                                                    shared_constructor_field28.clone();
                                                {
                                                    let source_ml_source2_nominal_field4812 =
                                                        shared_constructor_field29.clone();
                                                    {
                                                        let source_ml_source2_nominal_field4814 =
                                                            shared_constructor_field30.clone();
                                                        {
                                                            let source_ml_source2_nominal_field4816 =
                                                                shared_constructor_field31;
                                                            {
                                                                let source_ml_source2_nominal_field4818 =
                                                                    shared_constructor_field32;
                                                                {
                                                                    let source_ml_source2_nominal_field4820 =
                                                                        shared_constructor_field33;
                                                                    {
                                                                        let source_ml_source2_nominal_field4822 = shared_constructor_field34.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field4824 = shared_constructor_field35.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field4826 = shared_constructor_field36.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field4828 = shared_constructor_field37.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4830 = shared_constructor_field38.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4832 = shared_constructor_field39.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4834 = shared_constructor_field40.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field4836 = shared_constructor_field41.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field4838 = shared_constructor_field42.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field4840 = shared_constructor_field43.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field4842 = shared_constructor_field44.clone();
                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field4806.clone(), Box::new(source_ml_source2_nominal_field4808.clone()), Box::new(source_ml_source2_nominal_field4810.clone()), Box::new(source_ml_source2_nominal_field4812.clone()), Box::new(source_ml_source2_nominal_field4814.clone()), source_ml_source2_nominal_field4816, source_ml_source2_nominal_field4818, source_ml_source2_nominal_field4820, Box::new(source_ml_source2_nominal_field4822.clone()), Box::new(source_ml_source2_nominal_field4824.clone()), Box::new(source_ml_source2_nominal_field4826.clone()), Box::new(source_ml_source2_nominal_field4828.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field4832.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field4836.clone(), source_ml_source2_nominal_field4838.clone(), source_ml_source2_nominal_field4840.clone(), source_ml_source2_nominal_field4842.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }),
                                )
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesCall(
                                heap_3,
                                state_3,
                                callee,
                                receiver,
                                arguments,
                            ) => {
                                let heap_3 = *heap_3;
                                let state_3 = *state_3;
                                let callee = *callee;
                                let receiver = *receiver;
                                self::request_java_script_array_transform_call(
                                    heap_3.clone(),
                                    {
                                        let source_ml_source2_nominal_field4847 =
                                            shared_constructor_field26.clone();
                                        {
                                            let source_ml_source2_nominal_field4849 =
                                                shared_constructor_field27.clone();
                                            {
                                                let source_ml_source2_nominal_field4851 =
                                                    shared_constructor_field28.clone();
                                                {
                                                    let source_ml_source2_nominal_field4853 =
                                                        shared_constructor_field29.clone();
                                                    {
                                                        let source_ml_source2_nominal_field4855 =
                                                            shared_constructor_field30.clone();
                                                        {
                                                            let source_ml_source2_nominal_field4857 =
                                                                shared_constructor_field31;
                                                            {
                                                                let source_ml_source2_nominal_field4859 =
                                                                    shared_constructor_field32;
                                                                {
                                                                    let source_ml_source2_nominal_field4861 =
                                                                        shared_constructor_field33;
                                                                    {
                                                                        let source_ml_source2_nominal_field4863 = shared_constructor_field34.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field4865 = shared_constructor_field35.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field4867 = shared_constructor_field36.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field4869 = shared_constructor_field37.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4871 = shared_constructor_field38.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4873 = shared_constructor_field39.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4875 = shared_constructor_field40.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field4877 = shared_constructor_field41.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field4879 = shared_constructor_field42.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field4881 = shared_constructor_field43.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field4883 = shared_constructor_field44.clone();
                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field4847.clone(), Box::new(source_ml_source2_nominal_field4849.clone()), Box::new(source_ml_source2_nominal_field4851.clone()), Box::new(source_ml_source2_nominal_field4853.clone()), Box::new(source_ml_source2_nominal_field4855.clone()), source_ml_source2_nominal_field4857, source_ml_source2_nominal_field4859, source_ml_source2_nominal_field4861, Box::new(source_ml_source2_nominal_field4863.clone()), Box::new(source_ml_source2_nominal_field4865.clone()), Box::new(source_ml_source2_nominal_field4867.clone()), Box::new(source_ml_source2_nominal_field4869.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field4873.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field4877.clone(), source_ml_source2_nominal_field4879.clone(), source_ml_source2_nominal_field4881.clone(), source_ml_source2_nominal_field4883.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    callee.clone(),
                                    receiver.clone(),
                                    arguments.clone(),
                                )
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesNeedAuthority(
                                heap_4,
                                state_4,
                                constructor,
                            ) => {
                                let heap_4 = *heap_4;
                                let state_4 = *state_4;
                                let constructor = *constructor;
                                self::JavaScriptArrayTransformTransition::ArrayTransformNeedAuthority(Box::new(heap_4.clone()), Box::new({
                                    let source_ml_source2_nominal_field4888 = shared_constructor_field26.clone();
                                    {
                                        let source_ml_source2_nominal_field4890 = shared_constructor_field27.clone();
                                        {
                                            let source_ml_source2_nominal_field4892 = shared_constructor_field28.clone();
                                            {
                                                let source_ml_source2_nominal_field4894 = shared_constructor_field29.clone();
                                                {
                                                    let source_ml_source2_nominal_field4896 = shared_constructor_field30.clone();
                                                    {
                                                        let source_ml_source2_nominal_field4898 = shared_constructor_field31;
                                                        {
                                                            let source_ml_source2_nominal_field4900 = shared_constructor_field32;
                                                            {
                                                                let source_ml_source2_nominal_field4902 = shared_constructor_field33;
                                                                {
                                                                    let source_ml_source2_nominal_field4904 = shared_constructor_field34.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field4906 = shared_constructor_field35.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field4908 = shared_constructor_field36.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field4910 = shared_constructor_field37.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field4912 = shared_constructor_field38.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4914 = shared_constructor_field39.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4916 = shared_constructor_field40.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4918 = shared_constructor_field41.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field4920 = shared_constructor_field42.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field4922 = shared_constructor_field43.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field4924 = shared_constructor_field44.clone();
                                                                                                            self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field4888.clone(), Box::new(source_ml_source2_nominal_field4890.clone()), Box::new(source_ml_source2_nominal_field4892.clone()), Box::new(source_ml_source2_nominal_field4894.clone()), Box::new(source_ml_source2_nominal_field4896.clone()), source_ml_source2_nominal_field4898, source_ml_source2_nominal_field4900, source_ml_source2_nominal_field4902, Box::new(source_ml_source2_nominal_field4904.clone()), Box::new(source_ml_source2_nominal_field4906.clone()), Box::new(source_ml_source2_nominal_field4908.clone()), Box::new(source_ml_source2_nominal_field4910.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field4914.clone(), Some((Some(state_4.clone()),)), source_ml_source2_nominal_field4918.clone(), source_ml_source2_nominal_field4920.clone(), source_ml_source2_nominal_field4922.clone(), source_ml_source2_nominal_field4924.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }), Box::new(constructor.clone()))
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesAllocate(
                                heap_5,
                                state_5,
                                prototype,
                                length,
                            ) => {
                                let heap_5 = *heap_5;
                                let state_5 = *state_5;
                                let prototype = *prototype;
                                self::request_java_script_array_transform_allocation(
                                    heap_5.clone(),
                                    {
                                        let source_ml_source2_nominal_field4929 =
                                            shared_constructor_field26.clone();
                                        {
                                            let source_ml_source2_nominal_field4931 =
                                                shared_constructor_field27.clone();
                                            {
                                                let source_ml_source2_nominal_field4933 =
                                                    shared_constructor_field28.clone();
                                                {
                                                    let source_ml_source2_nominal_field4935 =
                                                        shared_constructor_field29.clone();
                                                    {
                                                        let source_ml_source2_nominal_field4937 =
                                                            shared_constructor_field30.clone();
                                                        {
                                                            let source_ml_source2_nominal_field4939 =
                                                                shared_constructor_field31;
                                                            {
                                                                let source_ml_source2_nominal_field4941 =
                                                                    shared_constructor_field32;
                                                                {
                                                                    let source_ml_source2_nominal_field4943 =
                                                                        shared_constructor_field33;
                                                                    {
                                                                        let source_ml_source2_nominal_field4945 = shared_constructor_field34.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field4947 = shared_constructor_field35.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field4949 = shared_constructor_field36.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field4951 = shared_constructor_field37.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4953 = shared_constructor_field38.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4955 = shared_constructor_field39.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4957 = shared_constructor_field40.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field4959 = shared_constructor_field41.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field4961 = shared_constructor_field42.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field4963 = shared_constructor_field43.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field4965 = shared_constructor_field44.clone();
                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field4929.clone(), Box::new(source_ml_source2_nominal_field4931.clone()), Box::new(source_ml_source2_nominal_field4933.clone()), Box::new(source_ml_source2_nominal_field4935.clone()), Box::new(source_ml_source2_nominal_field4937.clone()), source_ml_source2_nominal_field4939, source_ml_source2_nominal_field4941, source_ml_source2_nominal_field4943, Box::new(source_ml_source2_nominal_field4945.clone()), Box::new(source_ml_source2_nominal_field4947.clone()), Box::new(source_ml_source2_nominal_field4949.clone()), Box::new(source_ml_source2_nominal_field4951.clone()), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), source_ml_source2_nominal_field4955.clone(), Some((Some(state_5.clone()),)), Some((Some(prototype.clone()),)), source_ml_source2_nominal_field4961.clone(), source_ml_source2_nominal_field4963.clone(), source_ml_source2_nominal_field4965.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    prototype.clone(),
                                    length,
                                )
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesConstruct(
                                heap_6,
                                state_6,
                                constructor_2,
                                arguments_2,
                                fact,
                            ) => {
                                let heap_6 = *heap_6;
                                let state_6 = *state_6;
                                let constructor_2 = *constructor_2;
                                let fact = *fact;
                                match fact.clone() {
                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                        _,
                                        _,
                                        shared_constructor_field24,
                                        shared_constructor_field25,
                                        _,
                                    ) => match state_6.clone() {
                                        self::JavaScriptArraySpeciesState::ArraySpeciesState(
                                            _,
                                            shared_constructor_field20,
                                            shared_constructor_field21,
                                            shared_constructor_field22,
                                            shared_constructor_field23,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let shared_constructor_field22 =
                                                *shared_constructor_field22;
                                            let shared_constructor_field23 =
                                                *shared_constructor_field23;
                                            {
                                                let selected = {
                                                    let source_ml_source2_nominal_field4971 =
                                                        shared_constructor_field26.clone();
                                                    {
                                                        let source_ml_source2_nominal_field4973 =
                                                            shared_constructor_field27.clone();
                                                        {
                                                            let source_ml_source2_nominal_field4975 =
                                                                shared_constructor_field28.clone();
                                                            {
                                                                let source_ml_source2_nominal_field4977 =
                                                                    shared_constructor_field29
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field4979 =
                                                                        shared_constructor_field30
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field4981 = shared_constructor_field31;
                                                                        {
                                                                            let source_ml_source2_nominal_field4983 = shared_constructor_field32;
                                                                            {
                                                                                let source_ml_source2_nominal_field4985 = shared_constructor_field33;
                                                                                {
                                                                                    let source_ml_source2_nominal_field4987 = shared_constructor_field34.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4989 = shared_constructor_field35.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4991 = shared_constructor_field36.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4993 = shared_constructor_field37.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field4995 = shared_constructor_field38.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field4997 = shared_constructor_field39.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field4999 = shared_constructor_field40.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5001 = shared_constructor_field41.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field5003 = shared_constructor_field42.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field5005 = shared_constructor_field43.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field5007 = shared_constructor_field44.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override5011 = self::copy_java_script_array_constructor_fact(fact.clone());
                                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field4971.clone(), Box::new(source_ml_source2_nominal_field4973.clone()), Box::new(source_ml_source2_nominal_field4975.clone()), Box::new(source_ml_source2_nominal_field4977.clone()), Box::new(source_ml_source2_nominal_field4979.clone()), source_ml_source2_nominal_field4981, source_ml_source2_nominal_field4983, source_ml_source2_nominal_field4985, Box::new(source_ml_source2_nominal_field4987.clone()), Box::new(source_ml_source2_nominal_field4989.clone()), Box::new(source_ml_source2_nominal_field4991.clone()), Box::new(source_ml_source2_nominal_field4993.clone()), source_ml_source2_nominal_field4995.clone(), source_ml_source2_nominal_field4997.clone(), Some((Some(state_6.clone()),)), source_ml_source2_nominal_field5001.clone(), Some((Some(constructor_2.clone()),)), Some((Some(source_ml_source2_nominal_override5011.clone()),)), source_ml_source2_nominal_field5007.clone())
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
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
                                                match selected.clone() {
                                                        self::JavaScriptArrayTransformState::ArrayTransformState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14, shared_constructor_field15, shared_constructor_field16, shared_constructor_field17, shared_constructor_field18, shared_constructor_field19) => {
                                                            let shared_constructor_field2 = *shared_constructor_field2;
                                                            let shared_constructor_field3 = *shared_constructor_field3;
                                                            let shared_constructor_field4 = *shared_constructor_field4;
                                                            let shared_constructor_field5 = *shared_constructor_field5;
                                                            let shared_constructor_field9 = *shared_constructor_field9;
                                                            let shared_constructor_field10 = *shared_constructor_field10;
                                                            let shared_constructor_field11 = *shared_constructor_field11;
                                                            let shared_constructor_field12 = *shared_constructor_field12;
                                                            if ((shared_constructor_field25 && (shared_constructor_field24 == shared_constructor_field21)) && self::test_java_script_semantic_same_value(constructor_2.clone(), shared_constructor_field22.clone())) {
                                                                self::request_java_script_array_transform_allocation(heap_6.clone(), {
                                                                    let source_ml_source2_nominal_field5020 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field5022 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5024 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field5026 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field5028 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field5030 = shared_constructor_field6;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field5032 = shared_constructor_field7;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field5034 = shared_constructor_field8;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field5036 = shared_constructor_field9.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field5038 = shared_constructor_field10.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field5040 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5042 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field5044 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field5046 = shared_constructor_field14.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field5048 = shared_constructor_field15.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field5050 = shared_constructor_field16.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field5052 = shared_constructor_field17.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field5054 = shared_constructor_field18.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field5056 = shared_constructor_field19.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_override5059 = shared_constructor_field23.clone();
                                                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5020.clone(), Box::new(source_ml_source2_nominal_field5022.clone()), Box::new(source_ml_source2_nominal_field5024.clone()), Box::new(source_ml_source2_nominal_field5026.clone()), Box::new(source_ml_source2_nominal_field5028.clone()), source_ml_source2_nominal_field5030, source_ml_source2_nominal_field5032, source_ml_source2_nominal_field5034, Box::new(source_ml_source2_nominal_field5036.clone()), Box::new(source_ml_source2_nominal_field5038.clone()), Box::new(source_ml_source2_nominal_field5040.clone()), Box::new(source_ml_source2_nominal_field5042.clone()), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), source_ml_source2_nominal_field5046.clone(), source_ml_source2_nominal_field5048.clone(), Some((Some(source_ml_source2_nominal_override5059.clone()),)), source_ml_source2_nominal_field5052.clone(), source_ml_source2_nominal_field5054.clone(), source_ml_source2_nominal_field5056.clone())
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }, shared_constructor_field23.clone(), shared_constructor_field20)
                                                            } else {
                                                                self::JavaScriptArrayTransformTransition::ArrayTransformConstruct(Box::new(heap_6.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field5063 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field5065 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5067 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field5069 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field5071 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field5073 = shared_constructor_field6;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field5075 = shared_constructor_field7;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field5077 = shared_constructor_field8;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field5079 = shared_constructor_field9.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field5081 = shared_constructor_field10.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field5083 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5085 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field5087 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field5089 = shared_constructor_field14.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field5091 = shared_constructor_field15.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field5093 = shared_constructor_field16.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field5095 = shared_constructor_field17.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field5097 = shared_constructor_field18.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field5099 = shared_constructor_field19.clone();
                                                                                                                                            self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5063.clone(), Box::new(source_ml_source2_nominal_field5065.clone()), Box::new(source_ml_source2_nominal_field5067.clone()), Box::new(source_ml_source2_nominal_field5069.clone()), Box::new(source_ml_source2_nominal_field5071.clone()), source_ml_source2_nominal_field5073, source_ml_source2_nominal_field5075, source_ml_source2_nominal_field5077, Box::new(source_ml_source2_nominal_field5079.clone()), Box::new(source_ml_source2_nominal_field5081.clone()), Box::new(source_ml_source2_nominal_field5083.clone()), Box::new(source_ml_source2_nominal_field5085.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), source_ml_source2_nominal_field5089.clone(), source_ml_source2_nominal_field5091.clone(), source_ml_source2_nominal_field5093.clone(), source_ml_source2_nominal_field5095.clone(), source_ml_source2_nominal_field5097.clone(), source_ml_source2_nominal_field5099.clone())
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }), Box::new(constructor_2.clone()), arguments_2.clone(), Box::new(self::copy_java_script_array_constructor_fact(fact.clone())))
                                                            }
                                                        }
                                                    }
                                            }
                                        }
                                    },
                                }
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesThrow(
                                heap_7,
                                value,
                            ) => {
                                let heap_7 = *heap_7;
                                let value = *value;
                                self::JavaScriptArrayTransformTransition::ArrayTransformThrow(
                                    Box::new(heap_7.clone()),
                                    Box::new(value.clone()),
                                )
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesError(
                                heap_8,
                                error_class,
                                reason,
                            ) => {
                                let heap_8 = *heap_8;
                                self::JavaScriptArrayTransformTransition::ArrayTransformError(
                                    Box::new(heap_8.clone()),
                                    error_class.clone(),
                                    reason.clone(),
                                )
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(
                                heap_9,
                                reason_2,
                            ) => {
                                let heap_9 = *heap_9;
                                self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                                    Box::new(heap_9.clone()),
                                    reason_2.clone(),
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
