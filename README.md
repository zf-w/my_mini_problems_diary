## Learning Computer Science

This is Zhifeng's repository trying to archive some of his solved programming challenges in a somewhat structured way.

### Previous Attempt:

Previously, I tried to use another repository [(link)](https://github.com/zf-w/Learn-CS-Old) to archive those solved problems. I have added my thoughts on each problem into the root README file, but I found that's a somewhat unmanageable way in the long term. In addition, I didn't have a way to actually run those C++ codes in the previous repository.

From April of 2024, I start to slowly archive my reflections and thoughts on these daily problems.

## Topic Index

### Breadth First Search

lc0752

### Binary Tree Recursion

A type of Binary Tree or Tree structure problem that can be solved by recursively grouping sub-problem results. Normally, we need to carefully think about the information that needs to be collected from sub-problems.

lc0404, lc0979, lc0988

### Monotonic Stack

lc0085, lc0402

### Recursion and Backtracking

lc0140, lc1255

### Subsets

lc0078, lc2597

## Playground

### Rust: Using Option\<usize\> to replace i32 representation of index

lc0085

### Rust: Using unsafe pointers

lc0404

### Using an Index function to Swap between Current and Previous DP data

Usually, when we are applying Dynamic Programming techniques, we can reduce an matrix of DP data into two rows of data, representing the previous information and current information. How to switch between them can be an interesting problem to think about. Plain copying can be too costly. Manipulating pointers can be a way. Using an index function to calculate the position can be another way.

lc0514, lc2189

### Funny Breath-First-Search

lc2812

### Funny Bit manipulations

Sometimes, with special constraints on the input, we can use the extra space within the input data. For example, if we know the input will only use the first ten bits of an integer, we can use the remaining unused space.

lc0506

## To Review

lc310, lc834, lc3068
