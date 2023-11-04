---
layout: post
title: Software Engineering 2023 Reading Journal
date: Oct 25, 2023
categories:
  - rust
  - code
description: summary of posts about Rust that I read in 2023
keywords: rust, posts, journal, 2023
author: Matt
tags:
  - rust
  - software-engineering
  - 💾
---


1. [Oct-25-2023 - Compile Times and Code Graphs](#oct-25-2023)
2. [Oct-26-2023 - Thread per core](#oct-26-2023)
3. [Oct-27-2023 - Introducing Glommio](#oct-27-2023)
4. [Nov-01-2023 - Why Async Rust](#nov-01-2023)

## Oct-25-2023

**Title:** [Compile Times and Code Graphs](https://blog.danhhz.com/compile-times-and-code-graphs)

Having an ideal code dependency structure improves compile times by reducing the number of
units that transitively depend on the unit that gets a code change.

The ideal structure looks like:

* A base layer of a small number of foundational crates that are mostly stable.
* A middle layer that fans out to implement various business logic.
* A top layer that produces a few binaries by fanning in the middle layer.

![ideal-code-dependency]( /images/ideal-code-dep.jpeg)

In terms of concretely implementation this structure results in:
* A base layer of core types, protobuf definitions etc.
* A middle layer of traits describing interfaces/services.
* A top layer implementing interfaces and services.

## Oct-26-2023

**Title:** [Thread per core](https://without.boats/blog/thread-per-core/)

What are the tradeoffs between having a thread-per-core (shared nothing) arch versus a multi
threaded with work stealing.

> Thread per core combines three big ideas:
(1) concurrency should be handled in userspace instead of using expensive kernel threads, 
(2) I/O should be asynchronous to avoid blocking per-core threads, and 
(3) data is partitioned between CPU cores to eliminate synchronization cost and data movement between CPU caches. 
It’s hard to build high throughput systems without (1) and (2), but (3) is probably only needed on really large multicore machines

_ [source](https://twitter.com/penberg/status/1705484076054904922) _


Shared-nothing arch has been [demonstrated](https://penberg.org/papers/tpc-ancs19.pdf) to be performant than traditional
shared-state arch. However, shared-nothing requires data partitioning which is harder than passing mutexes in shared-state arch.

Due to real life load imbalance (e.g. hot keys) some threads in the shared-nothing arch will
perform more work than others. The raison d'etre of work stealing is to redistribute some of the work from busy threads to idle threads. This however introduces synchronization and cache misses.

There are clear reasons for both approaches, the correct approach seems to be determined by the load
patterns.

## Oct-27-2023

**Title:** [Introducing Glommio](https://www.datadoghq.com/blog/engineering/introducing-glommio/)

[Glommio](https://github.com/DataDog/glommio/) is an async runtime by Datadog implemented using the thread-per-core shared-nothing paradigm.

[Sharding](https://en.wikipedia.org/wiki/Shard_(database_architecture)) is necessary to implement thread-per-core, it partitions the keyspace to the various threads. Also since only a single thread operates on a shard, there's no need for locking; even if there might be several async tasks on the same thread their data access will be serialized.

Glommio provides a task queue mechanism that allows to define latency requirements and execution time requirements. Tasks can then be queued based on their needs e.g. if long running and not latency sensitive etc.

Glommio uses [io_uring](https://en.wikipedia.org/wiki/Io_uring); Linux's new async IO API. It has three rings: main, latency and poll. The latency ring exists in order to prioritize latency-sensitive task !? I'm not sure, it wasn't clear to me.

Thread-per-core is possible in k8s if you:
- Avoid oversubscription of resources
- Assign pods to [specific nodes](https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/)
- Limit pods to [specific CPUs](https://kubernetes.io/docs/tasks/administer-cluster/cpu-management-policies/)
- Isolate hardware interrupts, so they are outside the pod

### Nov-01-2023

**Title:** [Why Async Rust](https://without.boats/blog/why-async-rust/)

This is an overview of why the async concurrency model exists the way it is today in rust.

Rust has a stackless coroutine implementation using state machines and has cooperative scheduling.  A coroutine is a function which can be paused and then later resumed

>Rust’s async/await syntax is an example of a stackless coroutine mechanism: an async function is compiled to a function which returns a `Future`, and that future is what is used to store the state of the coroutine when it yields control.

Rust experimented with green threading (a stackful coroutine mechanism) but abandoned it coz:
- green threads stack management has overheads. For both segmented stack and stack copying approaches.
- [Stack copying](https://blog.cloudflare.com/how-stacks-are-handled-in-go/) boils down to a garbage collected approach which isn't feasible in Rust since there's no GC.
- There's a huge cost for FFI - when integrating with other languages. 

Rust async programming has a "readiness-based" approach where a future is driven by an external executor. The future wakes the executor when it's ready. 
This external execution approach is very similar to how iterators are implemented - an "external" struct holds the state of iteration and references to the actual data structures being iterated over.

Iterator trait:
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```
Future trait:
```rust
enum Poll<T> {
    Ready(T),
    Pending,
}
trait Future {
    type Output;
    fn poll(&mut self) -> Poll<Self::Output>;
}
```

Future combinators require access to the future's state across await points. Pinning via the `Pin` type solved this by guaranteeing that a future won't move in memory and therefore is safe to have internal references.