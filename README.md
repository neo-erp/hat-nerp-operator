# NERP Operator HAT

Independent HATHQ HAT repository. It owns the vocabulary, context plan, exact-term reducer and procedure for `inspect-nerp-local`. It contains no credentials and grants no authority. Hatter consumes the immutable package; it does not link this crate.

The reducer accepts only canonical vocabulary IDs, rejects revision conflicts and returns `vocabulary-term-unknown` for every unrecognized value. Unknown input is never guessed or completed.

