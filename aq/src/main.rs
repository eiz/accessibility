use accessibility::{AXAttribute, AXUIElement};
use core_foundation::{array::CFArray, string::CFString};

trait TreeVisitor {
    fn enter_element(&mut self, element: &AXUIElement) -> bool;
    fn exit_element(&mut self, element: &AXUIElement);
}

struct TreeWalker {
    attr_children: AXAttribute<CFArray<AXUIElement>>,
}

impl TreeWalker {
    pub fn new() -> Self {
        Self {
            attr_children: AXAttribute::children(),
        }
    }

    pub fn walk(&self, root: &AXUIElement, visitor: &mut dyn TreeVisitor) {
        if visitor.enter_element(root) {
            if let Ok(children) = root.attribute(&self.attr_children) {
                for child in children.into_iter() {
                    self.walk(&*child, visitor);
                }
            }
        }
        visitor.exit_element(root);
    }
}

struct PrintyBoi {
    level: usize,
    indent: String,
    children: AXAttribute<CFArray<AXUIElement>>,
    role: AXAttribute<CFString>,
}

impl PrintyBoi {
    pub fn new_with_indentation(indent: usize) -> Self {
        Self {
            level: 0,
            indent: " ".repeat(indent),
            children: AXAttribute::children(),
            role: AXAttribute::role(),
        }
    }
}

impl TreeVisitor for PrintyBoi {
    fn enter_element(&mut self, element: &AXUIElement) -> bool {
        let indent = self.indent.repeat(self.level);
        println![
            "{}- {}",
            indent,
            element
                .attribute(&self.role)
                .unwrap_or_else(|_| CFString::new("(error)")),
        ];

        if let Ok(names) = element.attribute_names() {
            for name in names.into_iter() {
                if &*name == self.children.as_CFString() {
                    continue;
                }

                if let Ok(value) = element.attribute(&AXAttribute::new(&*name)) {
                    println!["{}|. {}: {:?}", indent, *name, value];
                }
            }
        }
        self.level += 1;
        true
    }

    fn exit_element(&mut self, _element: &AXUIElement) {
        self.level -= 1;
    }
}

fn main() {
    let app = AXUIElement::application(80898);
    let windows = app.attribute(&AXAttribute::windows()).unwrap();
    let walker = TreeWalker::new();
    let mut printy = PrintyBoi::new_with_indentation(4);

    for window in &windows {
        walker.walk(&*window, &mut printy);
    }
}
