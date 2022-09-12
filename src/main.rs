/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

/*
 * pointer -> general concept for variable that stores a memory-address pointing to other data in memory
 * Most common pointer is a reference (&) -> don't own data
 * smart pointers -> data structures that have additional metadata and extra capabilities, own data
 *                   structs with deref(allows instances to be treated like references) and
 *                   drop(allows customizing behaviour when out of scope) trait
 * reference counting smart pointer -> allows for multiple owners by tracking them, cleans after no owners
 * String and vector -> also smart pointers
 */

use crate::List::{Cons, Nil};

fn main() {
    simple_example_of_box();
    use_list();
}

// You have to use Box<> because List would have infinite sizes otherwise.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn use_list() {
    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    println!("List is: {:#?}", list);
}

// Use cases: - size of type isn't known at compile time and you want to use the value knowing its size
//            - don't want to copy data, but transfer ownership
//            - owning a value and only caring about impl of specific T, not it being specific type
//              (Trait object)
fn simple_example_of_box() {
    // 5 is stored on the heap, b(a pointer/memory address) is stored on the stack
    let b = Box::new(5);
    println!("b = {}", b);
} // b and 5 are deallocated here
