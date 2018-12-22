use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;

use super::defclass;

lazy_static! {
    static ref CLASSES: RwLock<HashMap<String, Class>> = {
        RwLock::new(HashMap::new())
    };
}

pub fn init() {
    defclass!(standard_class () {

    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {}

#[derive(Clone, Debug)]
pub struct Class {
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

pub fn find_class(name: impl AsRef<str>) -> Option<Class> {
    CLASSES.read().unwrap().get(name.as_ref()).cloned()
}

pub fn find_class_err(name: impl AsRef<str>) -> Class {
    match find_class(&name) {
        Some(class) => class,
        None => panic!("class '{}' has not been defined", name.as_ref()),
    }
}

pub fn ensure_class(name: impl Into<String>, superclasses: Vec<Class>, slots: Vec<Slot>) {

}

pub fn canonicalize_direct_superclasses(superclasses: Vec<impl AsRef<str>>) -> Vec<Class>{
    superclasses.into_iter().map(find_class_err).collect()
}

pub fn canonicalize_direct_slots(slots: Vec<impl Into<String>>) -> Vec<Slot> {
    slots.into_iter().map(|name| Slot { name: name.into() }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(None, find_class("hello"));
    }
}
