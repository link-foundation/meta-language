Automatic error recovery retains its recorded partial trees within unchanged
resource bounds. Fixed lexical caching remains active for quiet token matching
outside repair rounds. Shifted zero-width scanner markers protect longer
continuations when they add no repairs; a repaired continuation cannot cross a
layout boundary into a later malformed command.
