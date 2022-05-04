use accessibility_sys::{
    kAXAllowedValuesAttribute, kAXBoundsForRangeParameterizedAttribute, kAXChildrenAttribute,
    kAXContentsAttribute, kAXDescriptionAttribute, kAXElementBusyAttribute, kAXEnabledAttribute,
    kAXFocusedApplicationAttribute, kAXFocusedAttribute, kAXFocusedUIElementAttribute,
    kAXHelpAttribute, kAXIdentifierAttribute, kAXLabelValueAttribute, kAXMainAttribute,
    kAXMaxValueAttribute, kAXMinValueAttribute, kAXMinimizedAttribute, kAXParentAttribute,
    kAXPlaceholderValueAttribute, kAXPositionAttribute, kAXRoleAttribute,
    kAXRoleDescriptionAttribute, kAXSelectedChildrenAttribute, kAXSelectedTextAttribute,
    kAXSelectedTextRangeAttribute, kAXSizeAttribute, kAXSubroleAttribute, kAXTitleAttribute,
    kAXTopLevelUIElementAttribute, kAXValueAttribute, kAXValueDescriptionAttribute,
    kAXValueIncrementAttribute, kAXVisibleChildrenAttribute, kAXWindowAttribute,
    kAXWindowsAttribute,
};
use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    boolean::CFBoolean,
    string::CFString,
};
use std::marker::PhantomData;

use crate::{AXUIElement, AXValue, ElementFinder, Error};

pub trait TAXAttribute {
    type Value: TCFType;
}

#[derive(Clone, Debug)]
pub struct AXAttribute<T>(CFString, PhantomData<*const T>);

impl<T: TCFType> TAXAttribute for AXAttribute<T> {
    type Value = T;
}

impl<T> AXAttribute<T> {
    #[allow(non_snake_case)]
    pub fn as_CFString(&self) -> &CFString {
        &self.0
    }
}

macro_rules! accessor {
    (@decl $name:ident, $typ:ty, $const:ident, $setter:ident) => {
        accessor!(@decl $name, $typ, $const);
        fn $setter(&self, value: impl Into<$typ>) -> Result<(), Error>;
    };
    (@decl $name:ident, $typ:ty, $const:ident) => {
        fn $name(&self) -> Result<$typ, Error>;
    };
    (@impl $name:ident, $typ:ty, $const:ident, $setter:ident) => {
        accessor!(@impl $name, $typ, $const);
        fn $setter(&self, value: impl Into<$typ>) -> Result<(), Error> {
            self.set_attribute(&AXAttribute::$name(), value)
        }
    };
    (@impl $name:ident, $typ:ty, $const:ident) => {
        fn $name(&self) -> Result<$typ, Error> {
            self.attribute(&AXAttribute::$name())
        }
    };
}

macro_rules! define_attributes {
    ($(($name:ident, $typ:ty, $const:ident $(,$setter:ident)?)),*,) => {
        impl AXAttribute<()> {
            $(
                pub fn $name() -> AXAttribute<$typ> {
                    AXAttribute(CFString::from_static_string($const), PhantomData)
                }
            )*
        }

        pub trait AXUIElementAttributes {
            $(accessor!(@decl $name, $typ, $const $(, $setter)? );)*
        }

        impl AXUIElementAttributes for AXUIElement {
            $(accessor!(@impl $name, $typ, $const $(, $setter)? );)*
        }

        impl AXUIElementAttributes for ElementFinder {
            $(accessor!(@impl $name, $typ, $const $(, $setter)? );)*
        }
    }
}

impl AXAttribute<CFType> {
    pub fn new(name: &CFString) -> Self {
        AXAttribute(name.to_owned(), PhantomData)
    }
}

define_attributes![
    (allowed_values, CFArray<CFType>, kAXAllowedValuesAttribute),
    (children, CFArray<AXUIElement>, kAXChildrenAttribute),
    (contents, AXUIElement, kAXContentsAttribute),
    (description, CFString, kAXDescriptionAttribute),
    (element_busy, CFBoolean, kAXElementBusyAttribute),
    (enabled, CFBoolean, kAXEnabledAttribute),
    (
        focused_application,
        AXUIElement,
        kAXFocusedApplicationAttribute
    ),
    (focused_uielement, AXUIElement, kAXFocusedUIElementAttribute),
    (focused, CFBoolean, kAXFocusedAttribute),
    (frame, AXValue, kAXFocusedAttribute),
    (help, CFString, kAXHelpAttribute),
    (identifier, CFString, kAXIdentifierAttribute),
    (label_value, CFString, kAXLabelValueAttribute),
    (main, CFBoolean, kAXMainAttribute, set_main),
    (max_value, CFType, kAXMaxValueAttribute),
    (min_value, CFType, kAXMinValueAttribute),
    (minimized, CFBoolean, kAXMinimizedAttribute),
    (parent, AXUIElement, kAXParentAttribute),
    (placeholder_value, CFString, kAXPlaceholderValueAttribute),
    (position, AXValue, kAXPositionAttribute),
    (role, CFString, kAXRoleAttribute),
    (role_description, CFString, kAXRoleDescriptionAttribute),
    (
        selected_children,
        CFArray<AXUIElement>,
        kAXSelectedChildrenAttribute
    ),
    (subrole, CFString, kAXSubroleAttribute),
    (size, AXValue, kAXSizeAttribute),
    (selected_text, CFString, kAXSelectedTextAttribute),
    (selected_text_range, AXValue, kAXSelectedTextRangeAttribute),
    (title, CFString, kAXTitleAttribute),
    (
        top_level_ui_element,
        AXUIElement,
        kAXTopLevelUIElementAttribute
    ),
    (value, CFType, kAXValueAttribute, set_value),
    (value_description, CFString, kAXValueDescriptionAttribute),
    (value_increment, CFType, kAXValueIncrementAttribute),
    (
        visible_children,
        CFArray<AXUIElement>,
        kAXVisibleChildrenAttribute
    ),
    (window, AXUIElement, kAXWindowAttribute),
    (windows, CFArray<AXUIElement>, kAXWindowsAttribute),
    // parameterized attributes
    (
        bounds_for_range,
        AXValue,
        kAXBoundsForRangeParameterizedAttribute
    ),
];
