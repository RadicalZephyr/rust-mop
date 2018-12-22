#![allow(dead_code, unused_variables)]

mod class;
use self::class::*;

mod generic;

#[macro_export]
macro_rules! defclass {
    ($name:ident ( $($direct_superclass_name:ident ),* ) {
        $( $direct_slot_name:ident );*
    }) => {
        {
            let superclass_names: Vec<&'static str> = vec![$(stringify!($direct_superclass_name)),*];
            let superclasses = canonicalize_direct_superclasses(superclass_names);
            let slot_names: Vec<&'static str> = vec![$(stringify!($direct_slot_name).to_string()),*];
            let slots = canonicalize_direct_slots(slot_names);
            ensure_class(stringify!($name), superclasses, slots);
        }
    }
}

#[macro_export]
macro_rules! defgeneric {
    ($name:ident () {}) => {()}
}

#[macro_export]
macro_rules! defmethod {
    ($name:ident () {}) => {()}
}

pub fn init() {
    class::init();
    generic::init();
}
