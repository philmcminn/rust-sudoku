# rust-sudoku

This project is a Sudoku solver written in Rust, capable of solving most 9x9
Sudoku puzzles in a few milliseconds. The solver represents the puzzle as
an exact cover problem, and implements Donald Knuth's
[Algorithm X](https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X) and a version
of the [Dancing Links technique](https://en.wikipedia.org/wiki/Dancing_Links)
to find its solutions. More implementation details to follow soon!

To build the project you need to [download
Rust](https://www.rust-lang.org/tools/install) and build using Cargo. Issuing
``cargo build --release`` will produce a binary file in the ``target/release``
directory that you can use to run the program on the command line on your
machine.

To run the program with a puzzle, you need to provide it with a puzzle file (see
the ``puzzles``) directory, e.g.:

``./sudoku puzzles/116.txt``

## Known bugs

The file reader is flaky at present and is prone to crashing on valid files.
Fixes to follow soon...