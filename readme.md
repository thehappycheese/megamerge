# `megamerge` <!-- omit in toc -->

The aim is to create a merge index / overlap list as fast as possible.

This may involve `numpy`/`scipy`, or we may delve into Rust.

The aim is to complete the part of the merge task that takes exponential time, such that python can perform the aggregations which take linear time.

There are some experiments in the `notebooks` folder.

## Contents <!-- omit in toc -->

- [1. Theory](#1-theory)
  - [1.1. Example `segmentation` and `data`](#11-example-segmentation-and-data)
  - [1.2. Overlap Tables](#12-overlap-tables)
    - [1.2.1. psudocode](#121-psudocode)
    - [1.2.2. example overlap tables](#122-example-overlap-tables)
  - [1.3. The solution to the exponential memory problem](#13-the-solution-to-the-exponential-memory-problem)
- [2. Development Setup](#2-development-setup)

## 1. Theory

Given a target `segmentation`, and some `data` our goal is to efficiently merge the measures and categories of interest from `data` onto the `segmentation` according to the overlap. An overlap occurs when `index` is matching and the `from`/`to` ranges overlap.

We are interested in maintaining the index of the `segmentation`; ie the number of rows must stay the same, and the values in the columns `index`, `from`, and `to` should remain unaffected; we are just adding new columns to the `segmentation`.

There are a number of ways to aggregate the  *measure* and *category* columns in the `data` table as they move to the `segmentation` but we will get to those later.

### 1.1. Example `segmentation` and `data`

**segmentation**

|   id | index | from |   to |
| ---: | ----: | ---: | ---: |
|    0 |     0 |    0 |  100 |
|    1 |     0 |  100 |  200 |
|    2 |     0 |  200 |  300 |
|    3 |     0 |  300 |  400 |
|    4 |     1 |    0 |  100 |

**data**

|   id | index | from |   to | some_measure | some_category |
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

### 1.2. Overlap Tables

Our main challenge is efficiently computing the "overlap table". For each row in
`segments` where the `index` column matches the `index` column in `data`, we
want the `id` and `overlap_length` of rows in `data`. From this we can compute
the length weighted average, length weighted percentile, proportional sum etc.
(more detail later)

Notes about the psudocode:

1. In practice, this code does not work because it requires exponential memory 😦,<br> 
   **however** there is a strong motivation to make it work anyway,
   because:
2. The algorithim is not fussy about self-intersecting data or self-intersecting segments.
3. `signed_overlap` is calculated as an intermediate value.<br> This is
   interesting because it would allow us to perform aggregations based on
   proximity (e.g. count `data` within 1 kilometer of each `segments`. Count
   `data` touching each `segments` etc)
   - positive values are the overlap length
   - negative values are the distance between intervals
   - zero values indicate touching intervals

#### 1.2.1. psudocode

```python
# `outer_minimum` and `outer_maximum` are similar in function to the outer product
# except they compute the minimum / maximum of instead of multiplying elements.
result = []
for index in unique(segmentation["index"]):
    overlap_min = outer_maximum(
        segmentation.loc[index, "from"],
        data.loc[index, "from"]
    )
    overlap_max = outer_minimum(
        segmentation.loc[index,   "to"],
        data.loc[index,   "to"]
    )
    signed_overlap = overlap_max - overlap_min
    overlap        = maximum(0, signed_overlap)
    result.append(overlap)
```

#### 1.2.2. example overlap tables

The psudocode above would produce the following two overlap tables

`from`/`to` overlap table for `index == 0`

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

`from`/`to` overlap table for `index == 1`

|   id |    4 |
| ---: | ---: |
|    9 |   70 |
|   10 |   20 |

### 1.3. The solution to the exponential memory problem



## 2. Development Setup

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
 maturin build --interpreter python
```