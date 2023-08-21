# generational-cache

A library crate providing generational-arena backed cache implementations in 100% safe, `[no_std]` compatible rust.

```rust
use generational_cache::prelude::*;

const CAPACITY: usize = 3;

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
  Err(LRUCacheError::CacheMiss) => {},
  _ => unreachable!("Wrong error on cache miss"),
};

match cache.query(&-42) {
  Err(LRUCacheError::CacheMiss) => {},
  _ => unreachable!("Wrong error on cache miss"),
};

assert_eq!(cache.query(&-3).unwrap(), &3);

assert_eq!(cache.least_recent().unwrap(), (&-4, &4));
assert_eq!(cache.most_recent().unwrap(), (&-3, &3));

assert_eq!(cache.remove(&-2).unwrap(), 42);

match cache.query(&-2) {
  Err(LRUCacheError::CacheMiss) => {},
  _ => unreachable!("Wrong error on cache miss"),
};

// zero capacity LRUCache is unusable
let mut cache = LRUCache::<_, i32, u64, AllocBTreeMap<_, _>>::with_backing_vector(Array::<_, 0_usize>::new());

match cache.insert(0, 0) {
    Err(LRUCacheError::ListUnderflow) => {}
    _ => unreachable!("Wrong error on list underflow."),
};

```

## Usage

`generational-cache` is a library crate. You may include it in your `Cargo.toml` as follows:

```toml
[dependencies]
generational-cache = "https://github.com/arindas/generational-cache"
```

## License
This repository is licensed under the MIT License. See [LICENSE](./LICENSE) for more details.
