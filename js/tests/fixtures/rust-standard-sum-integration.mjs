// Complete original Rust inputs and independent literal expectations. No
// observation or execution status is stored in this authoring fixture.
import {rustStandardSumSourceOriginals,rustStandardSumSourceRefusals} from './rust-standard-sum-source-rules.mjs';
import {rustStandardSumMethodOriginals} from './rust-standard-sum-methods.mjs';
import {rustStandardSumMethodSourceOriginals,rustStandardSumMethodSourceRefusals,rustStandardSumMethodContextOriginals} from './rust-standard-sum-method-source-originals.mjs';
import {rustArrayLookupOptionOriginals,rustArrayLookupOptionRefusals} from './rust-array-lookup-options.mjs';
import {rustStandardSumPredicateOriginals,rustStandardSumPredicateRefusals} from './rust-standard-sum-predicates.mjs';
import {sourceNamespaceOriginals,sourceNamespaceRefusals,sourceNamespaceVersionRange} from './rust-source-namespaces-draft.mjs';
import {rustStandardSumConversionOriginals,rustStandardSumConversionRefusals} from './rust-standard-sum-conversions.mjs';
import {genericAliasDefaultOriginals,genericAliasDefaultRefusals,genericAliasDefaultVersionRange} from './rust-generic-type-alias-defaults-draft.mjs';
import {rustStandardSumNestedOriginals,rustStandardSumNestedRefusals} from './rust-standard-sum-nested.mjs';
import {rustStandardSumOwnedOriginals,rustStandardSumOwnedRefusals} from './rust-standard-sum-owned-originals.mjs';
import {rustStandardSumCombinatorOriginals,rustStandardSumCombinatorRefusals} from './rust-standard-sum-combinators.mjs';
import {rustOwnedStandardSumObserverOriginals,rustOwnedStandardSumObserverRefusals} from './rust-owned-standard-sum-observers.mjs';
import {rustStandardSumOwnershipOriginals,rustStandardSumOwnershipRefusals} from './rust-standard-sum-ownership-transitions.mjs';
import {rustStandardSumGuardOriginals,rustStandardSumGuardRefusals} from './rust-standard-sum-guards.mjs';
import {rustOwnedStandardSumMethodOriginals,rustOwnedStandardSumMethodRefusals} from './rust-owned-standard-sum-methods.mjs';
import {rustStandardSumGuardAlternativeOriginals,rustStandardSumGuardAlternativeRefusals} from './rust-standard-sum-guard-alternatives.mjs';
import {rustOwnedStandardSumCallbackOriginals,rustOwnedStandardSumCallbackRefusals} from './rust-owned-standard-sum-callbacks.mjs';
import {callableDataOriginals,callableDataObservationRefusals,callableDataRecursiveRefusals,callableDataSourceVersionRange} from './callable-data-originals.mjs';
import {rustOwnedStandardSumMoveCaptureOriginals,rustOwnedStandardSumMoveCaptureRefusals} from './rust-owned-standard-sum-move-captures.mjs';
import {rustOwnedStandardSumBindOriginals,rustOwnedStandardSumBindRefusals} from './rust-owned-standard-sum-bind-callbacks.mjs';
import {rustStandardDefaultOriginals,rustStandardDefaultRefusals} from './rust-standard-default-values.mjs';
import {rustOwnedStandardSumPredicateOriginals,rustOwnedStandardSumPredicateRefusals} from './rust-owned-standard-sum-predicates.mjs';
import {rustStandardSumCopyOwnedCallbackOriginals,rustStandardSumCopyOwnedCallbackRefusals} from './rust-standard-sum-copy-owned-callbacks.mjs';
import {rustStandardSumOwnedPropagationOriginals,rustStandardSumOwnedPropagationRefusals} from './rust-standard-sum-owned-propagation.mjs';
import {rustStandardSumCloneOriginals,rustStandardSumCloneRefusals} from './rust-standard-sum-clones.mjs';
import {rustStandardSumPrimitiveQuestionOriginals,rustStandardSumPrimitiveQuestionRefusals} from './rust-standard-sum-primitive-question-conversions.mjs';

