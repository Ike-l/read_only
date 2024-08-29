# small_read_only
This crate adds ```#[derive(ReadOnly)]```.<br>
It implements getters for all fields without the ```#[NoRead]``` attribute.<br>

It works for:
* Structs

## Example 

```rust
use small_read_only::ReadOnly;

#[derive(ReadOnly)]
pub struct A<'a> {
    b: usize,
    c: String,
    d: &'a str,
}

impl<'a> A<'a> {
    pub fn new(b: usize, c: String, d: &'a str) -> Self {
        Self {
            b, c, d
        }
    }
}

let a = A::new(1, "c".to_string(), "d");

assert_eq!(a.b(), &1);
assert_eq!(a.c(), "c");
assert_eq!(a.d(), &"d");
```

## License
MIT or Apache-2.0
