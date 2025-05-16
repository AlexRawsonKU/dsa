# KU's EECS 330 Course, (Re)implemented in Idiomatic Rust

This repository contains relatively idiomatic Rust re-implementations of all of the data structures I implemented in the lab portion of EECS 330 at the University of Kansas.
This does not include data structures introduced in the lecture section of the course that were not implemented in the lab section.

As an additional challenge, the implementation is `#![no_std]`, and does not rely on any standard library heap data structures (although it may rely on or implement standard library traits, such as `Iterator` or `Hash`, so that the code can be used for generic types, as Rust does not support C++-style template classes.)

## Included Data Structures

- [ ] MyBinaryHeap
- [ ] MyBST
- [ ] MyDisjointSets
- [ ] MyGraph
- [ ] MyHashTable
- [ ] MyInfixCalculator
- [ ] MyLinkedList
- [x] MyBox (originally MyNumber, but nothing actually constrains the type to be a number)
- [ ] MyQueue
- [ ] MyStack
- [ ] MyVector
