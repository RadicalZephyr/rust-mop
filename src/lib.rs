#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;

lazy_static! {
    static ref CLASSES: RwLock<HashMap<String, Class>> = {
        RwLock::new(HashMap::new())
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Slot {}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Method {}

#[derive(Clone, Debug)]
struct Class {
    inner: Arc<RwLock<InnerClass>>,
}

impl PartialEq for Class {
    fn eq(&self, other: &Class) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Class {}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InnerClass {
    name: String,
    direct_superclasses: Vec<Class>,
    direct_slots: Vec<Slot>,
    class_precedence_list: Vec<Class>,
    effective_slots: Vec<Slot>,
    direct_subclasses: Vec<Class>,
    direct_methods: Vec<Method>,
}

fn find_class(name: impl AsRef<str>) -> Option<Class> {
    CLASSES.read().unwrap().get(name.as_ref()).cloned()
}

fn ensure_class(name: String, superclasses: Vec<Class>, slots: Vec<Slot>) {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(None, find_class("hello"));
    }
}
