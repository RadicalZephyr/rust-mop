#![allow(dead_code, unused_variables)]

mod class;

mod generic;

#[macro_export]
macro_rules! defclass {
    ($name:ident {}) => {()}
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