import {rustNominalStandardSumOwnershipOriginals,rustNominalStandardSumOwnershipRefusals} from './rust-nominal-standard-sum-ownership.mjs';

import {rustCopyClosureValueOriginals,rustCopyClosureValueRefusals} from './rust-copy-closure-values.mjs';

import {rustOwnedRecordUpdateOriginals,rustOwnedRecordUpdateRefusals} from './rust-owned-record-updates.mjs';

import {rustOwnedRecordPlaceOriginals,rustOwnedRecordPlaceRefusals} from './rust-owned-record-places.mjs';

import {rustOwnedRecordStateOriginals,rustOwnedRecordStateRefusals} from './rust-owned-record-state.mjs';
import {rustOwnedRecordEffectOriginals,rustOwnedRecordEffectRetainedOriginals,rustOwnedRecordEffectRefusals} from './rust-owned-record-effects.mjs';
import {rustOwnedRecordCompletionOriginals,rustOwnedRecordCompletionRefusals} from './rust-owned-record-completions.mjs';
import {rustOwnedRecursiveContainerOriginals,rustOwnedRecursiveContainerRefusals} from './rust-owned-recursive-containers.mjs';
import {rustSourceOwnerQualificationOriginals,rustSourceOwnerQualificationRefusals} from './rust-source-owner-qualification.mjs';
import {rustOwnedRecordLoopPreservedOriginals} from './rust-owned-record-loops.mjs';

export const rustStandardSumIntegrationVersions=['1.90.0','1.99.0'];
export const rustStandardSumIntegrationTargets=['JavaScript','Rust','Lean','Rocq'];
export const rustStandardSumIntegrationTransports=['original','text','binary'];
const retainedOriginalCompilerDeclaration=row=>row.originalCompilerOutcome?{
 ...row,originalCompilerExpectation:row.originalCompilerExpectation??{
  model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
  outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust source',
 },
}:row;
// Future native controls retain every original input and every assertion. This
// is runner work data, not claimed native evidence or a handwritten interpreter.
export const rustStandardSumIntegrationNativeControls=[
 'compile and run each complete original Rust source at the selected std snapshot',
 'compare original stdout exactly with its independent literal stdout',
 'run the generated native frontend on the identical complete original source',
 'compare actual native and JavaScript checked Links after both transports',
 'emit all four targets from each actual native/JavaScript restored typed root',
 'compile and run emitted Rust and compare exact stdout with the same literal',
 'retain all original refusal inputs and compare actual native/JavaScript diagnostics',
];

// Preserve the exact former draft refusal inputs and their former boundary
// text. These were never executed observations. The same inputs now have
// positive controls after their real expression consumers were integrated.
export const rustStandardSumIntegrationPreviousRefusals=[
 {name:'unlifted constructor argument cannot leak private completion',reasonIncludes:'question-mark',source:'fn later(value: Option<u8>) -> Option<u8> { Some(value?) } fn main() {}'},
 {name:'unlifted arithmetic cannot leak private completion',reasonIncludes:'question-mark',source:'fn later(value: Option<u8>) -> Option<u8> { let number = value? + 1; Some(number) } fn main() {}'},
 {name:'stacked propagation needs nested continuation lifting',reasonIncludes:'stacked propagation',source:'fn later(value: Option<Option<u8>>) -> Option<u8> { let number = value??; Some(number) } fn main() {}'},
];

