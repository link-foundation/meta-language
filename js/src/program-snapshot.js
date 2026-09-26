import { LinkType } from './primitives.js';

/** Schema revision for source-buffer-independent program snapshots. */
export const PROGRAM_SNAPSHOT_SCHEMA_VERSION = 1;

/** Builds a JSON-compatible snapshot from exact retained source-token fragments. */
export function createProgramSnapshot(program) {
  const fragments = retainedFragments(program);
  return Object.freeze({
    schemaVersion: PROGRAM_SNAPSHOT_SCHEMA_VERSION,
    language: program.language,
    project: freezeProject(program.project),
    fragments: Object.freeze(fragments.map((fragment) => Object.freeze(fragment))),
  });
}

/** Validates a snapshot and returns the inputs needed to reconstruct a program. */
export function readProgramSnapshot(input) {
  let snapshot = input;
  if (typeof snapshot === 'string') {
    try {
      snapshot = JSON.parse(snapshot);
    } catch (error) {
      throw new TypeError(`invalid program snapshot JSON: ${error.message}`, { cause: error });
    }
  }
  if (!snapshot || typeof snapshot !== 'object' || Array.isArray(snapshot)) {
    throw new TypeError('invalid program snapshot: expected an object');
  }
  if (snapshot.schemaVersion !== PROGRAM_SNAPSHOT_SCHEMA_VERSION) {
    throw new TypeError(`invalid program snapshot schema version ${snapshot.schemaVersion}`);
  }
  if (typeof snapshot.language !== 'string' || snapshot.language.length === 0) {
    throw new TypeError('invalid program snapshot language');
  }
  const project = readProject(snapshot.project);
  if (!Array.isArray(snapshot.fragments)) {
    throw new TypeError('invalid program snapshot fragments');
  }

  let expectedStart = 0;
  let source = '';
  for (const fragment of snapshot.fragments) {
    if (!fragment || typeof fragment !== 'object' || Array.isArray(fragment)) {
      throw new TypeError('invalid program snapshot fragment');
    }
    const { byteStart, byteEnd, text } = fragment;
    if (
      !Number.isSafeInteger(byteStart) ||
      !Number.isSafeInteger(byteEnd) ||
      byteStart !== expectedStart ||
      byteEnd < byteStart ||
      typeof text !== 'string' ||
      utf8Length(text) !== byteEnd - byteStart
    ) {
      throw new TypeError(`invalid program snapshot fragment span at byte ${expectedStart}`);
    }
    source += text;
    expectedStart = byteEnd;
  }
  return { source, language: snapshot.language, project };
}

function retainedFragments(program) {
  const tokens = program.network.links()
    .filter((link) => link.metadata().linkType === LinkType.SourceToken)
    .filter((link) => !link.metadata().flags.isMissing)
    .filter((link) => link.metadata().span)
    .sort((left, right) => {
      const difference = left.metadata().span.byteRange.start - right.metadata().span.byteRange.start;
      return difference || left.id().asU64() - right.id().asU64();
    });
  const fragments = [];
  let coveredUntil = 0;
  for (const token of tokens) {
    const { start, end } = token.metadata().span.byteRange;
    if (start < coveredUntil) continue;
    if (start !== coveredUntil) {
      throw new TypeError(`program snapshot has an uncovered source byte at ${coveredUntil}`);
    }
    const text = token.metadata().term ?? '';
    if (utf8Length(text) !== end - start) {
      throw new TypeError(`program snapshot token span does not match its text at ${start}`);
    }
    fragments.push({ byteStart: start, byteEnd: end, text });
    coveredUntil = end;
  }
  if (fragments.map(({ text }) => text).join('') !== program.source) {
    throw new TypeError('program snapshot fragments do not reconstruct the analyzed source');
  }
  return fragments;
}

function readProject(project) {
  if (!project || typeof project !== 'object' || Array.isArray(project)) {
    throw new TypeError('invalid program snapshot project');
  }
  if (typeof project.root !== 'string') {
    throw new TypeError('invalid program snapshot project root');
  }
  return {
    root: project.root,
    files: readStringArray(project.files, 'files'),
    dependencies: readStringArray(project.dependencies, 'dependencies'),
    extensions: readStringArray(project.extensions, 'extensions'),
  };
}

function readStringArray(value, name) {
  if (!Array.isArray(value) || value.some((entry) => typeof entry !== 'string')) {
    throw new TypeError(`invalid program snapshot project ${name}`);
  }
  return [...value];
}

function freezeProject(project) {
  return Object.freeze({
    root: project.root,
    files: Object.freeze([...project.files]),
    dependencies: Object.freeze([...project.dependencies]),
    extensions: Object.freeze([...project.extensions]),
  });
}

function utf8Length(value) {
  return new TextEncoder().encode(value).length;
}
