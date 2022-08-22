# ANTLR code generation logic

The validator includes a parser for type expressions based on an ANTLR grammar.
Unfortunately, the ANTLR code generator is written in Java, and would thus add
a huge build dependency (a JRE) to the validator build environment. This is
especially problematic for the distribution of Cargo crates, which are
fundamentally source distributions that should not depend on anything other
than other Rust crates. Therefore, the generated files are checked in to git
and distributed with the crate, and regeneration must thus be done manually.
Call the generate.py script to do so.
