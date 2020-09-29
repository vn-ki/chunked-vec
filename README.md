# ChunkedVec

A space efficient alternative to `Vec<Option<T>`.


```rust
let mut v = ChunkedVecInt::new();

v.append(Some(1));
v.append(None);
v.append(Some(2));

assert_eq!(v.get(0), Some(&1));
assert_eq!(v.get(1), None);
assert_eq!(v.get(2), Some(&2));
```

### Tests

Basic tests for `BitVec` and `ChunkedVecInt` are provided in their respective files.

```bash
cargo test
```