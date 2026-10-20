// Append one explicit observation model; preserve every original primitive byte.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {checkProjectData,writeProjectData} from './project-data.mjs';

/** Generate the closed Boolean callable premise from the frozen primitive basis. */
export function generateSharedBooleanFunctionModel({check=false}={}) {
  const original=captureGitArtifactCertificate('76e27c44c5854a2cb547aea5cef06ea80adf729c',
    'js/src/data/formal-concept-models.lino');
  const previous=readStructuredDataLino(original.blob.toString('utf8'));
  const declaration={
    typedProgramBasis:previous['checked typed program'],
    interpretation:'closed partial callable value over Boolean parameters and results',
    types:['bool'],expressionForms:['lit','var','let','if','call','unary','binary'],
    unary:['not'],binary:['and','or','eq','ne'],
    conjunction:'evaluate the left operand; evaluate and return the right operand only if the left is true',
    disjunction:'evaluate the left operand; evaluate and return the right operand only if the left is false',
    recursion:'closed declaration equations interpreted as partial functions',
    entry:'the actual owned target of the validated ordered selected-function wrapper',
    binding:'ordered parameters and actual owned lexical and recursive declaration references',
    interface:'callable value observation; owned declaration names and full names are retained origin aliases, not namespace observations',
    sourceSignaturePremise:previous['closed binary floating point function'].sourceSignaturePremise
      .replace('direct primitive f64 or Boolean','direct primitive Boolean'),
    exclusions:['numbers','strings','imports','captures','host effects','references','parameter defaults','parameter guards'],
    excludedObservations:['public namespace spelling','original source syntax','host stack exhaustion','machine resource limits'],
    assumptions:['JavaScript callers supply the declared primitive Boolean arguments',
      'host stack exhaustion and machine resource limits are outside this abstract callable interpretation'],
  };
  assert.equal(Object.hasOwn(previous,'closed boolean function value'),false);
  const value={...previous,'closed boolean function value':declaration};
  for(const runtime of ['js','rust']){
    const file=new URL('../../'+runtime+'/src/data/formal-concept-models.lino',import.meta.url);
    if(check)checkProjectData(file,value);else writeProjectData(file,value);
  }
  return value;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateSharedBooleanFunctionModel({check:process.argv.includes('--check')});
}
