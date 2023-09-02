// #![feature(once_cell)]
#[macro_use]
extern crate lazy_static;
mod test1;
mod codvar_test;
mod once_test;
mod once_call_test;
mod lazy_static_test;
mod lazy_static_tokio_test;
mod static_mutex_test;
mod serde_enum_struct_test;
mod string_test;
mod async_filed;
mod mut_test;

use std::collections::HashMap;
use test1::*;
use codvar_test::*;
use once_test::*;
use lazy_static_test::*;

fn test_loop(target: i32) {
    let mut test_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
    loop {
        match test_vec.pop() {
            Some(val) => {
                println!("val pop is {}", val);
                if val == target {
                    println!("target is found, break");
                    break;
                }
            }
            None => {
                println!("target not found");
                println!("vec has been empty, break");
                break;
            }
        }
    }
}

fn test_hash_map() {
    let mut test_map: HashMap<i32, Vec<String>> = HashMap::new();
    test_map.entry(1).or_insert(Vec::new()).push("hello".to_string());
    test_map.entry(1).or_insert(Vec::new()).push("world".to_string());
    test_map.entry(2).or_insert(Vec::new()).push("hello".to_string());
    for (key, val) in test_map {
        println!("key:{}, val:{:?}", key, val);
    }
}

fn test_array_slice() {
    let mut test_array = [1, 2, 3, 4, 5, 6];
    test_array[1] = 5;
    let slice = &mut test_array[1..3];
    slice[1] = 8;
    let slice2 = &mut test_array[2..4]; // NLL
    slice2[1] = 9;
    // println!("{:?}", slice);
    println!("{:?}", test_array);
}

fn test_mut_borrow() {
    let mut a = 1;
    let b = &mut a;
    *b = 2;
    let c = &mut a; // NLL
    *c = 3;
    // println!("{}",b);
    println!("{}", a);
}

fn exec<F: FnOnce()>(f: F)  {
    f()
}

fn exec1<F: FnMut()>(mut f: F)  {
    f()
}

fn exec2<F: Fn()>(f: F)  {
    f()
}

fn main() {
    test_loop(6);
    println!("---------------");
    test_loop(100);
    test_hash_map();
    test_array_slice();
    test_mut_borrow();
    let x = vec![1, 2, 3];
    println!("{:?}", x);

    let s = String::from("test");

    let update_string =  || println!("{}",s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
    test_1();
    test_2();
    test_3();
}