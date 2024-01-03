use std::{ffi::c_void, marker::PhantomData};

use accessibility_sys::{
    kAXErrorFailure, kAXErrorSuccess, kAXValueTypeCFRange, kAXValueTypeCGPoint, kAXValueTypeCGRect,
    kAXValueTypeCGSize, AXValueCreate, AXValueGetTypeID, AXValueGetValue, AXValueRef, AXValueType,
};
use core_foundation::{base::CFRange, declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use core_graphics_types::geometry::{CGPoint, CGRect, CGSize};

use crate::util::ax_call;

pub trait AXValueKind {
    const TYPE: AXValueType;
}

impl AXValueKind for CGPoint {
    const TYPE: AXValueType = kAXValueTypeCGPoint;
}
impl AXValueKind for CGSize {
    const TYPE: AXValueType = kAXValueTypeCGSize;
}
impl AXValueKind for CGRect {
    const TYPE: AXValueType = kAXValueTypeCGRect;
}
impl AXValueKind for CFRange {
    const TYPE: AXValueType = kAXValueTypeCFRange;
}

declare_TCFType!(AXValue<T: AXValueKind>, AXValueRef);
impl_TCFType!(AXValue<T: AXValueKind>, AXValueRef, AXValueGetTypeID);
impl_CFTypeDescription!(AXValue<T: AXValueKind>);

#[derive(Copy, Clone, Debug)]
pub struct WrongType;

impl<T: AXValueKind> AXValue<T> {
    pub fn new(val: &T) -> Result<Self, WrongType> {
        let ptr = unsafe { AXValueCreate(T::TYPE, val as *const T as *const c_void) };
        if ptr.is_null() {
            Err(WrongType)
        } else {
            Ok(Self(ptr, PhantomData))
        }
    }

    pub fn value(&self) -> Result<T, WrongType> {
        unsafe {
            ax_call(
                |x: *mut T| match AXValueGetValue(self.0, T::TYPE, x as *mut _) {
                    true => kAXErrorSuccess,
                    false => kAXErrorFailure,
                },
            )
            .map_err(|_| WrongType)
        }
    }
}
