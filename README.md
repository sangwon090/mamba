# mamba

An experimental programming language and its compiler.

## Language Specification
- The language specification is not yet written, but it is mostly based on the python's grammar.
- Read `test.mamba` to see how it looks like.
- A statically-typed language.
- Semicolon required;

## The Structure 
- Lexer
- Parser
  - Hand-written Parser (WIP)
  - Parser Generator (TODO)
- Code Generator
  - LLVM IRGen (WIP)
  - RISC-V Codegen (TODO)

## The Goal
- Start with little compiler knowledge but finish with plenty of knowledge.
- Write everything from scratch without depending on third-party libraries.
- Make a working compiler even if it does not have industry-level completeness.