use std::{ffi::c_void, str::FromStr};

use accessibility_sys::{
    pid_t, AXObserverAddNotification, AXObserverCallback, AXObserverCreate, AXObserverGetTypeID,
    AXObserverRef, AXUIElementRef,
};

use core_foundation::{
    base::TCFType,
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};

use super::AXUIElement;

declare_TCFType!(AXObserver, AXObserverRef);
impl_TCFType!(AXObserver, AXObserverRef, AXObserverGetTypeID);
impl_CFTypeDescription!(AXObserver);

impl AXObserver {
    pub fn new(pid: pid_t, callback: &AXObserverCallback) -> Self {
        unsafe {
            let observer_ref: *mut AXObserverRef = std::ptr::null_mut();
            let _ = AXObserverCreate(pid, *callback, observer_ref);
            Self::wrap_under_create_rule(*observer_ref)
        }
    }

    pub fn add_notification(&self, notification: String, ui_element: &AXUIElement) {
        unsafe {
            // Create CFStringRef from notification
            let notification_cfstr = CFString::from_str(notification.as_str()).unwrap();
            let my_ptr: *mut c_void = self as *const _ as *mut _;

            AXObserverAddNotification(
                self.0,
                ui_element.as_CFTypeRef() as AXUIElementRef,
                notification_cfstr.as_CFTypeRef() as CFStringRef,
                my_ptr,
            );
        }
    }
}
