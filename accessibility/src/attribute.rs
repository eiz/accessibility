use accessibility_sys::{
    kAXAllowedValuesAttribute, kAXChildrenAttribute, kAXContentsAttribute, kAXDescriptionAttribute,
    kAXElementBusyAttribute, kAXEnabledAttribute, kAXFocusedAttribute, kAXFocusedWindowAttribute,
    kAXFrameAttribute, kAXFrontmostAttribute, kAXHelpAttribute, kAXIdentifierAttribute,
    kAXLabelValueAttribute, kAXMainAttribute, kAXMainWindowAttribute, kAXMaxValueAttribute,
    kAXMinValueAttribute, kAXMinimizedAttribute, kAXParentAttribute, kAXPlaceholderValueAttribute,
    kAXPositionAttribute, kAXRoleAttribute, kAXRoleDescriptionAttribute,
    kAXSelectedChildrenAttribute, kAXSizeAttribute, kAXSubroleAttribute, kAXTitleAttribute,
    kAXTitleUIElementAttribute, kAXTopLevelUIElementAttribute, kAXValueAttribute,
    kAXValueDescriptionAttribute, kAXValueIncrementAttribute, kAXVisibleChildrenAttribute,
    kAXWindowAttribute, kAXWindowsAttribute,
};
use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    boolean::CFBoolean,
    string::CFString,
};
use core_graphics_types::geometry::{CGPoint, CGRect, CGSize};
use std::marker::PhantomData;

use crate::{value::AXValue, AXUIElement, ElementFinder, Error};

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

macro_rules! constructor {
    ($name:ident, $typ:ty, $const:ident $(,$setter:ident)?) => {
        pub fn $name() -> AXAttribute<$typ> {
            AXAttribute(CFString::from_static_string($const), PhantomData)
        }
    };
}

macro_rules! accessor {
    (@decl $name:ident, AXValue<$typ:ty>, $const:ident, $setter:ident) => {
        accessor!(@decl $name, AXValue<$typ>, $const);
        fn $setter(&self, value: impl Into<$typ>) -> Result<(), Error>;
    };
    (@decl $name:ident, $typ:ty, $const:ident, $setter:ident) => {
        accessor!(@decl $name, $typ, $const);
        fn $setter(&self, value: impl Into<$typ>) -> Result<(), Error>;
    };
    (@decl $name:ident, AXValue<$typ:ty>, $const:ident) => {
        fn $name(&self) -> Result<$typ, Error>;
    };
    (@decl $name:ident, $typ:ty, $const:ident) => {
        fn $name(&self) -> Result<$typ, Error>;
    };
    (@impl $name:ident, AXValue<$typ:ty>, $const:ident, $setter:ident) => {
        accessor!(@impl $name, AXValue<$typ>, $const);
        fn $setter(&self, value: impl Into<$typ>) -> Result<(), Error> {
            self.set_attribute(&AXAttribute::$name(), AXValue::new(&value.into()).expect("wrong type"))
        }
    };
    (@impl $name:ident, $typ:ty, $const:ident, $setter:ident) => {
        accessor!(@impl $name, $typ, $const);
        fn $setter(&self, value: impl Into<$typ>) -> Result<(), Error> {
            self.set_attribute(&AXAttribute::$name(), value)
        }
    };
    (@impl $name:ident, AXValue<$typ:ty>, $const:ident) => {
        fn $name(&self) -> Result<$typ, Error> {
            self.attribute(&AXAttribute::$name()).map(|v| v.value().expect("wrong type"))
        }
    };
    (@impl $name:ident, $typ:ty, $const:ident) => {
        fn $name(&self) -> Result<$typ, Error> {
            self.attribute(&AXAttribute::$name())
        }
    };
}

macro_rules! define_attributes {
    ($(($($args:tt)*)),*,) => {
        impl AXAttribute<()> {
            $(constructor!($($args)*);)*
        }

        pub trait AXUIElementAttributes {
            $(accessor!(@decl $($args)*);)*
        }

        impl AXUIElementAttributes for AXUIElement {
            $(accessor!(@impl $($args)*);)*
        }

        impl AXUIElementAttributes for ElementFinder {
            $(accessor!(@impl $($args)*);)*
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
    (focused, CFBoolean, kAXFocusedAttribute),
    (focused_window, AXUIElement, kAXFocusedWindowAttribute),
    (frontmost, CFBoolean, kAXFrontmostAttribute, set_frontmost),
    (frame, AXValue<CGRect>, kAXFrameAttribute),
    (help, CFString, kAXHelpAttribute),
    (identifier, CFString, kAXIdentifierAttribute),
    (label_value, CFString, kAXLabelValueAttribute),
    (main, CFBoolean, kAXMainAttribute, set_main),
    (main_window, AXUIElement, kAXMainWindowAttribute),
    (max_value, CFType, kAXMaxValueAttribute),
    (min_value, CFType, kAXMinValueAttribute),
    (minimized, CFBoolean, kAXMinimizedAttribute),
    (parent, AXUIElement, kAXParentAttribute),
    (placeholder_value, CFString, kAXPlaceholderValueAttribute),
    (
        position,
        AXValue<CGPoint>,
        kAXPositionAttribute,
        set_position
    ),
    (role, CFString, kAXRoleAttribute),
    (role_description, CFString, kAXRoleDescriptionAttribute),
    (
        selected_children,
        CFArray<AXUIElement>,
        kAXSelectedChildrenAttribute
    ),
    (size, AXValue<CGSize>, kAXSizeAttribute, set_size),
    (subrole, CFString, kAXSubroleAttribute),
    (title, CFString, kAXTitleAttribute),
    (title_ui_element, AXUIElement, kAXTitleUIElementAttribute),
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
];
