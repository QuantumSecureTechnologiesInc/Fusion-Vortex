# Fusion Clustering

**Version:** 0.2.0  
**Type:** Algorithm Library  
**License:** MIT

## Overview

Fusion Clustering (`fusion_clustering`) implements high-performance clustering algorithms for data analysis and AI/ML workflows within Fusion. It includes implementations of K-Means, DBSCAN, and Hierarchical clustering.

## Features

- **K-Means**: Lloyd's algorithm with K-Means++ initialization
- **DBSCAN**: Density-based clustering for noisy data
- **Performance**: Parallelized distance calculations

## Usage

```rust
use fusion_clustering::{KMeans, Point};

let data = vec![Point::new(&[1.0, 2.0]), Point::new(&[5.0, 6.0])];
let kmeans = KMeans::new(2); // k=2
let clusters = kmeans.fit(&data)?;
```

## Dependencies

- `fusion_core`
- `rand`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
