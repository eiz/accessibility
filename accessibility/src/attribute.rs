use accessibility_sys::{
    kAXChildrenAttribute, kAXDescriptionAttribute, kAXLabelValueAttribute,
    kAXPlaceholderValueAttribute, kAXRoleAttribute, kAXValueAttribute, kAXWindowsAttribute,
    AXError,
};
use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    string::CFString,
};
use std::marker::PhantomData;

use crate::AXUIElement;

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
        fn $setter(&self, value: impl Into<$typ>) -> Result<(), AXError>;
    };
    (@decl $name:ident, $typ:ty, $const:ident) => {
        fn $name(&self) -> Result<$typ, AXError>;
    };
    (@impl $name:ident, $typ:ty, $const:ident, $setter:ident) => {
        accessor!(@impl $name, $typ, $const);
        fn $setter(&self, value: impl Into<$typ>) -> Result<(), AXError> {
            self.set_attribute(&AXAttribute::$name(), value)
        }
    };
    (@impl $name:ident, $typ:ty, $const:ident) => {
        fn $name(&self) -> Result<$typ, AXError> {
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
    }
}

impl AXAttribute<CFType> {
    pub fn new(name: &CFString) -> Self {
        AXAttribute(name.to_owned(), PhantomData)
    }
}

define_attributes![
    (children, CFArray<AXUIElement>, kAXChildrenAttribute),
    (description, CFString, kAXDescriptionAttribute),
    (label_value, CFString, kAXLabelValueAttribute),
    (placeholder_value, CFString, kAXPlaceholderValueAttribute),
    (role, CFString, kAXRoleAttribute),
    (value, CFType, kAXValueAttribute, set_value),
    (windows, CFArray<AXUIElement>, kAXWindowsAttribute),
];
