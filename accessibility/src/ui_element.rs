use accessibility_sys::{AXUIElementCreateSystemWide, AXUIElementGetTypeID, AXUIElementRef};
use core_foundation::{base::TCFType, declare_TCFType, impl_TCFType};

declare_TCFType!(AXUIElement, AXUIElementRef);
impl_TCFType!(AXUIElement, AXUIElementRef, AXUIElementGetTypeID);

impl AXUIElement {
    pub fn system_wide() -> AXUIElement {
        unsafe { Self::wrap_under_create_rule(AXUIElementCreateSystemWide()) }
    }
}
