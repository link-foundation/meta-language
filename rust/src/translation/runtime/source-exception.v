Definition ml_emit_source_exception {E : Type} (lines : list string) (next : list string * option E) : list string * option E := (List.app lines (fst next), snd next).
