[![Project Status: Concept – Minimal or no implementation has been done yet, or the repository is only intended to be a limited example, demo, or proof-of-concept.](https://www.repostatus.org/badges/latest/concept.svg)](https://www.repostatus.org/#concept)
[![CI Status](https://github.com/jwodder/advent-of-code-rust/actions/workflows/test.yml/badge.svg)](https://github.com/jwodder/advent-of-code-rust/actions/workflows/test.yml)
[![codecov.io](https://codecov.io/gh/jwodder/advent-of-code-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/jwodder/advent-of-code-rust)
[![MIT License](https://img.shields.io/github/license/jwodder/advent-of-code-rust.svg)](https://opensource.org/licenses/MIT)

This is a collection of my solutions to [Advent of
Code](https://adventofcode.com) written in Rust.  Most problems were solved
well after the respective season was over, but I did them anyway in order to
gain experience in Rust and as general mental exercise.

The input files and answers are encrypted using
[git-crypt](https://www.agwa.name/projects/git-crypt/) so that others can't use
them to cheat, and also because [the author of Advent of Code has asked people
not to make the input files public][nosharing].  They are only kept here for
use in automated regression testing to ensure I don't break any solutions with
some later refactor.

Note that some problems (at time of writing, all involving analysis of
pseudo-assembly programs) were solved by working them out "manually" rather
than by writing code; their solutions take the form of writeups in `.md` files
encrypted with git-crypt.

[nosharing]: https://www.reddit.com/r/adventofcode/comments/7lesj5/is_it_kosher_to_share_puzzle_inputs_and_answers/drlt9am/
