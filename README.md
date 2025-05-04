# rust-globals

A crate for working with global variables in rust. Provides an attribute macro `#[unchecked_global]` and `#[checked_global]` which can be applied to global variable declartions. Checked globals are checked at run time to ensure the value was initialized. Unchecked globals have no checking for initiailation.


### Examples


```rust
#[unchecked_global]
static mut MY_GLOBAL: u32 = 0;

/// Provided by macro
pub fn my_global() -> &mut u32;
```

If the variable has no initializer, an init function is generated:

```rust
#[unchecked_global]
static mut MY_GLOBAL: u32;

/// Provided by macro
pub fn my_global() -> &mut u32;
/// Must be called prior to accessing via my_global
pub fn my_global_init(value: u32);
```
