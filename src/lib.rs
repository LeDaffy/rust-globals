pub use globals_proc_macro::unchecked_global;
pub use globals_proc_macro::checked_global;

pub union UncheckedGlobal<T> {
    uninit: (),
    value: core::mem::ManuallyDrop<T>,
}

impl<T> UncheckedGlobal<T> {
    pub const fn uninit() -> Self {
        Self { uninit: () }
    }
    pub const fn new(value: T) -> Self {
        Self { value: core::mem::ManuallyDrop::new(value) }
    }
}

pub const fn unchecked_get_mut<T>(this: *mut UncheckedGlobal<T>) -> &'static mut T {
    unsafe { &mut *(this as *mut T) }
}



pub enum CheckedGlobal<T> {
    Uninit,
    Value(T),
}

impl<T> CheckedGlobal<T> {
    pub const fn uninit() -> Self {
        Self::Uninit
    }
    pub const fn new(value: T) -> Self {
        Self::Value(value)
    }
}

pub fn checked_get_unchecked<T>(this: *mut CheckedGlobal<T>) -> &'static mut T {
    match unsafe { &mut  *this } {
        CheckedGlobal::Uninit => unsafe { std::hint::unreachable_unchecked() },
        CheckedGlobal::Value(value) => value,
    }
}

pub fn checked_get_mut_or_init<T, F: FnOnce() -> T>(this: *mut CheckedGlobal<T>, f: F) -> &'static mut T {
    match unsafe { &mut *this } {
        CheckedGlobal::Uninit => unsafe { 
            *this  = CheckedGlobal::new(f()); 
            match &mut *this {
                CheckedGlobal::Uninit => std::hint::unreachable_unchecked(),
                CheckedGlobal::Value(value) => value,
            }
        },
        CheckedGlobal::Value(value) => value,
    }
}

pub fn checked_get_mut<T>(this: *mut CheckedGlobal<T>) -> Option<&'static mut T> {
    match unsafe { &mut *this } {
        CheckedGlobal::Uninit => None,
        CheckedGlobal::Value(value) => Some(value),
    }
}

