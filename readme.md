# megamerge <!-- omit in toc -->

A python library accelerated by a rust binary for joining / merging tables with
an interval index.

This project is an improvement over a previous tool documented here
<https://github.com/thehappycheese/dtimsprep>

The part of the algorithm accelerated by rust has been minimized as it seems
preferable to maintain the majority of the code in python, where things are a
bit more concise and easy to read. Rust has been used to accelerate the part of
the task which runs in exponential time `O(n²)`, wrapped in a python library
which handles the linear-time `O(n)` portion of the task.

## Contents <!-- omit in toc -->

- [1. Installation](#1-installation)
- [2. Usage](#2-usage)
- [3. Theory](#3-theory)
  - [3.1. Example `segmentation` and `data`](#31-example-segmentation-and-data)
  - [3.2. Overlap Tables](#32-overlap-tables)
    - [3.2.1. psudocode](#321-psudocode)
    - [3.2.2. example overlap tables](#322-example-overlap-tables)
  - [3.3. The solution to the exponential memory problem](#33-the-solution-to-the-exponential-memory-problem)
- [4. Development Setup](#4-development-setup)

## 1. Installation

See the [releases](https://github.com/thehappycheese/megamerge/releases) page
for install / uninstall instructions

## 2. Usage

TODO

The api has not been finalized yet. The
[releases](https://github.com/thehappycheese/megamerge/releases) page has some
example code, and there is a
[notebooks](https://github.com/thehappycheese/megamerge/tree/main/notebooks)
folder with some example jupyter notebooks.

## 3. Theory

Given a target `segmentation`, and some `data` our goal is to efficiently merge
the measures and categories of interest from `data` onto the `segmentation`
according to the overlap. An overlap occurs when `key` is matching and the
`from`/`to` ranges overlap.

We are interested in maintaining the key of the `segmentation`; ie the number of
rows must stay the same, and the values in the columns `key`, `from`, and `to`
should remain unaffected; we are just adding new columns to the `segmentation`.

There are a number of ways to aggregate the *measure* and *category* columns in
the `data` table as they move to the `segmentation` but we will get to those
later.

### 3.1. Example `segmentation` and `data`

**segmentation**

|   id | key | from |   to |
| ---: | ----: | ---: | ---: |
|    0 |     0 |    0 |  100 |
|    1 |     0 |  100 |  200 |
|    2 |     0 |  200 |  300 |
|    3 |     0 |  300 |  400 |
|    4 |     1 |    0 |  100 |

**data**

|   id | key | from |   to | some_measure | some_category |
| ---: | ----: | ---: | ---: | -----------: | :-----------: |
|    0 |     0 |   50 |  140 |          1.0 |      "A"      |
|    1 |     0 |  140 |  160 |          2.0 |      "B"      |
|    2 |     0 |  160 |  180 |          3.0 |      "B"      |
|    3 |     0 |  180 |  220 |          4.0 |      "B"      |
|    4 |     0 |  220 |  240 |          5.0 |      "C"      |
|    5 |     0 |  240 |  260 |          5.0 |      "C"      |
|    6 |     0 |  260 |  280 |          6.0 |      "D"      |
|    7 |     0 |  280 |  300 |          7.0 |      "E"      |
|    8 |     0 |  300 |  320 |          8.0 |      "F"      |
|    9 |     1 |   10 |   80 |          9.0 |      "G"      |
|   10 |     1 |   80 |  120 |         10.0 |      "H"      |

### 3.2. Overlap Tables

Our main challenge is efficiently computing the `overlap_tables`. For each row in
`segments` where the `key` column matches the `key` column in `data`, we want
the `id` and `overlap_length` of rows in `data`. From this we can compute the
length weighted average, length weighted percentile, proportional sum etc. (more
detail later)

#### 3.2.1. psudocode

```python
# `outer_minimum` and `outer_maximum` are similar in function to the outer product
# except they compute the minimum / maximum of instead of multiplying elements
# and return a huge matrix which has the dimensions of the first and second input.
overlap_tables = []
for key in unique(segmentation["key"]):
    overlap_min = outer_maximum(
        segmentation.loc[key, "from"],
        data.loc[key, "from"]
    )
    overlap_max = outer_minimum(
        segmentation.loc[key,   "to"],
        data.loc[key,   "to"]
    )
    signed_overlap = overlap_max - overlap_min
    overlap        = maximum(0, signed_overlap)
    overlap_tables.append(overlap)
```

Notes about the psudocode:

1. In practice, this code does not work because it requires exponential memory 😦,<br> 
   **however** there is a strong motivation to make it work anyway,
   because:
2. The this approach is not fussy about self-intersections in `data` or
   self-intersections in `segments`
   - If multiple rows have overlapping `from`/`to` ranges (in either `data` or
     `segments`) then we are still able to aggregate all overlapping values from
     `data` proportional to the length by which they overlap the target row in
     `segments`.
   - Typically we are not interested in `segments` which are self-intersecting
     anyway, but it is fairly common to have self-intersecting `data`
3. `signed_overlap` is calculated as an intermediate value.<br> This is
   interesting because it would allow us to perform aggregations based on
   proximity (e.g. count `data` within 1 kilometer of each `segments`. Count
   `data` touching each `segments` etc)
   - positive values are the overlap length
   - negative values are the distance between intervals
   - zero values indicate touching intervals

#### 3.2.2. example overlap tables

The psudocode above would produce the following two overlap tables

`from`/`to` overlap table for `key == 0`

|   id |    0 |    1 |    2 |    3 |
| ---: | ---: | ---: | ---: | ---: |
|    0 |   50 |   40 |    0 |    0 |
|    1 |    0 |   20 |    0 |    0 |
|    2 |    0 |   20 |    0 |    0 |
|    3 |    0 |   20 |   20 |    0 |
|    4 |    0 |    0 |   20 |    0 |
|    5 |    0 |    0 |   20 |    0 |
|    6 |    0 |    0 |   20 |    0 |
|    7 |    0 |    0 |   20 |    0 |
|    8 |    0 |    0 |    0 |   20 |

`from`/`to` overlap table for `key == 1`

|   id |    4 |
| ---: | ---: |
|    9 |   70 |
|   10 |   20 |

### 3.3. The solution to the exponential memory problem

We still effectively calculate and check every cell in the overlap table. We
simply store our output in a sparse data-structure; discarding all zeros.

So far there are no smart optimizations that reduce the `O(n²)` time. These
require either

- some assumptions about the input data (ie sorted, non-self-intersecting etc)
  or
- some pre-treatment to facilitate a divide and conquer approach
  - This is an avenue we could explore in the future, but for now it seems our
    simple brute force method is still working.

For the moment we rely on the brute-force speed of iterating over the data in
rust; We can top-out all CPU cores to get things done fast. For practical
applications our speedup is over 100 times; tasks that took 20 minutes previously
should now take about 10 seconds. But there is no escaping the exponential time problem. For some worst imaginable cases this still isn't fast enough. For these cases the use of an interval index should be reconsidered in favor of traditional database keys and join operations which are super fast even on very large data.

## 4. Development Setup

On windows:

```powershell
python -m pip install pip --upgrade
python -m venv .env
conda deactivate
./.env/Scripts/activate
pip install maturin
maturin develop
```

Building on windows apparently requires the `--interpreter python` argument for 'reasons'

```python
maturin build --release --interpreter python
```