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


[1]: https://rust-analyzer.github.io/blog/2020/04/20/first-release.html
[2]: https://github.com/rust-analyzer/rust-analyzer
[3]: https://microsoft.github.io/language-server-protocol/
[4]: https://github.com/rust-lang/rls
[5]: https://github.com/fannheyward/coc-rust-analyzer