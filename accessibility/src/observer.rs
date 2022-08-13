use std::{ffi::c_void, str::FromStr};

use accessibility_sys::{
    pid_t, AXObserverAddNotification, AXObserverCallback, AXObserverCallbackWithInfo,
    AXObserverCreate, AXObserverCreateWithInfoCallback, AXObserverGetRunLoopSource,
    AXObserverGetTypeID, AXObserverRef, AXObserverRemoveNotification, AXUIElementRef,
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

unsafe impl Send for AXObserver {}

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

    pub fn new_from_bundle(bundle_id: &str, callback: AXObserverCallback) -> Result<Self, Error> {
        let bundle_ui_element = AXUIElement::application_with_bundle(bundle_id)?;
        let bundle_pid = bundle_ui_element.pid()?;
        unsafe {
            Ok(TCFType::wrap_under_create_rule(
                ax_call(|x| AXObserverCreate(bundle_pid, callback, x)).map_err(Error::Ax)?,
            ))
        }
    }

    pub fn new_from_bundle_with_info(
        bundle_id: &str,
        callback: AXObserverCallbackWithInfo,
    ) -> Result<Self, Error> {
        let bundle_ui_element = AXUIElement::application_with_bundle(bundle_id)?;
        let bundle_pid = bundle_ui_element.pid()?;
        unsafe {
            Ok(TCFType::wrap_under_create_rule(
                ax_call(|x| AXObserverCreateWithInfoCallback(bundle_pid, callback, x))
                    .map_err(Error::Ax)?,
            ))
        }
    }

    pub fn add_notification<T>(
        &mut self,
        notification: &str,
        ui_element: &AXUIElement,
        mut ctx: T,
    ) -> Result<(), Error> {
        unsafe {
            // Create CFStringRef from notification string
            let notification_cfstr = CFString::from_str(notification).unwrap();

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

    pub fn remove_notification(
        &mut self,
        notification: &str,
        ui_element: &AXUIElement,
    ) -> Result<(), Error> {
        unsafe {
            // Create CFStringRef from notification string
            let notification_cfstr = CFString::from_str(notification).unwrap();

            Ok(ax_call_void(|| {
                AXObserverRemoveNotification(
                    self.0,
                    ui_element.as_CFTypeRef() as AXUIElementRef,
                    notification_cfstr.as_concrete_TypeRef(),
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

    pub fn stop(&self) {
        let runloop = CFRunLoop::get_current();
        unsafe {
            let source = TCFType::wrap_under_create_rule(AXObserverGetRunLoopSource(self.0));
            runloop.remove_source(&source, kCFRunLoopDefaultMode)
        }
    }
}

// // This is WIP - I am trying to build an abstraction similar to the AXSwift project's implementation.
// pub type Callback = fn(observer: &Observer, element: AXUIElement, notification: AXNotification);
// pub struct Observer {
//     ax_observer: AXObserver,
//     pub callback: Callback,
// }

// impl Observer {
//     pub fn new(process_id: pid_t, callback: Callback) -> Result<Self, Error> {
//         let ax_observer = AXObserver::new(process_id, internal_callback)?;

//         let callback = callback;

//         ax_observer.start();

//         Ok(Self {
//             ax_observer,
//             callback,
//         })
//     }

//     pub fn add_notification(
//         &mut self,
//         notification: AXNotification,
//         element: AXUIElement,
//     ) -> Result<(), Error> {
//         let self_ptr = std::ptr::addr_of!(self);
//         self.ax_observer
//             .add_notification(notification.to_string(), &element, &self_ptr)
//     }

//     pub fn remove_notification(
//         &mut self,
//         notification: AXNotification,
//         element: AXUIElement,
//     ) -> Result<(), Error> {
//         self.ax_observer
//             .remove_notification(notification.to_string(), &element)
//     }

//     pub fn start(&self) {
//         self.ax_observer.start();
//     }

//     pub fn stop(&self) {
//         self.ax_observer.stop();
//     }
// }

// unsafe extern "C" fn internal_callback(
//     _ax_observer_ref: AXObserverRef,
//     ax_ui_element_ref: AXUIElementRef,
//     notification_ref: CFStringRef,
//     raw_info: *mut c_void,
// ) {
//     let observer_address: *const &mut Observer = mem::transmute(raw_info);
//     let observer = &**(observer_address.as_ref()).unwrap();
//     let ax_ui_element: AXUIElement = TCFType::wrap_under_get_rule(ax_ui_element_ref);

//     let notification_string = CFString::wrap_under_get_rule(notification_ref).to_string();
//     let notification: AXNotification =
//         AXNotification::from_str(notification_string.as_str()).unwrap();

//     (observer.callback)(observer, ax_ui_element, notification);
// }
