pub fn some_process() {
    println!("Called: outer_child::outer_grandchild::some_process()");

    // Parent module items can be accessed from child modules
    println!("Call: outer_grandchild > super::SOME_CONSTANT");
    println!("{}", super::SOME_CONSTANT);
}

pub(super) fn only_parent_can_call() {
    println!("This is grandchild_method. but only parent can call this method.");
}

pub(crate) fn crate_can_call() {
    println!("This is grandchild_method. This can be call in crate.");
}

