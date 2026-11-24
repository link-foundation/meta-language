// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::ml_string;
use super::read_java_script_call_pending_array_callback_error;
use super::read_java_script_call_pending_array_species_error;
use super::read_java_script_call_pending_construction_error;
use super::read_java_script_call_pending_conversion_error;
use super::read_java_script_call_pending_intrinsic_error;
use super::read_java_script_call_pending_iterator_error;
use super::read_java_script_call_pending_object_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_host_capability_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode)
        == (self::ml_string::Text::from_units(vec![
            113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 104, 111, 115, 116, 32, 99, 97, 112,
            97, 98, 105, 108, 105, 116, 121,
        ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    suspended: bool,
    pending: self::JavaScriptCallPending,
) -> self::ml_string::Text {
    match pending.clone() {
        self::JavaScriptCallPending::WaitArgumentRest(_, _, _) => {
            self::read_java_script_call_pending_array_callback_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitIteratorProtocol(_, _, _, _, _, _) => {
            self::read_java_script_call_pending_iterator_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitArraySpread(_, _, _) => {
            self::read_java_script_call_pending_iterator_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitArrayIteratorNext(_, _, _) => {
            self::read_java_script_call_pending_iterator_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitCollectionIteratorResult(_, _, _) => {
            self::read_java_script_call_pending_iterator_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitArrayConcatenation(_, _, _, _) => {
            self::read_java_script_call_pending_array_species_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitArraySlice(_, _, _, _) => {
            self::read_java_script_call_pending_array_species_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitArrayTransform(_, _, _, _) => {
            self::read_java_script_call_pending_array_species_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitArraySearch(_, _, _) => {
            self::read_java_script_call_pending_array_callback_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitCollectionForEach(_, _, _) => {
            self::read_java_script_call_pending_array_callback_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitArrayCallback(_, _, _) => {
            self::read_java_script_call_pending_array_callback_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitObjectSpread(_, _, _) => {
            self::read_java_script_call_pending_object_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitObjectEnumeration(_, _, _) => {
            self::read_java_script_call_pending_object_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitPropertyDefinitions(_, _, _) => {
            self::read_java_script_call_pending_object_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitPropertyReflection(_, _, _) => {
            self::read_java_script_call_pending_object_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitInstanceOf(_, _, _) => {
            self::read_java_script_call_pending_object_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitObjectIntegrity(_, _, _) => {
            self::read_java_script_call_pending_object_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitObjectPrototype(_, _, _) => {
            self::read_java_script_call_pending_object_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitArrayAssignment(_, _, _) => {
            self::read_java_script_call_pending_array_callback_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitKey(_, _, _) => {
            self::read_java_script_call_pending_conversion_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitDefinition(_, _, _) => {
            self::read_java_script_call_pending_conversion_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitNumber(_, _, _) => {
            self::read_java_script_call_pending_conversion_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitText(_, _, _) => {
            self::read_java_script_call_pending_conversion_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitArrayCreation(_, _, _, _, _, _) => {
            self::read_java_script_call_pending_construction_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitConstruction(_, _, _, _) => {
            self::read_java_script_call_pending_construction_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitFunctionBind(_, _, _) => {
            self::read_java_script_call_pending_construction_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitFunctionText(_, _, _, _) => {
            self::read_java_script_call_pending_construction_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitConstructorCheck(_, _, _) => {
            self::read_java_script_call_pending_construction_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitObjectFromEntries(_, _, _, _) => {
            self::read_java_script_call_pending_object_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitPrimitiveWrapper(_, _) => {
            self::read_java_script_call_pending_construction_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitSymbolAllocation(_, _, _) => {
            self::read_java_script_call_pending_intrinsic_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitSymbolDescription(_, _) => {
            self::read_java_script_call_pending_intrinsic_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitFunctionNameSymbol(_, _, _, _, _) => {
            self::read_java_script_call_pending_intrinsic_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitSourceError(_, _, _, _) => {
            self::read_java_script_call_pending_intrinsic_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitErrorValue(_, _, _, _, _, _, _, _, _, _) => {
            self::read_java_script_call_pending_intrinsic_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitErrorIntrinsic(_) => {
            self::read_java_script_call_pending_intrinsic_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitHostCapability(_, _) => {
            self::read_java_script_call_pending_intrinsic_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        self::JavaScriptCallPending::WaitError(_, _, _) => {
            self::read_java_script_call_pending_intrinsic_error(
                program.clone(),
                heap.clone(),
                frame.clone(),
                fn_.clone(),
                suspended,
                pending.clone(),
            )
        }
        _ => self::ml_string::Text::from_units(vec![]),
    }
}
