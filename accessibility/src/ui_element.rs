use std::{
    hash::{Hash, Hasher},
    thread,
    time::{Duration, Instant},
};

use accessibility_sys::{
    kAXTrustedCheckOptionPrompt, pid_t, AXIsProcessTrusted, AXIsProcessTrustedWithOptions,
    AXUIElementCopyActionNames, AXUIElementCopyAttributeNames, AXUIElementCopyAttributeValue,
    AXUIElementCopyParameterizedAttributeNames, AXUIElementCopyParameterizedAttributeValue,
    AXUIElementCreateApplication, AXUIElementCreateSystemWide, AXUIElementGetPid,
    AXUIElementGetTypeID, AXUIElementIsAttributeSettable, AXUIElementPerformAction, AXUIElementRef,
    AXUIElementSetAttributeValue,
};
use cocoa::{
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSFastEnumeration, NSString},
};
use core_foundation::{
    array::CFArray,
    base::{TCFType, TCFTypeRef},
    boolean::CFBoolean,
    declare_TCFType,
    dictionary::CFDictionary,
    impl_CFTypeDescription, impl_TCFType,
    string::CFString,
};
use objc::{class, msg_send, rc::autoreleasepool, sel, sel_impl};

use crate::{
    util::{ax_call, ax_call_void},
    AXAttribute, Error,
};

declare_TCFType!(AXUIElement, AXUIElementRef);
impl_TCFType!(AXUIElement, AXUIElementRef, AXUIElementGetTypeID);
impl_CFTypeDescription!(AXUIElement);

unsafe impl Send for AXUIElement {}
unsafe impl Sync for AXUIElement {}

impl AXUIElement {
    pub fn system_wide() -> Self {
        unsafe { Self::wrap_under_create_rule(AXUIElementCreateSystemWide()) }
    }

    pub fn application(pid: pid_t) -> Self {
        unsafe { Self::wrap_under_create_rule(AXUIElementCreateApplication(pid)) }
    }

    pub fn application_with_bundle(bundle_id: &str) -> Result<Self, Error> {
        unsafe {
            autoreleasepool(|| {
                let bundle_id_str = NSString::alloc(nil).init_str(bundle_id).autorelease();
                let apps: id = msg_send![
                    class![NSRunningApplication],
                    runningApplicationsWithBundleIdentifier: bundle_id_str
                ];

                if let Some(app) = apps.iter().next() {
                    let pid: pid_t = msg_send![app, processIdentifier];

                    Ok(Self::wrap_under_create_rule(AXUIElementCreateApplication(
                        pid,
                    )))
                } else {
                    Err(Error::NotFound)
                }
            })
        }
    }

    pub fn application_with_bundle_timeout(
        bundle_id: &str,
        timeout: Duration,
    ) -> Result<Self, Error> {
        let deadline = Instant::now() + timeout;

        loop {
            match Self::application_with_bundle(bundle_id) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    let now = Instant::now();

                    if now >= deadline {
                        return Err(e);
                    } else {
                        let time_left = deadline.saturating_duration_since(now);
                        thread::sleep(std::cmp::min(time_left, Duration::from_millis(250)));
                    }
                }
            }
        }
    }

    pub fn pid(&self) -> Result<pid_t, Error> {
        unsafe { Ok(ax_call(|x| AXUIElementGetPid(self.0, x)).map_err(Error::Ax)?) }
    }

    pub fn attribute_names(&self) -> Result<CFArray<CFString>, Error> {
        unsafe {
            Ok(CFArray::wrap_under_create_rule(
                ax_call(|x| AXUIElementCopyAttributeNames(self.0, x)).map_err(Error::Ax)?,
            ))
        }
    }

    pub fn parameterized_attribute_names(&self) -> Result<CFArray<CFString>, Error> {
        unsafe {
            Ok(CFArray::wrap_under_create_rule(
                ax_call(|x| AXUIElementCopyParameterizedAttributeNames(self.0, x))
                    .map_err(Error::Ax)?,
            ))
        }
    }

    pub fn attribute<T: TCFType>(&self, attribute: &AXAttribute<T>) -> Result<T, Error> {
        unsafe {
            Ok(T::wrap_under_create_rule(T::Ref::from_void_ptr(
                ax_call(|x| {
                    AXUIElementCopyAttributeValue(
                        self.0,
                        attribute.as_CFString().as_concrete_TypeRef(),
                        x,
                    )
                })
                .map_err(Error::Ax)?,
            )))
        }
    }

    pub fn set_attribute<T: TCFType>(
        &self,
        attribute: &AXAttribute<T>,
        value: impl Into<T>,
    ) -> Result<(), Error> {
        let value = value.into();

        unsafe {
            Ok(ax_call_void(|| {
                AXUIElementSetAttributeValue(
                    self.0,
                    attribute.as_CFString().as_concrete_TypeRef(),
                    value.as_CFTypeRef(),
                )
            })
            .map_err(Error::Ax)?)
        }
    }

    pub fn is_attribute_settable<T: TCFType>(
        &self,
        attribute: &AXAttribute<T>,
    ) -> Result<bool, Error> {
        unsafe {
            Ok(ax_call(|x| {
                AXUIElementIsAttributeSettable(
                    self.0,
                    attribute.as_CFString().as_concrete_TypeRef(),
                    x,
                )
            })
            .map_err(Error::Ax)?)
        }
    }

    pub fn parameterized_attribute<T: TCFType, U: TCFType>(
        &self,
        attribute: &AXAttribute<T>,
        parameter: &U,
    ) -> Result<T, Error> {
        unsafe {
            Ok(T::wrap_under_create_rule(T::Ref::from_void_ptr(
                ax_call(|x| {
                    AXUIElementCopyParameterizedAttributeValue(
                        self.0,
                        attribute.as_CFString().as_concrete_TypeRef(),
                        parameter.as_CFTypeRef(),
                        x,
                    )
                })
                .map_err(Error::Ax)?,
            )))
        }
    }

    pub fn action_names(&self) -> Result<CFArray<CFString>, Error> {
        unsafe {
            Ok(CFArray::wrap_under_create_rule(
                ax_call(|x| AXUIElementCopyActionNames(self.0, x)).map_err(Error::Ax)?,
            ))
        }
    }

    pub fn perform_action(&self, name: &CFString) -> Result<(), Error> {
        unsafe {
            Ok(
                ax_call_void(|| AXUIElementPerformAction(self.0, name.as_concrete_TypeRef()))
                    .map_err(Error::Ax)?,
            )
        }
    }

    /// Checks whether or not this application is a trusted accessibility client.
    pub fn application_is_trusted() -> bool {
        unsafe {
            return AXIsProcessTrusted();
        }
    }

    /// Same as [application_is_trusted], but also shows the user a prompt asking
    /// them to allow accessibility API access if it hasn't already been given.
    pub fn application_is_trusted_with_prompt() -> bool {
        unsafe {
            let option_prompt = CFString::wrap_under_get_rule(kAXTrustedCheckOptionPrompt);
            let dict: CFDictionary<CFString, CFBoolean> =
                CFDictionary::from_CFType_pairs(&[(option_prompt, CFBoolean::true_value())]);
            return AXIsProcessTrustedWithOptions(dict.as_concrete_TypeRef());
        }
    }
}

impl Hash for AXUIElement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
