use std::mem::MaybeUninit;

use accessibility_sys::{kAXErrorSuccess, AXError};

pub(crate) unsafe fn ax_call<F, V>(f: F) -> Result<V, AXError>
where
    F: Fn(*mut V) -> AXError,
{
    let mut result = MaybeUninit::uninit();
    let err = (f)(result.as_mut_ptr());

    if err != kAXErrorSuccess {
        return Err(err);
    }

    Ok(result.assume_init())
}
