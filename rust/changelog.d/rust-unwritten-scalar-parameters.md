Admit an unused Rust `mut` declaration on an unshadowed directly owned scalar
Copy parameter when its entire closed function body contains no assignment
operators and still passes the portable body and source-origin checks. Written
parameters, borrowed or nominal values and unproved body forms remain refused;
this selected projection does not provide a general mutation or borrow model.
