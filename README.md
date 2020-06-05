# rust-sudoku

This project is a Sudoku solver written in Rust, and is capable of solving all
9x9 Sudoku puzzles I have tried it on in just a few milliseconds. The solver
represents the puzzle as an exact cover problem, and implements Donald Knuth's
[Algorithm X](https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X) and a version
of the [Dancing Links technique](https://en.wikipedia.org/wiki/Dancing_Links) to
find its solutions. (Read [more about the
implementation](#more-about-the-implementation) at the end of this README file.)

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

## More About the Implementation

I wrote this solver by way of starting to learn how to program in Rust. The
[Dancing Links technique](https://en.wikipedia.org/wiki/Dancing_Links) involves
modelling a sparse matrix as a set of circular, doubly linked lists (one for
each row and one for each column).

I quickly found, however, that due to Rust's strong restrictions on data
ownership, this wasn't going to be easy to implement! I began by playing with
[Rust's smart
pointers](https://doc.rust-lang.org/1.18.0/book/second-edition/ch15-00-smart-pointers.html),
but found the code very verbose, and that they were (not really unsuited for
this particular task
anyway)[https://rust-unofficial.github.io/too-many-lists/fifth.html]. The
alternative would have been to reach for (unsafe
Rust)[https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html] --- however, the
goal of this exercise for me was to learn Rust, and as a beginner it seemed
wrong to be reaching for the very last chapter of the manual to find out how to
do things that the language was not encouraging you to be doing!

Instead, I implement the links within the cover matrix itself. The matrix is
itself implemented using a Vector, but with methods to access each element at
each row and column position. Each matrix element is a ``struct`` that involves
four indexes to the next occupied elements in the matrix situated to the top,
left, right and bottom. These indexes are removed and reinstated in a similar
fashion to the pointers used in the doubly-linked lists of Knuth's original
algorithm.
