This is a program for performing regression testing of my Advent of Code
solutions.  When run, all solutions with answers listed in a
`{year}/answers.csv` file are run concurrently in batches of 8, and their
output is compared to the listed answers.  If any solution fails to output its
expected answer within 30 seconds, the `regresstest` program fails.

As a prerequisite for running this program, all `{year}/answers.csv` and
`{year}/inputs/*` files must first be decrypted.
