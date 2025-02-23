// Publish grandchild module by `pub mod` keyword.
pub(super) mod outer_grandchild;

pub const SOME_CONSTANT: &str = "This is outer_child::SOME_CONSTANT";

pub fn some_process() {
    println!("Called: outer_child::some_process()");

    println!("Call: outer_child > inner_grandchild::some_process()");
    inner_grandchild::some_process();

    println!("Call: outer_child > outer_grandchild::some_process()");
    outer_grandchild::some_process();
}

mod inner_grandchild {
    pub fn some_process() {
        println!("Called: inner_grandchild::some_process()");
    }
}

