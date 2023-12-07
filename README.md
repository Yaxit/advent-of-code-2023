# Advent of Code 2023 Solutions in Rust 🦀

![Advent of Code](https://img.shields.io/badge/Advent%20of%20Code-2023-brightgreen)
![Language](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

This repository contains my solutions for the Advent of Code 2023 challenges implemented in Rust. Advent of Code is an annual coding event with daily programming puzzles to solve in the days leading up to Christmas.

## Table of Contents

- [Advent of Code 2023 Solutions in Rust 🦀](#advent-of-code-2023-solutions-in-rust-)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Directory Structure](#directory-structure)
  - [Getting Started](#getting-started)
  - [Running Solutions](#running-solutions)
  - [License](#license)

## Introduction

Each directory in this repository corresponds to a specific day of the Advent of Code challenge. Inside each directory, you'll find the Rust source code for solving that day's puzzle. The solutions are organized in a way that makes it easy to navigate and understand.

## Directory Structure

The repository is organized as follows:

```
/advent_of_code_2023
  ├── 01
  │   ├── input.txt
  │   ├── input_short.txt
  │   ├── src
  │   │   ├── main.rs
  │   │   └── ...
  │   └── ...
  ├── 02
  │   ├── input.txt
  │   ├── input_short.txt
  │   ├── src
  │   │   ├── main.rs
  │   │   └── ...
  │   └── ...
  └── ...
```

- Each day's directory contains an `input.txt` file with the full puzzle input and an `input_short.txt` file with the initial snippet provided in the puzzle text.
- The `src` directory within each day contains the Rust source code (`main.rs`) for solving the puzzle.

## Getting Started

1. Clone this repository to your local machine:

```bash
git clone https://github.com/Yaxit/advent-of-code-2023.git
```

2. Change into the cloned directory:

```bash
cd advent-of-code-2023
```

3. Start exploring the solutions!

## Running Solutions

Navigate to the specific day's directory you want to run and execute the following command:

```bash
cd 01  # Change to the desired day's directory
cargo run -- input.txt
```

Replace `01` with the appropriate day's directory.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Feel free to use and modify the code as you see fit. Happy coding!
