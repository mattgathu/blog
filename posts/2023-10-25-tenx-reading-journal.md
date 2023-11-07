---
layout: post
title: Software 2023 Reading Journal
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
5. [Nov-05-2023 - Are You Sure You Want to Use MMAP in Your Database Management System? ](#nov-05-2023)
6. [Nov-06-2023 - Log Structured File Systems](#nov-06-2023)
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

### Nov-05-2023

**Title:** [Are You Sure You Want to Use MMAP in Your Database Management System?](https://db.cs.cmu.edu/mmap-cidr2022/)

I have been reading up on mmap and came across this paper. It covers correctness and perf issues that using mmap brings and how it's a terrible idea to use it. The paper focuses on DBMSes but I think the issues apply to a broader context.

**What is mmap?**
>_Memory-mapped_ (MMAP) file I/O is an OS-provided feature that maps the contents of a file on secondary storage into a program’s address space. The program then accesses pages via pointers as if the file resided entirely in memory. The OS transparently loads pages only when the program references them and automatically evicts pages if memory fills up.

In DBMSes it seems like a great substitute for [buffer bools](https://15445.courses.cs.cmu.edu/fall2021/notes/05-bufferpool.pdf); an in-memory cache of pages read from disk.

**What makes mmap attractive?**
*  OS manages file I/O for the application. App does not deal with low-level page management.
*  mmap avoids data copying to userspace as application can directly access data in OS page cache. This should be more performant than normal file I/O.

**How mmap works?**
![mmap-access]( /images/mmap-access.png)

1. A program calls mmap and receives a pointer to the memory-mapped file contents. 
2. The OS reserves part of the program’s virtual address space but does not load any part of the file. 
3. The program accesses the file’s contents using the pointer. 
4. The OS attempts to retrieve the page.
5. Since no valid mapping exists for the specified virtual address, the OS triggers a page fault to load the referenced part of the file from secondary storage into a physical memory page. 
6. The OS adds an entry to the page table that maps the virtual address to the new physical address. 
7. The initiating CPU core also caches this entry in its local translation lookaside buffer (TLB) to accelerate future accesses

**Why mmap sucks? 💩**
- Transaction safety
	- Since OS performs transparent paging, it can flush dirty pages to disk anytime. This is an issue for apps that perform transactions and need to control when updates are committed.
- I/O stalls
	- When a page fault occurs, mmap performs blocking I/O to fetch page from disk. mmap does not support async reads.
- Error Handling
	- For apps that perform data integrity checks e.g. using checksums, mmap makes it hard due to transparent page loads/evictions. There's no way to know if a page changed.
	- mmap I/O errors are more complicated to handle.
- Perf Issues
	- page table contention
	- single-threaded page eviction
	- TLB shootdowns: occur during page eviction when a cpu core needs to invalidate mappings in a remote TLB. Whereas flushing the local TLB is inexpensive, issuing interprocessor interrupts to synchronize remote TLBs can take thousands of cycles
 - Other
	- If the mmap is shared, contention on the mmap write lock leads to poor perf.
	- mmap is more expensive than read sys call for SSDs.
	- mmap implementations across platforms are not compatible. windows vs linux vs darwin
	- Since the OS manages the file mapping,  the in-memory data layout needs to match the physical representation on secondary storage, leading to wasted space and reduced I/O throughput. Notably no compression can be performed for data on disk.

### Nov-06-2023

**Title:** [Log Structured File Systems](https://pages.cs.wisc.edu/~remzi/OSTEP/file-lfs.pdf)

A chapter in  the [OSTEP book](https://pages.cs.wisc.edu/~remzi/OSTEP/). 

The basic idea of LFS is **all updates are written to disk sequentially to an append only log data structure.** LFS never overwrites existing data, it writes data segments to free locations.

**How is good write perf achieved?**

Using write buffering. The LFS tracks updates in memory and when there's a sufficient number of updates, it will write them sequentially to disk. This provides efficient use of disk bandwidth.
A good buffer size can be determined using this equation:

**D = (F/1-F) x R<sub>peak</sub> x T<sub>pos</sub>** where:

**D** - buffer size in MB
**F** - effective rate between 0 and 1 corresponding to desired efficiency; 0.9 => 90%
**R<sub>peak</sub>** - data transfer peak rate in MB/s
**T<sub>pos</sub>** - disk seek time to certain position

**How do you track file inodes in  LFS?**

LFS introduces an inode map structured for this. It maps the inode number to a disk address with the most recent version of the inode. LFS stores chunks of the inode map next to the data blocks on the log.  It then uses fixed addresses on disk (checkpoint regions) to store pointers to the latest pieces of the inode map.

**How is a data read from a file?**

The algorithm is simple:
1. Read checkpoint region
2. Read entire inode map and cache it memory (for subsequent reads)
3. Look up the file's inode and get  it's address on disk
4. Read data block from file.

**Garbage collection**

LFS has to clean old versions of files that have been replaces by newer versions.
LFS implements a cleaner that reads in a number of old (partially-used) segments, determines which blocks are live within these segments, and then write out a new set of segments with just the live blocks within them, freeing up the old ones for writing.

It uses a segment summary block to store info used to decide whether a data block is live or stale. It basically keeps the inode version of the file in the summary block and check if the inode is still the current one in the inode map. If the inode map returns a different inode then the data block is stale and can be garbage collected.

**When and which blocks to clean?**

The when is simple:
- periodic
- during idle time
- when disk is full
Which blocks to clean is harder:
- based on policies / heuristics
- hot/code segments approach: do not GC hot segments regularly since the keep changing.

**Crash recovery**

How to recover when system crashes as LFS is writing to disk?
- by carefully writing checkpoint regions
- uses timestamps in CR's head and tail.
- detects crashes by checking timestamps
- recovery starts at last CR and uses the **roll forward** technique
