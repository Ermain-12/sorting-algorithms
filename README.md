#Sorting Algorithms
A sorting algorithm is an algorithm that puts elements of a list in a certain order. The most-used orders are numerical order and lexicographical order.<sup>1</sup>

#Bubble Sort
Bubble sort, sometimes referred to as sinking sort, is a simple sorting algorithm that repeatedly steps through the list to be sorted, compares each pair of adjacent items and swaps them if they are in the wrong order. The pass through the list is repeated until no swaps are needed, which indicates that the list is sorted.<sup>3</sup>

#Quicksort
Quicksort is a divide and conquer algorithm. Quicksort first divides a large array into two smaller sub-arrays: the low elements and the high elements. Quicksort can then recursively sort the sub-arrays.<sup>4</sup>

The steps are:

<ol>
  <li>Pick an element, called a pivot, from the array.</li>
  <li>Partitioning: reorder the array so that all elements with values less than the pivot come before the pivot, while all elements with values greater than the pivot come after it (equal values can go either way). After this partitioning, the pivot is in its final position. This is called the partition operation.</li>
  <li>Recursively apply the above steps to the sub-array of elements with smaller values and separately to the sub-array of elements with greater values.</li>
</ol> 

#Shell sort
Shellsort, also known as Shell sort or Shell's method, is an in-place comparison sort. It can be seen as either a generalization of sorting by exchange (bubble sort) or sorting by insertion (insertion sort). The method starts by sorting pairs of elements far apart from each other, then progressively reducing the gap between elements to be compared. Starting with far apart elements can move some out-of-place elements into position faster than a simple nearest neighbor exchange.<sup>5</sup>

#About this repository
This repository contains the implementation of some of the basic sorting algorithms on Python Rust and C programming languages. Intended for academic purposes only. Algorithms implemented in Python were taken from the textbook <b>Problem Solving with Algorithms and Data Structures using Python</b><sup>2</sup>, published on Interactive Python, which has a Creative Commons license.

References
<ol>
<li>Sorting algoritm: https://en.wikipedia.org/wiki/Sorting_algorithm</li>
<li>Problem Solving with Algorithms and Data Structures using Python: http://interactivepython.org/runestone/static/pythonds/index.html</li>
<li>Bubble sort: https://en.wikipedia.org/wiki/Bubble_sort</li>
<li>Quicksort: https://en.wikipedia.org/wiki/Quicksort</li>
<li>Shellsort: https://en.wikipedia.org/wiki/Shellsort</li>
</ol>
