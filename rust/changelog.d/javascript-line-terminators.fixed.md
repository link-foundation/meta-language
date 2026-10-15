Preserve active JavaScript statements after carriage return, line separator and
paragraph separator endings of line comments. Shared generated ECMAScript rules
also preserve return, break and continue boundaries and reject line breaks after
throw, including those within comments. Existing non-JavaScript comment policies
remain intact; native execution is verified in CI.
