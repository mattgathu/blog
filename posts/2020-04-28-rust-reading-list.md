---
layout: post
title: "Rust 2020 Reading Journal"
date: April 28, 2020
categories:
- rust
- code
description: 'summary of posts about Rust that I read in 2020'
keywords: rust, posts, journal, 2020
author: Matt
---

1. [Apr-27-2020 - Rust Analyzer: First Release](#apr-27-2020)
2. [Apr-30-2020 - Rust + nix = easier unix systems programming](#apr-30-2020)


## Apr-27-2020

**Title:** [Rust Analyzer: First Release][1]

[Rust Analyzer][2] is a IDE backend for rust that is implemented as a compiler frontend. It implements the
[language server protocol][3] and adopts lazy and incremental compilation features of rustc. 

It's now providing a better experience than [RLS][4] and wants to replace it as the official language server protocol
implementation for Rust.

It has dedicated plugins for [Vim][5], EMACS and VS Code. VS Code is the first class citizen here.

The bad
- Doesn't use rustc directly. _limited error detection_
- No cache persistence on disk.
- Everything is in-memory.

## Apr-30-2020

**Title:** [Rust + nix = easier unix systems programming][6]

I stumbled on this when searching for [nix][7] on Hacker News. It is actually
talking about another [nix][8] - a rust library that aims to provide a unified
interface for *nix platform APIs (Linux, BSD etc).

I like this qoute about systems programming:
> Systems programming is programming where you spend more time reading man pages than reading the internet.

Using a C program that use the `kill` and `fork` system calls together, Kamal explains how things
can get pretty bad due to C's return codes being conflated by `kill` and `fork`.

> Putting the two together, that program could really ruin our day. If the fork() call fails for
> some reason, we store -1 in child. Later, we call kill(-1, SIGKILL), which tries to kill all our
> processes, and most likely hose our login. Not even screen or tmux will save us!

Enter nix, and rust.

Nix uses Rust enums to model the result of calling `fork`. See [this][10].

This allows for pattern matching to deal with each case and prevent any conflation.

Also rust's `Result` type comes in handy for dealing with operations that might fail.

_Fun fact: Kamal was one of my mentors during [Rust Reach][9]_

[1]: https://rust-analyzer.github.io/blog/2020/04/20/first-release.html
[2]: https://github.com/rust-analyzer/rust-analyzer
[3]: https://microsoft.github.io/language-server-protocol/
[4]: https://github.com/rust-lang/rls
[5]: https://github.com/fannheyward/coc-rust-analyzer
[6]: http://kamalmarhubi.com/blog/2016/04/13/rust-nix-easier-unix-systems-programming-3/
[7]: https://nixos.org
[8]: https://github.com/nix-rust/nix/
[9]: https://blog.rust-lang.org/2017/06/27/Increasing-Rusts-Reach.html
[10]: https://github.com/nix-rust/nix/blob/5c8cdd005270557ceb91cdafc1eca7c971ee9219/src/unistd.rs#L162-L165
