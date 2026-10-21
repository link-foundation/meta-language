Historical translation checks now restore their exact immutable source inputs
through checked byte inverses after newer Number and UTF-16 behavior is added.
Original assertions and refusal cases remain intact. The parity source check
also recognizes the exact experimental Rust storage module registered in native
unit tests while continuing to reject drivers and unregistered sources.
