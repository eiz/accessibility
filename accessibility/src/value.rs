#![allow(non_upper_case_globals)]
use std::convert::TryFrom;
use std::{ffi::c_void, mem::MaybeUninit};

use crate::Error;

use cocoa::appkit::CGPoint;
use core_foundation::base::{CFRange, TCFType};
use core_foundation::{declare_TCFType, impl_CFTypeDescription, impl_TCFType};

use accessibility_sys::{
    kAXValueTypeAXError, kAXValueTypeCFRange, kAXValueTypeCGPoint, kAXValueTypeCGRect,
    kAXValueTypeCGSize, kAXValueTypeIllegal, AXValueCreate, AXValueGetType, AXValueGetTypeID,
    AXValueGetValue, AXValueRef, AXValueType,
};
use core_graphics_types::geometry::{CGRect, CGSize};

declare_TCFType!(AXValue, AXValueRef);
impl_TCFType!(AXValue, AXValueRef, AXValueGetTypeID);
impl_CFTypeDescription!(AXValue);

// AXValues can only be of type CGPoint, CGSize, CGRect, or CFRange.
pub trait ValidAXValueType {}
impl ValidAXValueType for CGRect {}
impl ValidAXValueType for CGSize {}
impl ValidAXValueType for CGPoint {}
impl ValidAXValueType for CFRange {}

impl AXValue {
    #[allow(non_snake_case)]
    pub fn from_CGSize(mut size: CGSize) -> Result<Self, Error> {
        unsafe {
            Ok(Self::wrap_under_create_rule(AXValueCreate(
                kAXValueTypeCGSize,
                &mut size as *mut _ as *mut c_void,
            )))
        }
    }

    #[allow(non_snake_case)]
    pub fn from_CGPoint(mut point: CGPoint) -> Result<Self, Error> {
        unsafe {
            Ok(Self::wrap_under_create_rule(AXValueCreate(
                kAXValueTypeCGPoint,
                &mut point as *mut _ as *mut c_void,
            )))
        }
    }

    #[allow(non_snake_case)]
    pub fn from_CGRect(mut rect: CGRect) -> Result<Self, Error> {
        unsafe {
            Ok(Self::wrap_under_create_rule(AXValueCreate(
                kAXValueTypeCGRect,
                &mut rect as *mut _ as *mut c_void,
            )))
        }
    }

    #[allow(non_snake_case)]
    pub fn from_CFRange(mut range: CFRange) -> Result<Self, Error> {
        unsafe {
            Ok(Self::wrap_under_create_rule(AXValueCreate(
                kAXValueTypeCFRange,
                &mut range as *mut _ as *mut c_void,
            )))
        }
    }

    pub fn get_type(&self) -> AXValueType {
        unsafe { AXValueGetType(self.0) }
    }

    pub fn get_value<T: ValidAXValueType>(&self) -> Result<T, Error> {
        unsafe {
            // Verify that the value is requested of the correct type.
            match self.get_type() {
                kAXValueTypeAXError => {
                    return Err(Error::Ax(helper_u32_to_i32(kAXValueTypeAXError)));
                }
                kAXValueTypeIllegal => {
                    return Err(Error::Ax(helper_u32_to_i32(kAXValueTypeIllegal)));
                }
                kAXValueTypeCGPoint => {
                    if std::any::type_name::<T>() != "core_graphics_types::geometry::CGPoint" {
                        return Err(Error::Ax(helper_u32_to_i32(kAXValueTypeAXError)));
                    }
                }
                kAXValueTypeCGSize => {
                    if std::any::type_name::<T>() != "core_graphics_types::geometry::CGSize" {
                        return Err(Error::Ax(helper_u32_to_i32(kAXValueTypeAXError)));
                    }
                }
                kAXValueTypeCGRect => {
                    if std::any::type_name::<T>() != "core_graphics_types::geometry::CGRect" {
                        return Err(Error::Ax(helper_u32_to_i32(kAXValueTypeAXError)));
                    }
                }
                kAXValueTypeCFRange => {
                    if std::any::type_name::<T>() != "core_foundation_sys::base::CFRange" {
                        return Err(Error::Ax(helper_u32_to_i32(kAXValueTypeAXError)));
                    }
                }
                other => {
                    assert!(false, "unexpected AXValueType: {}", other);
                }
            }

            let mut value = MaybeUninit::<T>::uninit();
            let mut_ptr = &mut value as *mut _ as *mut c_void;

            if AXValueGetValue(self.0, self.get_type(), mut_ptr) {
                Ok(value.assume_init() as T)
            } else {
                Err(Error::Ax(helper_u32_to_i32(kAXValueTypeAXError)))
            }
        }
    }
}

// Just a helper to convert between i32 and u32; it is safe, because the values for u32 are known here and are small enough.
fn helper_u32_to_i32(v: u32) -> i32 {
    i32::try_from(v).ok().unwrap()
}
