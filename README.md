<p align="center">
<h1 align="center"><code>generational-cache</code></h1>
</p>

<p align="center">
  <a href="https://github.com/arindas/generational-cache/actions/workflows/rust-ci.yml">
    <img src="https://github.com/arindas/generational-cache/actions/workflows/rust-ci.yml/badge.svg">
  </a>
  <a href="https://codecov.io/gh/arindas/generational-cache"> 
    <img src="https://codecov.io/gh/arindas/generational-cache/branch/main/graph/badge.svg?token=44d8cAmnlE"/> 
  </a>
  <a href="https://crates.io/crates/generational-cache">
  <img src="https://img.shields.io/crates/v/generational-cache" />
  </a>
  <a href="https://github.com/arindas/generational-cache/actions/workflows/rustdoc.yml">
    <img src="https://github.com/arindas/generational-cache/actions/workflows/rustdoc.yml/badge.svg">
  </a>
</p>

<p align="center">
Generational Arena based cache impls. in 100% safe, [no_std] compatible Rust.
</p>

## Usage

`generational-cache` is a library crate. You may include it in your `Cargo.toml` as follows:

```toml
[dependencies]
generational-cache = "0.1.2"
```

Refer to latest git [API Documentation](https://arindas.github.io/generational-cache/docs/generational_cache/)
or [Crate Documentation](https://docs.rs/generational-cache) for more details.

### Examples

#### `#1`: Generational arena based LRU cache implementation
```rust
#[no_std]

use generational_cache::prelude::*;

const CAPACITY: usize = 3;

// users can choose between different map and vector implementations
let mut cache = LRUCache::<_, i32, u64, AllocBTreeMap<_, _>>::with_backing_vector(Array::<_, CAPACITY>::new());

cache.insert(-1, 1).unwrap();
cache.insert(-2, 2).unwrap();
cache.insert(-3, 3).unwrap();

assert_eq!(cache.least_recent().unwrap(), (&-1, &1));
assert_eq!(cache.most_recent().unwrap(), (&-3, &3));

assert_eq!(cache.insert(-4, 4).unwrap(), Eviction::Block { key: -1, value: 1});

assert_eq!(cache.least_recent().unwrap(), (&-2, &2));
assert_eq!(cache.most_recent().unwrap(), (&-4, &4));

assert_eq!(cache.insert(-2, 42).unwrap(), Eviction::Value(2));

assert_eq!(cache.least_recent().unwrap(), (&-3, &3));
assert_eq!(cache.most_recent().unwrap(), (&-2, &42));

match cache.remove(&-42) {
  Ok(Lookup::Miss) => {},
  _ => unreachable!("Wrong result on cache miss"),
};

match cache.query(&-42) {
  Ok(Lookup::Miss) => {},
  _ => unreachable!("Wrong result on cache miss"),
};

assert_eq!(cache.query(&-3).unwrap(), Lookup::Hit(&3));

assert_eq!(cache.least_recent().unwrap(), (&-4, &4));
assert_eq!(cache.most_recent().unwrap(), (&-3, &3));

assert_eq!(cache.remove(&-2).unwrap(), Lookup::Hit(42));

match cache.query(&-2) {
  Ok(Lookup::Miss) => {},
  _ => unreachable!("Wrong result on cache miss"),
};

// zero capacity LRUCache is unusable
let mut cache = LRUCache::<_, i32, u64, AllocBTreeMap<_, _>>::with_backing_vector(Array::<_, 0_usize>::new());

match cache.insert(0, 0) {
    Err(LRUCacheError::ListUnderflow) => {}
    _ => unreachable!("Wrong error on list underflow."),
};

```

(… we plan on adding more cache implementations in the future).

## License
This repository is licensed under the MIT License. See
[License](https://raw.githubusercontent.com/arindas/generational-cache/main/LICENSE)
for more details.
