---
layout: post
title: "Rust 2023 Reading Journal"
date: Oct 25, 2023
categories:
- rust
- code
description: 'summary of posts about Rust that I read in 2023'
keywords: rust, posts, journal, 2023
author: Matt
---


1. [Oct-25-2023 - Compile Times and Code Graphs](#oct-25-2023)

## Oct-25-2023

**Title:** [Compile Times and Code Graphs][1]

Having an ideal code dependency structure improves compile times by reducing the number of 
units that transitively depend on the unit that gets a code change.

The ideal structure looks like:

* A base layer of a small number of foundational crates that are mostly stable.
* A middle layer that fans out to implement various business logic.
* A top layer that produces a few binaries by fanning in the middle layer.

![ideal-code-dependency][ideal-code-dependency]

In terms of concretely implementation this structure results in:
* A base layer of core types, protobuf definitions etc.
* A middle layer of traits describing interfaces/services.
* A top layer implementing interfaces and services.


[1]: https://blog.danhhz.com/compile-times-and-code-graphs
[ideal-code-dependency]: /images/ideal-code-dep.jpeg