export const rustStandardSumIntegrationOriginals=[
 ...rustStandardSumSourceOriginals.map(row=>({...retainedOriginalCompilerDeclaration(row),singleCallOwners:[]})),
 ...rustStandardSumMethodOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustStandardSumMethodSourceOriginals,
 ...rustArrayLookupOptionOriginals,
 ...rustStandardSumIntegrationPreviousRefusals.map(row=>({...row,name:'lifted former draft input: '+row.name,sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:''})),
 {name:'nested constructor and arithmetic preserve exact propagated values',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'0\n8\n',source:`
fn step(value: Option<u8>) -> Option<u8> { Some(value? + 1) }
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() { println!("{}", reveal(step(None))); println!("{}", reveal(step(Some(7)))); }
`},
 {name:'stacked questions preserve each distinct nested failure',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'0\n0\n7\n',source:`
fn step(value: Option<Option<u8>>) -> Option<u8> { let number = value??; Some(number) }
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() { println!("{}", reveal(step(None))); println!("{}", reveal(step(Some(None)))); println!("{}", reveal(step(Some(Some(7))))); }
`},
 {name:'earlier argument runs before a failed later question',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:['earlier','later'],stdout:'before\nearlier\nlater\n0\nbefore\nearlier\nlater\ncombine\n7\n',source:`
fn earlier() -> u8 { println!("earlier"); 3 }
fn later(flag: bool) -> Option<u8> { println!("later"); if flag { Some(4) } else { None } }
fn combine(left: u8, right: u8) -> u8 { println!("combine"); left + right }
fn step(flag: bool) -> Option<u8> { println!("before"); Some(combine(earlier(), later(flag)?)) }
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() { println!("{}", reveal(step(false))); println!("{}", reveal(step(true))); }
`},
 {name:'Boolean and keeps propagation in the selected right branch',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'before\nafter\n7\nbefore\n0\nbefore\nafter\n7\nbefore\ntaken\nafter\n7\n',source:`
fn step(flag: bool, value: Option<bool>) -> Option<u8> {
    println!("before");
    if flag && value? { println!("taken"); }
    println!("after"); Some(7)
}
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() {
    println!("{}", reveal(step(false, None))); println!("{}", reveal(step(true, None)));
    println!("{}", reveal(step(true, Some(false)))); println!("{}", reveal(step(true, Some(true))));
}
`},
 {name:'Boolean or keeps propagation in the selected right branch',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'before\ntaken\nafter\n7\nbefore\n0\nbefore\nafter\n7\n',source:`
fn step(flag: bool, value: Option<bool>) -> Option<u8> {
    println!("before");
    if flag || value? { println!("taken"); }
    println!("after"); Some(7)
}
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() { println!("{}", reveal(step(true, None))); println!("{}", reveal(step(false, None))); println!("{}", reveal(step(false, Some(false)))); }
`},
 {name:'match arm question executes only in its selected arm',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'9\n0\n7\n',source:`
fn step(flag: bool, value: Option<u8>) -> Option<u8> { let number = match flag { true => value?, false => 9 }; Some(number) }
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() { println!("{}", reveal(step(false, None))); println!("{}", reveal(step(true, None))); println!("{}", reveal(step(true, Some(7)))); }
`},
 {name:'match guard question preserves matching and lazy fallback order',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'before\nafter\n9\nbefore\n0\nbefore\nafter\n3\nbefore\nafter\n7\n',source:`
fn step(flag: bool, value: Option<bool>) -> Option<u8> {
    println!("before");
    let number = match flag { true if value? => 7, true => 3, false => 9 };
    println!("after"); Some(number)
}
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() {
    println!("{}", reveal(step(false, None))); println!("{}", reveal(step(true, None)));
    println!("{}", reveal(step(true, Some(false)))); println!("{}", reveal(step(true, Some(true))));
}
`},
 {name:'tuple fields preserve nested question and later typed literal',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'0\n16\n',source:`
fn step(value: Option<u8>) -> Option<(u8,u8)> { Some((value?, 9u8)) }
fn reveal(value: Option<(u8,u8)>) -> u8 { match value { None => 0, Some((left,right)) => left + right } }
fn main() { println!("{}", reveal(step(None))); println!("{}", reveal(step(Some(7)))); }
`},
 {name:'inferred tuple constructor retains complete actual child types',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'7\n',source:`
fn read() -> u8 { let value = Some((7u8,true)); match value { Some((number,true)) => number, Some((_,false)) => 0, None => 0 } }
fn main() { println!("{}", read()); }
`},
 {name:'if-let resolves constructor against its actual named scrutinee',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'0\n8\n',source:`
fn step(value: Option<u8>) -> Option<u8> { if let Some(number) = value { Some(number + 1) } else { None } }
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() { println!("{}", reveal(step(None))); println!("{}", reveal(step(Some(7)))); }
`},
 {name:'closure propagation returns to its nearest actual callable only',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'inner-before\nouter\n0\ninner-before\ninner-after\nouter\n8\n',source:`
fn step(value: Option<u8>) -> Option<u8> {
    let inner = || -> Option<u8> { println!("inner-before"); let number = value?; println!("inner-after"); Some(number + 1) };
    let answer = inner(); println!("outer"); answer
}
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() { println!("{}", reveal(step(None))); println!("{}", reveal(step(Some(7)))); }
`},
 {name:'let-else resolves constructor before callable fallback return',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'0\n8\n',source:`
fn step(value: Option<u8>) -> Option<u8> { let Some(number) = value else { return None; }; Some(number + 1) }
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() { println!("{}", reveal(step(None))); println!("{}", reveal(step(Some(7)))); }
`},
 {name:'standard sum import and constructor aliases keep their actual owner',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'0\n8\n',source:`
use std::option::Option as Maybe;
use std::option::Option::{Some as Present, None as Absent};
fn step(value: Maybe<u8>) -> Maybe<u8> { match value { Present(number) => Present(number + 1), Absent => Absent } }
fn reveal(value: Maybe<u8>) -> u8 { match value { Present(number) => number, Absent => 0 } }
fn main() { println!("{}", reveal(step(Absent))); println!("{}", reveal(step(Present(7)))); }
`},
 {name:'forward local constructor spelling remains actual user callable',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:['Some'],singleCallFunction:'before',stdout:'8\n',source:`
fn before() -> u8 { Some(7) }
fn Some(value: u8) -> u8 { value + 1 }
fn main() { println!("{}", before()); }
`},
 {name:'qualified local Option remains its actual nominal datatype',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],nominalOwner:'Option',stdout:'0\n7\n',source:`
#[derive(Clone, Copy)]
enum Option { Some(u8), None }
fn reveal(value: crate::Option) -> u8 { match value { crate::Option::Some(number) => number, crate::Option::None => 0 } }
fn main() { println!("{}", reveal(crate::Option::None)); println!("{}", reveal(crate::Option::Some(7))); }
`},
 {name:'local std module never authorizes a standard sum identity',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],nominalOwner:'std.option.Option',stdout:'0\n7\n',source:`
mod std { pub mod option { #[derive(Clone, Copy)] pub enum Option { Some(u8), None } } }
fn reveal(value: std::option::Option) -> u8 { match value { std::option::Option::Some(number) => number, std::option::Option::None => 0 } }
fn main() { println!("{}", reveal(std::option::Option::None)); println!("{}", reveal(std::option::Option::Some(7))); }
`},
 {name:'question operand executes once before selected continuation',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:['produce'],stdout:'before\noperand\n0\nbefore\noperand\nafter\n8\n',source:`
fn produce(flag: bool) -> Option<u8> {
    println!("operand");
    if flag { Some(7) } else { None }
}
fn step(flag: bool) -> Option<u8> {
    println!("before");
    let number = produce(flag)?;
    println!("after");
    Some(number + 1)
}
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() {
    println!("{}", reveal(step(false)));
    println!("{}", reveal(step(true)));
}
`},
 {name:'sequential question operands retain left-to-right early exit',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:['first','second'],stdout:'before\nfirst\n0\nbefore\nfirst\nsecond\nafter\n10\n',source:`
fn first(flag: bool) -> Option<u8> { println!("first"); if flag { Some(7) } else { None } }
fn second() -> Option<u8> { println!("second"); Some(3) }
fn step(flag: bool) -> Option<u8> {
    println!("before");
    let left = first(flag)?;
    let right = second()?;
    println!("after");
    Some(left + right)
}
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() {
    println!("{}", reveal(step(false)));
    println!("{}", reveal(step(true)));
}
`},
 {name:'direct tail propagates outer absence without flattening inner absence',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'0\n0\n7\n',source:`
fn flatten(value: Option<Option<u8>>) -> Option<u8> { value? }
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() {
    println!("{}", reveal(flatten(None)));
    println!("{}", reveal(flatten(Some(None))));
    println!("{}", reveal(flatten(Some(Some(7)))));
}
`},
 {name:'conditional propagation joins through actual callable completion carrier',sourceVersionRange:'>=1.90.0 <1.100.0',singleCallOwners:[],stdout:'before\n0\nbefore\n7\nafter\n9\nbefore\nafter\n9\n',source:`
fn step(flag: bool, value: Option<u8>) -> Option<u8> {
    println!("before");
    if flag { let number = value?; println!("{}", number); }
    println!("after");
    Some(9)
}
fn reveal(value: Option<u8>) -> u8 { match value { None => 0, Some(number) => number } }
fn main() {
    println!("{}", reveal(step(true, None)));
    println!("{}", reveal(step(true, Some(7))));
    println!("{}", reveal(step(false, None)));
}
`},
];

