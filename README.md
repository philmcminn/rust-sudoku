# rust-sudoku

This project is a Sudoku solver written in Rust, and is capable of solving all
9x9 Sudoku puzzles I have tried it on in just a few milliseconds, and 16x16
puzzles in less than half a second. The solver represents the puzzle as an exact
cover problem, and implements Donald Knuth's [Algorithm
X](https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X) and a version of the
[Dancing Links technique](https://en.wikipedia.org/wiki/Dancing_Links) to find
its solutions. (Read [more about the
implementation](#more-about-the-implementation) at the end of this README file.)

## Building

To build the project you need to [download
Rust](https://www.rust-lang.org/tools/install) and use Cargo by simply
issuing the command

``cargo build --release``

in the root directory of the project. This will produce a binary file in the
``target/release`` directory that you can use to run the program on the command
line of your machine.

## Running

To run the program with a puzzle, you need to provide it with a puzzle file (see
the ``puzzles``) directory, e.g.:

``./sudoku puzzles/9x9/116.txt``

The command line will also take a string instead of a filename, e.g.:

``./sudoku .2..........6....3.7~4.8.........3..2.8..4..1.6..5.........1.7~8.5....9..........4.``

where a '.' indicates an empty cell in the puzzle. All numbers must be > 1.
Adjacent numbers must be separated by a non-digit character, i.e., '~' in the
above example. (This is because the program can be used solve puzzles greater
than 9x9 in size, and so more than one digit may be needed for a cell.)

By default, the program terminates on finding the first solution. It is
capable of finding all solutions, if there is more than one. To find all
solutions, add the ``--all`` switch after the puzzle input file name / string,
for example:

``./sudoku puzzles/4x4/empty.txt --all``

will find all 288 possibilities for an empty 4x4 puzzle.

## Example Output

```
$ sudoku puzzles/16x16/1.txt
Initial Sudoku (95/256) is:
---------------------------------------------------------
|  . 13 15  . |  .  .  .  3 |  .  . 12  . |  . 10 16  . |
|  .  .  .  . |  9 10  .  8 |  4  . 15  2 |  . 12  .  . |
|  3 16  .  6 |  .  .  . 13 |  .  . 10  . |  .  .  . 11 |
|  .  .  .  . | 11  . 15  . |  .  .  8  . |  .  2  .  6 |
---------------------------------------------------------
|  .  .  . 15 |  .  .  .  . |  .  .  .  . |  .  9 12  . |
|  .  .  9  7 |  2  .  .  . |  .  4  . 10 | 11 16 13  3 |
| 10  .  .  . |  .  1  6 16 |  .  .  .  . |  .  .  .  . |
| 13  .  . 12 | 15  .  .  . |  .  .  2 14 | 10  .  4  . |
---------------------------------------------------------
|  .  .  7  . |  8  .  .  . |  .  .  .  . |  .  .  .  . |
|  . 11  .  . |  .  2  3  . | 14  .  .  . |  7  . 10  . |
|  9 10  .  4 | 12 11  .  5 |  .  7  .  3 |  .  .  .  2 |
|  . 15  .  3 |  4  7  .  . |  . 10  .  . |  . 14  .  5 |
---------------------------------------------------------
|  .  6  .  . |  .  3 13  9 | 11  .  .  . |  .  .  8  . |
|  . 14 11  . |  .  .  .  1 |  .  .  . 16 |  .  5  . 12 |
| 12  .  . 13 |  5  .  .  . |  .  . 14  . |  .  . 11  9 |
|  .  5  .  . |  6  .  7  . |  .  8  .  . |  4  .  . 15 |
---------------------------------------------------------

Found 1 solution in 244.2ms:
---------------------------------------------------------
|  4 13 15  8 |  7  6  2  3 |  1 11 12  5 |  9 10 16 14 |
| 14  7  5 11 |  9 10 16  8 |  4  6 15  2 |  1 12  3 13 |
|  3 16  2  6 |  1  5 12 13 |  9 14 10  7 |  8  4 15 11 |
|  1  9 12 10 | 11 14 15  4 |  3 16  8 13 |  5  2  7  6 |
---------------------------------------------------------
| 11  2  6 15 |  3  4  5 10 | 16 13  7  1 | 14  9 12  8 |
|  5  1  9  7 |  2  8 14 12 | 15  4  6 10 | 11 16 13  3 |
| 10  4  8 14 | 13  1  6 16 | 12  9  3 11 |  2 15  5  7 |
| 13  3 16 12 | 15  9 11  7 |  8  5  2 14 | 10  6  4  1 |
---------------------------------------------------------
|  6 12  7  2 |  8 13 10 14 |  5 15  1  4 |  3 11  9 16 |
|  8 11  1  5 | 16  2  3 15 | 14 12  9  6 |  7 13 10  4 |
|  9 10 14  4 | 12 11  1  5 | 13  7 16  3 | 15  8  6  2 |
| 16 15 13  3 |  4  7  9  6 |  2 10 11  8 | 12 14  1  5 |
---------------------------------------------------------
| 15  6  4  1 | 14  3 13  9 | 11  2  5 12 | 16  7  8 10 |
|  7 14 11  9 | 10 15  8  1 |  6  3  4 16 | 13  5  2 12 |
| 12  8 10 13 |  5 16  4  2 |  7  1 14 15 |  6  3 11  9 |
|  2  5  3 16 |  6 12  7 11 | 10  8 13  9 |  4  1 14 15 |
---------------------------------------------------------
```

## More About the Implementation

I wrote this solver by way of starting to learn how to program in Rust. The
[Dancing Links technique](https://en.wikipedia.org/wiki/Dancing_Links) involves
modelling a sparse matrix as a set of circular, doubly linked lists (one for
each row and one for each column).

I quickly found, however, that due to Rust's strong restrictions on data
ownership, this wasn't going to be easy to implement! I began by playing with
[Rust's smart
pointers](https://doc.rust-lang.org/1.18.0/book/second-edition/ch15-00-smart-pointers.html),
but found the code very verbose, and that they were [not really unsuited for
this particular task
anyway](https://rust-unofficial.github.io/too-many-lists/fifth.html). The
alternative would have been to reach for [unsafe
Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) &mdash; however, the
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
