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
2. [Jan-24-2024 - What is Complexity Science?](#jan-24-2024)

## Jan-16-2024

**Title** [Programming a Problem Oriented Language](https://colorforth.github.io/POL.htm)

#forth 

The opening chapters of this book by Chuck Moore cover basic principles around software writing. 

He puts forward several ideas:

1. **Keep It Simple**

As more capabilities are added to a program, its complexity increases. Maintaining compatibility among the capabilities and internal consistency can get out of hand.

2. **Do not speculate**

Do not put code in your program that _might_ be used.  It's better to write future code in the future and keep only code that is currently used. Other people working on your program later might not see your "future" plans; it's best to leave them out.

3. **Write it yourself**

Write your own subroutines instead of using standard ones or libraries. What I got from this point is **understand your code**; sometimes your own implementation will best serve your particular use case. Libraries are generic and might not be very efficient in your narrow application; you could possibly write faster code.

> Before you can write your own subroutine, you have to know how. This means, to be practical, that you have written it before; which makes it difficult to get started. But give it a try. After writing the same subroutine a dozen times on as many computers and languages, you'll be pretty good at it.

4. **Use names with mnemonic values**

Use names that are easy to remember and with the correct grammatical connotations: nouns for variables, verbs for functions. Avoid clever words whose mnemonic value is too subjective. Use comments sparingly, your code should be self documenting. Comments should say _what_ the program is doing, not the _how_. The how should be obvious from the code.

**Programs vs Input**

> The simplest possible program is one that has no input.

Chuck views input as information that controls a program, and, in that sense, a program without input has encapsulated all its complexity internally. On the other hand, a program that expects input has to contend with the complexity of the input.

> In order to sharpen your recognition of input, let me describe a program that has input. Consider a program that fits a smooth curve through measured data points. It needs a lot of information in order to run: the number of data points, the spacing between points, the number of iterations to perform, perhaps even which function to fit. This information might be built into the program; if it is not, it must be supplied as input. The measured data itself, the object of the entire program, is not input; but must be accompanied by input in order to be intelligible.



## Jan-24-2024
**Title:** [What is Complexity Science?](https://complexityexplained.github.io/)

#complexity-science 

This is a general explainer on complex systems. 

A complex system is a large collection of components that interact locally but self-organize to exhibit non-trivial properties at larger scales that can't be explained nor predicted from knowing the individual components alone.

> "_There's no love in a carbon atom,_ _No hurricane in a water molecule,_ _No financial collapse in a dollar bill._" - Peter Dodds

**Interactions**

Components in a complex system interact in multiple ways. These interactions can generate new information that makes it hard to study the components individually or predict their future.
The components can also be systems in of themselves, leading to a **system of systems**.

Examples:
* Computer networks
* Neurons in the human brain

**Emergence**

The interaction of components in a complex system generate new properties that can not be deduced from the properties of the individual components. This phenomenon is emergence.
*The whole is greater than the sum of its parts.*

Examples:
* air and water vapor forming a tornado
* billions of neurons in the brain producing intelligence

**Dynamics**

Systems can be analyzed in terms of the changes of their states over time. A state is a set of variables that best characterized the system. Complex systems exhibit nonlinear change that is not proportional to time, the system's current state nor changes in the environment.

A system's stability varies from stable one that remain unchanged by small perturbation to un-stable ones that are disrupted by small perturbations. In some instances small changes in the environment completely changes the system; these are bifurcations, phase transitions or tipping points.

Complex systems can be path dependent - its future is dependent on its present and past states.

Example:
* Financial volatility in the stock market


**Self-Organization**

The interaction of the components of a complex system may produce global patterns or behaviour; self-organization, without central or external control.
Self-organization can produce physical structures such as crystalline patterns in materials or dynamic behaviours such as shoaling in fish.

Example:
* starlings flocking in complex patterns

**Adaptation**

Complex systems are always adaptive and responding to the environment. This adaptation can happen:
* cognitively
* by learning and psychological development
* through information sharing
* by evolutionary means: genetic variation and natural selection

Adaptation gives complex systems the ability to be:
* adaptive: able to change in order to remain functional and survive.
* robust: withstand perturbations.
* resilient: go back to previous state after perturbation

Example:
* immune system learning about pathogens

**Interdisciplinarity**

Complex systems have universality - many systems in different domains have common underlying features that can be described using the same models or principles.
Complex systems appear in all scientific and professional domains.

Example:
* common properties of information processing systems: nervous system, internet

**Methods**

Mathematical and computational methods are useful tools to study complex systems.

> Computers are also used to analyze massive data coming from complex systems to reveal and visualize hidden patterns that are not visible to human eyes. These computational methods can lead to discoveries that then deepen our understanding and appreciation of nature. 

Example:
* mathematical and computer models of the brain