export const rustStandardSumIntegrationAdditionalOriginals=[
 ...rustStandardSumMethodContextOriginals,
 ...sourceNamespaceOriginals.map(row=>({singleCallOwners:[],sourceVersionRange:sourceNamespaceVersionRange,...row})),
 ...rustStandardSumConversionOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustStandardSumPredicateOriginals,
 ...genericAliasDefaultOriginals.map(row=>({singleCallOwners:[],sourceVersionRange:genericAliasDefaultVersionRange,...row})),
 ...rustStandardSumNestedOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustStandardSumOwnedOriginals,
 ...rustStandardSumCombinatorOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedStandardSumObserverOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustStandardSumOwnershipOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustStandardSumGuardOriginals,
 ...rustOwnedStandardSumMethodOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustStandardSumGuardAlternativeOriginals,
 ...rustOwnedStandardSumCallbackOriginals.map(row=>({singleCallOwners:[],...row})),
 ...callableDataOriginals.map(row=>({singleCallOwners:[],sourceVersionRange:callableDataSourceVersionRange,...row})),
 ...rustOwnedStandardSumMoveCaptureOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedStandardSumBindOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustStandardDefaultOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedStandardSumPredicateOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustStandardSumCopyOwnedCallbackOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustStandardSumOwnedPropagationOriginals.map(row=>({singleCallOwners:[],...retainedOriginalCompilerDeclaration(row)})),
 ...rustStandardSumCloneOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedRecordStateOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedRecordEffectOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedRecordEffectRetainedOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedRecordCompletionOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedRecursiveContainerOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustSourceOwnerQualificationOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedRecordLoopPreservedOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedRecordPlaceOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustOwnedRecordUpdateOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustCopyClosureValueOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustNominalStandardSumOwnershipOriginals.map(row=>({singleCallOwners:[],...row})),
 ...rustStandardSumPrimitiveQuestionOriginals.map(row=>({singleCallOwners:[],...row})),
];
rustStandardSumIntegrationOriginals.push(...rustStandardSumIntegrationAdditionalOriginals);

