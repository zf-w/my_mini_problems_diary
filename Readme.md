## Learning Computer Science

This is Zhifeng's repository trying to archive some of his solved programming challenges in a somewhat structured way.

### Previous Attempt:

Previously, I tried to use another repository [(link)](https://github.com/zf-w/Learn-CS-Old) to archive those solved problems. I have added my thoughts on each problem into the root README file, but I found that's a somewhat unmanageable way in the long term. In addition, I didn't have a way to actually run those C++ codes in the previous repository.

From April of 2024, I start to slowly archive my reflections and thoughts on these daily problems.

## Topic Index

### Breadth First Search

lc0752

### Binary Search

lc1482, lc1552

### Binary Tree Recursion

A type of Binary Tree or Tree structure problem that can be solved by recursively grouping sub-problem results. Normally, we need to carefully think about the information that needs to be collected from sub-problems.

lc0404, lc0979, lc0988, lc1530

### Dynamic Programming

lc0552

### Graph / Distances

lc1334, lc2976

### Monotonic Stack

lc0085, lc0402

### Prefix Tree

lc0648

### Recursion and Backtracking

lc0140, lc1255

### Sliding Window

lc1052, lc1248, lc1438

### Stack

lc2751

### Subsets

lc0078, lc2597

### Topology BFS

lc2392

### Use Tree Map to Calculate Maximum and Minimum Value

lc1438

## Playground

### Funny Breath-First-Search

lc2812

### Funny Bit manipulations

Sometimes, with special constraints on the input, we can use the extra space within the input data. For example, if we know the input will only use the first ten bits of an integer, we can use the remaining unused space.

lc0506

### Interesting

lc0260

### Rust: Iterators

lc3110

### Rust: Using Option\<usize\> to replace i32 representation of index

lc0085

### Rust: Using unsafe pointers

lc0404

### Using an Index function to Swap between Current and Previous DP data

Usually, when we are applying Dynamic Programming techniques, we can reduce an matrix of DP data into two rows of data, representing the previous information and current information. How to switch between them can be an interesting problem to think about. Plain copying can be too costly. Manipulating pointers can be a way. Using an index function to calculate the position can be another way.

lc0514, lc2189

## Intuition Tree

0. What's the return type...: [1]
1. When the returning value is number...: [2,4]
2. and the question is asking about the minimum or maximum value of something...: [3]
3. and if the in-question problem solution can be represented by a number, and we can fairly easily check if a number works:
   This question might be about Binary Search on that number.
4. the question is asking about `Substring`...:[5]
5. with Maximum Length of Something:
   This question might be about `Sliding Window`.

## Interesting

lc2418

## To Review

lc0310,lc0330, lc0834, lc0959, lc1823, lc3068

## Busy and Copied

lc0959, lc1568
