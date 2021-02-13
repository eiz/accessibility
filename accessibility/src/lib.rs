pub mod attribute;
pub mod ui_element;
mod util;

use core_foundation::array::CFArray;
use std::{
    cell::Cell,
    cmp, thread,
    time::{Duration, Instant},
};

pub use attribute::*;
pub use ui_element::*;

pub trait TreeVisitor {
    fn enter_element(&self, element: &AXUIElement) -> TreeWalkerFlow;
    fn exit_element(&self, element: &AXUIElement);
}

pub struct TreeWalker {
    attr_children: AXAttribute<CFArray<AXUIElement>>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TreeWalkerFlow {
    Continue,
    SkipSubtree,
    Exit,
}

impl TreeWalker {
    pub fn new() -> Self {
        Self {
            attr_children: AXAttribute::children(),
        }
    }

    pub fn walk(&self, root: &AXUIElement, visitor: &dyn TreeVisitor) {
        let _ = self.walk_one(root, visitor);
    }

    fn walk_one(&self, root: &AXUIElement, visitor: &dyn TreeVisitor) -> TreeWalkerFlow {
        let mut flow = visitor.enter_element(root);

        if flow == TreeWalkerFlow::Continue {
            if let Ok(children) = root.attribute(&self.attr_children) {
                for child in children.into_iter() {
                    let child_flow = self.walk_one(&*child, visitor);

                    if child_flow == TreeWalkerFlow::Exit {
                        flow = child_flow;
                        break;
                    }
                }
            }
        }

        visitor.exit_element(root);
        flow
    }
}

pub struct ElementFinder {
    implicit_wait: Option<Duration>,
    predicate: Box<dyn Fn(&AXUIElement) -> bool>,
    result: Cell<Option<AXUIElement>>,
}

impl ElementFinder {
    pub fn new<F>(predicate: F, implicit_wait: Option<Duration>) -> Self
    where
        F: 'static + Fn(&AXUIElement) -> bool,
    {
        Self {
            predicate: Box::new(predicate),
            implicit_wait,
            result: Cell::new(None),
        }
    }

    pub fn find(&self, root: &AXUIElement) -> Option<AXUIElement> {
        let mut deadline = Instant::now();
        let walker = TreeWalker::new();

        if let Some(implicit_wait) = &self.implicit_wait {
            deadline += *implicit_wait;
        }

        loop {
            walker.walk(root, self);

            if let Some(result) = self.result.take() {
                return Some(result);
            }

            if Instant::now() >= deadline {
                return None;
            }

            if let Some(implicit_wait) = &self.implicit_wait {
                thread::sleep(cmp::min(*implicit_wait, Duration::from_millis(250)));
            }
        }
    }
}

impl TreeVisitor for ElementFinder {
    fn enter_element(&self, element: &AXUIElement) -> TreeWalkerFlow {
        if (self.predicate)(element) {
            self.result.set(Some(element.clone()));
            return TreeWalkerFlow::Exit;
        }

        TreeWalkerFlow::Continue
    }

    fn exit_element(&self, _element: &AXUIElement) {}
}
