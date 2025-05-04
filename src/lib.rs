pub use globals_proc_macro::unchecked_global;

pub union UninitializedGlobal<T> {
    uninit: (),
    value: core::mem::ManuallyDrop<T>,
}

impl<T> UninitializedGlobal<T> {
    pub const fn uninit() -> Self {
        Self { uninit: () }
    }
    pub const fn new(value: T) -> Self {
        Self { value: core::mem::ManuallyDrop::new(value) }
    }
}

pub const fn uninit_as_ref_mut<T>(uninit: *mut UninitializedGlobal<T>) -> &'static mut T {
    unsafe { &mut *(uninit as *mut T) }
}
