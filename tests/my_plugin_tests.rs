#![feature(plugin)]

#[plugin] extern crate my_plugin;

generate_struct!(
    name => Post,
    derive => (Show, Copy),
    fields => {
        id: i32,
        title: &'static str,
    }
);

#[test] fn struct_is_generated() {
}
