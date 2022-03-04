# `megamerge`

The aim is to create a merge index / overlap list as fast as possible.

This may involve `numpy`/`scipy`, or we may delve into Rust.

The aim is to complete the part of the merge task that takes exponential time, such that python can perform the aggregations which take linear time.