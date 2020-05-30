# quixote
Static code analysis for Rust programs.

## What is this?
This is an experiment to build a static code analysis tool for Rust. The intention is to parse Rust programs to then insert them into an SQL database to make it possible to manipulate them more easily.

This project is inspired by lgtm.com's CodeQL, except that instead of using a new language to query code it uses SQL and Postgres which should enable easy analysis of the most massive code bases.

## Does it work?
Not at the moment. It's in a very early development stage.

## When will it work?
Not sure about that. The aim is to use it to analyse [Rust's compiler](https://github.com/rust-lang/rust) and [Servo](https://github.com/servo/servo) which are two of the largest Rust code bases (both in the order of millions of lines of code).

## Can I contribute? What's the roadmap?
Yes! The roadmap is currently:
1. Build a parser for Rust programs
2. Loaded parsed Rust programs into an SQL database which has two steps.

    - Build an SQL model to represent Rust programs

    - Add code to load parsed output into a database.