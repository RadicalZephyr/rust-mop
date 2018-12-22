#![allow(dead_code, unused_variables)]

mod class;
use self::class::*;

mod generic;

#[macro_export]
macro_rules! defclass {
    ($name:ident ( $($direct_superclass_name:ident ),* ) {
        $( $direct_slot_name:ident );*
    }) => {
        ensure_class(stringify!($name).to_string(),
                     canonicalize_direct_superclasses(vec![$(stringify!($direct_superclass_name).to_string()),*]),
                     canonicalize_direct_slots(vec![$(stringify!($direct_slot_name).to_string()),*])
        );
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
