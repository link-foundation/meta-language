The recorded Elixir source transformations now distinguish bare call arguments
from completed postfix operands, preserve capture and attribute precedence, and
attach `do` blocks to the enclosing call. Registered scanner policies also
reserve continuing operator line breaks before statement reduction. Syntactic
boundary preferences retain clause-body spans when another arrow clause follows.
