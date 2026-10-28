Draft automatic lowering from checked JavaScript primitive expression functions
to shared registered call instructions. Lexical owner identities, branch blocks,
operand order and exact primitive payloads are retained. The compiler itself
is automatically translated to Rust. Unsupported allocation, captures and other
source forms remain explicit refusals; controls await the complete draft.

Extend the draft to ordinary record reads and property calls with their actual
receiver, plus checked lifted loops with parallel parameter updates and fixed
frame storage. Native semantics continue to derive from JavaScript source.
