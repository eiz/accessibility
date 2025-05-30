use std::{ffi::c_void, marker::PhantomData};

use accessibility_sys::{
    kAXErrorFailure, kAXErrorSuccess, kAXValueTypeAXError, kAXValueTypeCFRange,
    kAXValueTypeCGPoint, kAXValueTypeCGRect, kAXValueTypeCGSize, kAXValueTypeIllegal,
    AXValueCreate, AXValueGetType, AXValueGetTypeID, AXValueGetValue, AXValueRef, AXValueType,
};
use core_foundation::{base::CFRange, declare_TCFType, impl_CFTypeDescription, impl_TCFType};
use core_graphics_types::geometry::{CGPoint, CGRect, CGSize};

use crate::{util::ax_call, Error};

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

pub(crate) fn value_type_name(kind: AXValueType) -> &'static str {
    #[allow(non_upper_case_globals)]
    match kind {
        kAXValueTypeCGPoint => "CGPoint",
        kAXValueTypeCGSize => "CGSize",
        kAXValueTypeCGRect => "CGRect",
        kAXValueTypeCFRange => "CFRange",
        kAXValueTypeAXError => "AXError",
        kAXValueTypeIllegal => "Illegal",
        _ => "<unknown>",
    }
}

declare_TCFType!(AXValue<T: AXValueKind>, AXValueRef);
impl_TCFType!(AXValue<T: AXValueKind>, AXValueRef, AXValueGetTypeID);
impl_CFTypeDescription!(AXValue<T: AXValueKind>);

impl<T: AXValueKind> AXValue<T> {
    pub fn new(val: &T) -> Result<Self, Error> {
        let ptr = unsafe { AXValueCreate(T::TYPE, val as *const T as *const c_void) };
        assert!(!ptr.is_null());
        Ok(Self(ptr, PhantomData))
    }

    pub fn value(&self) -> Result<T, Error> {
        unsafe {
            ax_call(
                |x: *mut T| match AXValueGetValue(self.0, T::TYPE, x as *mut _) {
                    true => kAXErrorSuccess,
                    false => kAXErrorFailure,
                },
            )
            .map_err(|_| Error::UnexpectedValueType {
                expected: T::TYPE,
                received: AXValueGetType(self.0),
            })
        }
    }
}
