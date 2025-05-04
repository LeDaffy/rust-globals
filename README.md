# rust-globals

A crate for working with global variables in rust. Provides an attribute macro `#[unsafe_global]` applied to global variable declartions:


```rust
#[unsafe_global]
static mut MY_GLOBAL: u32 = 0;

/// Provided by macro
pub fn my_global() -> &mut u32;
```

If the variable has no initializer, an init function is generated:

```rust
#[unsafe_global]
static mut MY_GLOBAL: u32 = 0;

/// Provided by macro
pub fn my_global() -> &mut u32;
/// Must be called prior to accessing via my_global
pub fn my_global_init(value: u32);
```
