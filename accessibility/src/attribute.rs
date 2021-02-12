use accessibility_sys::{
    kAXChildrenAttribute, kAXDescriptionAttribute, kAXLabelValueAttribute, kAXRoleAttribute,
    kAXValueAttribute, kAXWindowsAttribute,
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
pub struct AXAttribute<T>(CFString, PhantomData<T>);

impl<T: TCFType> TAXAttribute for AXAttribute<T> {
    type Value = T;
}

impl<T> AXAttribute<T> {
    #[allow(non_snake_case)]
    pub fn as_CFString(&self) -> &CFString {
        &self.0
    }
}

macro_rules! attribute {
    ($name:ident, $typ:ty, $const:ident) => {
        pub fn $name() -> AXAttribute<$typ> {
            AXAttribute(CFString::from_static_string($const), PhantomData)
        }
    };
}

impl AXAttribute<CFType> {
    pub fn new(name: &CFString) -> Self {
        AXAttribute(name.to_owned(), PhantomData)
    }
}

impl AXAttribute<()> {
    attribute!(children, CFArray<AXUIElement>, kAXChildrenAttribute);
    attribute!(description, CFString, kAXDescriptionAttribute);
    attribute!(label_value, CFString, kAXLabelValueAttribute);
    attribute!(role, CFString, kAXRoleAttribute);
    attribute!(value, CFType, kAXValueAttribute);
    attribute!(windows, CFArray<AXUIElement>, kAXWindowsAttribute);
}
