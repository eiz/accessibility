use std::{ffi::c_void, str::FromStr};

use accessibility_sys::{
    pid_t, AXObserverAddNotification, AXObserverCallback, AXObserverCallbackWithInfo,
    AXObserverCreate, AXObserverCreateWithInfoCallback, AXObserverGetRunLoopSource,
    AXObserverGetTypeID, AXObserverRef, AXUIElementRef,
};

use crate::{
    util::{ax_call, ax_call_void},
    Error,
};

use core_foundation::{
    base::TCFType,
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
    runloop::{kCFRunLoopDefaultMode, CFRunLoop},
    string::CFString,
};

use super::AXUIElement;

declare_TCFType!(AXObserver, AXObserverRef);
impl_TCFType!(AXObserver, AXObserverRef, AXObserverGetTypeID);
impl_CFTypeDescription!(AXObserver);

impl AXObserver {
    pub fn new(pid: pid_t, callback: AXObserverCallback) -> Result<Self, Error> {
        unsafe {
            Ok(TCFType::wrap_under_create_rule(
                ax_call(|x| AXObserverCreate(pid, callback, x)).map_err(Error::Ax)?,
            ))
        }
    }

    pub fn new_with_info(pid: pid_t, callback: AXObserverCallbackWithInfo) -> Result<Self, Error> {
        unsafe {
            Ok(TCFType::wrap_under_create_rule(
                ax_call(|x| AXObserverCreateWithInfoCallback(pid, callback, x))
                    .map_err(Error::Ax)?,
            ))
        }
    }

    pub fn add_notification<T>(
        &mut self,
        notification: String,
        ui_element: &AXUIElement,
        mut ctx: T,
    ) -> Result<(), Error> {
        unsafe {
            // Create CFStringRef from notification string
            let notification_cfstr = CFString::from_str(notification.as_str()).unwrap();

            Ok(ax_call_void(|| {
                AXObserverAddNotification(
                    self.0,
                    ui_element.as_CFTypeRef() as AXUIElementRef,
                    notification_cfstr.as_concrete_TypeRef(),
                    &mut ctx as *mut _ as *mut c_void,
                )
            })
            .map_err(Error::Ax)?)
        }
    }

    pub fn start(&self) {
        let runloop = CFRunLoop::get_current();
        unsafe {
            let source = TCFType::wrap_under_create_rule(AXObserverGetRunLoopSource(self.0));
            runloop.add_source(&source, kCFRunLoopDefaultMode)
        }
    }
}
