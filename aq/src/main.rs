use accessibility::AXUIElement;
use core_foundation::base::TCFType;

fn main() {
    let system_element = AXUIElement::system_wide();

    println!["it loads: {:?}", system_element.as_CFTypeRef()];
}
