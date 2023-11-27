# Advent of Code
This repository contains solutions for the advent of code, organized by year and
then challenge.

## Setup
A Nix flake with direnv is used to lock down dependencies. You may alternatively install toolchains separately.

If using Nix with direnv, setup is as simple as `direnv allow` after cloning the directory.

## Using Solutions
You'll need to grab inputs from adventofcode.com in order to run solutions.

### elisp
Drop inputs as "input.txt" within the same directory as the solution. Open
the main file with emacs, highlight the final line which executes the solution,
and:

```
M-x eval-buffer
C-x C-e
```

And the solution will be printed in the minibuffer.
