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

pub fn insert_class(class: Class) {
    let name = class.inner.read().unwrap().name.clone();
    CLASSES.write().unwrap().insert(name, class);
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

pub fn ensure_class(name: impl AsRef<str>, direct_superclasses: Vec<Class>, direct_slots: Vec<Slot>) {
    if let None = find_class(&name) {
        let name = name.as_ref().to_string();
        // TODO: replace this:
        let class_precedence_list = vec![];
        let effective_slots = vec![];
        let direct_subclasses = vec![];
        let direct_methods = vec![];
        let class = InnerClass { name, class_precedence_list,
                                 direct_superclasses, direct_slots,
                                 effective_slots,
                                 direct_subclasses, direct_methods };
        let inner = Arc::new(RwLock::new(class));
        let class_obj = Class { inner };
        // with this. eventually.
        // let standard_class = find_class("StandardClass").unwrap();
        // let class_obj = make_instance(standard_class);
        insert_class(class_obj);
    } else {
        panic!("Can't redefine the class named {}.", name.as_ref());
    }
}

pub fn canonicalize_direct_superclasses(superclasses: Vec<impl AsRef<str>>) -> Vec<Class>{
    superclasses.into_iter().map(find_class_err).collect()
}

pub fn canonicalize_direct_slots(slots: Vec<impl Into<String>>) -> Vec<Slot> {
    slots.into_iter().map(|name| Slot { name: name.into() }).collect()
}

pub fn make_instance(class: Class) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(None, find_class("hello"));
    }
}
