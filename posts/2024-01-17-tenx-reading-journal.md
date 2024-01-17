---
layout: post
title: 2024 Software Engineering Reading Journal
date: Jan 17, 2024
categories:
  - rust
  - code
description: summary of posts about on software that I read in 2024
keywords: rust, posts, journal, 2024, software
author: Matt
tags:
  - rust
  - software-engineering
  - 💾
  - forth
---

1. [Jan-16-2024 - Programming a Problem Oriented Language](#jan-16-2024)

## Jan-16-2024

**Title** [Programming a Problem Oriented Language](https://colorforth.github.io/POL.htm)

The opening chapters of this book by Chuck Moore cover basic principles around software writing. 

He puts forward several ideas:

1. **Keep It Simple**

As more capabilities are added to a program, its complexity increases. Maintaining compatibility among the capabilities and internal consistency can get out of hand.

2. **Do not speculate**

Do not put code in your program that _might_ be used.  It's better to write future code in the future and keep only code that is currently used. Other people working on your program later might not see your "future" plans; it's best to leave them out.

3. **Write it yourself**

Write your own subroutines instead of using standard ones or libraries. What I got from this point is **understand your code**; sometimes your own implementation will best serve your particular use case. Libraries are generic and might not be very efficient in your narrow application; you could possibly write faster code.

> Before you can write your own subroutine, you have to know how. This means, to be practical, that you have written it before; which makes it difficult to get started. But give it a try. After writing the same subroutine a dozen times on as many computers and languages, you'll be pretty good at it.

**Programs vs Input**

> The simplest possible program is one that has no input.

Chuck views input as information that controls a program, and, in that sense, a program without input has encapsulated all its complexity internally. On the other hand, a program that expects input has to contend with the complexity of the input.

> In order to sharpen your recognition of input, let me describe a program that has input. Consider a program that fits a smooth curve through measured data points. It needs a lot of information in order to run: the number of data points, the spacing between points, the number of iterations to perform, perhaps even which function to fit. This information might be built into the program; if it is not, it must be supplied as input. The measured data itself, the object of the entire program, is not input; but must be accompanied by input in order to be intelligible.