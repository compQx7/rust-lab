pub mod outer_child;

pub fn module_declaration_sample() {
    // Child module in this file. Can call with `inner_child::xxxxx()`.
    println!("Call: parent > inner_child::some_process()");
    inner_child::some_process();

    // Child module outside this file.
    println!("Call: parent > outer_child::some_process()");
    outer_child::some_process();

    // This is not possible because of `pub(super)` from grandchild module.
    // outer_child::outer_grandchild::only_parent_can_call();

    // This is possible because of `pub(crate)` from grandchild module.
    println!("Call: parent > outer_child::outer_grandchild::crate_can_call()");
    outer_child::outer_grandchild::crate_can_call();
}

mod inner_child {
    pub fn some_process() {
        println!("Called: inner_child::some_process()");
    }
}

