# rust-sudoku

This project is a Sudoku solver written in Rust, capable of solving most 9x9
Sudoku puzzles in a few milliseconds. The solver represents the puzzle as
an exact cover problem, and implements Donald Knuth's
[Algorithm X](https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X) and a version
of the [Dancing Links technique](https://en.wikipedia.org/wiki/Dancing_Links)
to find its solutions. More implementation details to follow soon!

## Building

To build the project you need to [download
Rust](https://www.rust-lang.org/tools/install) and build using Cargo. Issuing
``cargo build --release`` will produce a binary file in the ``target/release``
directory that you can use to run the program on the command line on your
machine.

## Running

To run the program with a puzzle, you need to provide it with a puzzle file (see
the ``puzzles``) directory, e.g.:

``./sudoku puzzles/116.txt``

The command line will also take a string instead of a filename, e.g.:

``./sudoku .2..........6....3.7~4.8.........3..2.8..4..1.6..5.........1.7~8.5....9..........4.``

where a '.' indicates an empty cell in the puzzle. Adjacent numbers must be
separated by a non-digit character, i.e., '~' in the above example. This is
because the program can be used solve puzzles greater than 9x9 in size, and
so more than one digit may be needed for a cell.

By default, the program terminates on finding the first solution. It is
capable of finding all solutions, if there is more than one. To find all
solutions, add the ``--all`` switch after the puzzle input file name / string, e.g.

``./sudoku puzzles/4x4-empty.txt -all``

will find all 288 possibilities for an empty 4x4 puzzle.