export const rustStandardSumIntegrationRefusals=[
 ...rustStandardSumSourceRefusals,
 ...rustStandardSumMethodSourceRefusals,
 ...rustArrayLookupOptionRefusals,
 {name:'question cannot return from a nonsum callable',reasonIncludes:'nearest actual callable',source:'fn bad(value: Option<u8>) -> u8 { let number = value?; number } fn main() {}'},
 {name:'closure cannot propagate into enclosing callable',reasonIncludes:'nearest actual callable',source:'fn bad(value: Option<u8>) -> Option<u8> { let local = || -> u8 { value? }; Some(local()) } fn main() {}'},
 {name:'unconstrained unsuffixed tuple child retains inference refusal',reasonIncludes:'complete',source:'fn later() -> u8 { let value = Some((1,true)); match value { Some((number,_)) => number, None => 0 } } fn main() {}'},
 ...sourceNamespaceRefusals.map(row=>({sourceVersionRange:sourceNamespaceVersionRange,...row})),
 // These new controls include absent argument syntax as well as type/domain
 // refusals. Existing rows keep their original semantic-only classification.
 ...rustStandardSumConversionRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row})),
 ...rustStandardSumPredicateRefusals,
 ...genericAliasDefaultRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],sourceVersionRange:genericAliasDefaultVersionRange,...row})),
 ...rustStandardSumNestedRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row})),
 ...rustStandardSumOwnedRefusals,
 ...rustStandardSumCombinatorRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row})),
 ...rustOwnedStandardSumObserverRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row})),
 ...rustStandardSumOwnershipRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row})),
 ...rustStandardSumGuardRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row})),
 ...rustOwnedStandardSumMethodRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row,
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
 outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust ownership'}})),
 ...rustStandardSumGuardAlternativeRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row})),
 ...rustOwnedStandardSumCallbackRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row,
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
 outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust source'}})),
 ...callableDataObservationRefusals.map(row=>({diagnosticKinds:['unsupported'],sourceVersionRange:callableDataSourceVersionRange,...row})),
 ...rustOwnedStandardSumMoveCaptureRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row,
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
 outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust source'}})),
 ...rustOwnedStandardSumBindRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row,
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
 outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust source'}})),
 ...rustStandardDefaultRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row,
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
 outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust source'}})),
 ...rustOwnedStandardSumPredicateRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row,
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
 outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust source'}})),
 ...rustStandardSumCopyOwnedCallbackRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row,
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
 outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust source'}})),
 ...rustStandardSumOwnedPropagationRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row,
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
 outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust source'}})),
 ...rustStandardSumCloneRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row,
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
 outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust source'}})),
 ...rustOwnedRecordStateRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...retainedOriginalCompilerDeclaration(row)})),
 ...rustOwnedRecordEffectRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...retainedOriginalCompilerDeclaration(row)})),
 ...rustOwnedRecordCompletionRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...retainedOriginalCompilerDeclaration(row)})),
 ...rustOwnedRecursiveContainerRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...retainedOriginalCompilerDeclaration(row)})),
 ...rustSourceOwnerQualificationRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...retainedOriginalCompilerDeclaration(row)})),
 ...rustOwnedRecordPlaceRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...retainedOriginalCompilerDeclaration(row)})),
 ...rustOwnedRecordUpdateRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...retainedOriginalCompilerDeclaration(row)})),
 ...rustCopyClosureValueRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...retainedOriginalCompilerDeclaration(row)})),
 ...rustNominalStandardSumOwnershipRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...retainedOriginalCompilerDeclaration(row)})),
 ...rustStandardSumPrimitiveQuestionRefusals.map(row=>({diagnosticKinds:['syntax','type','unsupported'],...row,
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',
 outcome:row.originalCompilerOutcome,category:row.originalCompilerOutcome==='accept'?'valid Rust source':'invalid Rust source'}})),
];
// These sources must pass the frontend and both codecs before each named
// backend independently refuses. A parse error cannot satisfy an emitter cell.
export const rustStandardSumIntegrationEmissionRefusals=callableDataRecursiveRefusals.map(row=>({
 sourceVersionRange:callableDataSourceVersionRange,...row,targets:['Rust','Lean'],diagnosticKinds:['unsupported'],
 originalCompilerExpectation:{model:'Rust original compiler expectation 1',versions:['1.90.0','1.99.0'],edition:'2024',outcome:'accept',category:'valid Rust source'},
}));
