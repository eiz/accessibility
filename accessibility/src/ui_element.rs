use accessibility_sys::{
    pid_t, AXError, AXUIElementCopyActionNames, AXUIElementCopyAttributeNames,
    AXUIElementCopyAttributeValue, AXUIElementCreateApplication, AXUIElementCreateSystemWide,
    AXUIElementGetTypeID, AXUIElementRef,
};
use core_foundation::{
    array::CFArray,
    base::{TCFType, TCFTypeRef},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
    string::CFString,
};

use crate::{util::ax_call, AXAttribute};

declare_TCFType!(AXUIElement, AXUIElementRef);
impl_TCFType!(AXUIElement, AXUIElementRef, AXUIElementGetTypeID);
impl_CFTypeDescription!(AXUIElement);

impl AXUIElement {
    pub fn system_wide() -> Self {
        unsafe { Self::wrap_under_create_rule(AXUIElementCreateSystemWide()) }
    }

    pub fn application(pid: pid_t) -> Self {
        unsafe { Self::wrap_under_create_rule(AXUIElementCreateApplication(pid)) }
    }

    pub fn attribute_names(&self) -> Result<CFArray<CFString>, AXError> {
        unsafe {
            Ok(CFArray::wrap_under_create_rule(ax_call(|x| {
                AXUIElementCopyAttributeNames(self.0, x)
            })?))
        }
    }

    pub fn attribute<T: TCFType>(&self, attribute: &AXAttribute<T>) -> Result<T, AXError> {
        unsafe {
            Ok(T::wrap_under_create_rule(T::Ref::from_void_ptr(ax_call(
                |x| {
                    AXUIElementCopyAttributeValue(
                        self.0,
                        attribute.as_CFString().as_concrete_TypeRef(),
                        x,
                    )
                },
            )?)))
        }
    }

    pub fn action_names(&self) -> Result<CFArray<CFString>, AXError> {
        unsafe {
            Ok(CFArray::wrap_under_create_rule(ax_call(|x| {
                AXUIElementCopyActionNames(self.0, x)
            })?))
        }
    }
}
