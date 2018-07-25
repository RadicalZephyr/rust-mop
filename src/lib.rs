#![allow(dead_code, unused_variables)]

struct Slot {}

struct Method {}

struct Class {
    name: String,
    direct_superclasses: Vec<Class>,
    direct_slots: Vec<Slot>,
    class_precedence_list: Vec<Class>,
    effective_slots: Vec<Slot>,
    direct_subclasses: Vec<Class>,
    direct_methods: Vec<Method>,
}

fn find_class(name: &str) -> Option<Class> {
    None
}

fn ensure_class(name: String, superclasses: Vec<Class>, slots: Vec<Slot>) {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
