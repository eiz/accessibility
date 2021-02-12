use accessibility::{AXAttribute, AXUIElement};
use core_foundation::base::TCFType;

fn main() {
    let system_element = AXUIElement::system_wide();
    let names = system_element
        .attribute_names()
        .expect("failed to get attribute names");
    let actions = system_element
        .action_names()
        .expect("failed to get action names");

    println!["{:?} {:?}", names, actions];
    println!["it loads: {:?}", system_element.as_CFTypeRef()];

    let app = AXUIElement::application(1250);
    let windows = app.attribute(&AXAttribute::windows()).unwrap();
    let attr_children = AXAttribute::children();
    let attr_role = AXAttribute::role();

    for window in &windows {
        println!["{:?}", *window];

        for child in window.attribute(&attr_children).unwrap().into_iter() {
            println!["  {:?}", child.attribute(&attr_role)];
        }
    }
}
