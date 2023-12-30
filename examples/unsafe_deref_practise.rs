fn main() {
    let mut age = 18;
    let r1 = &age as *const i32;
    let r2 = &mut age as *mut i32;
    unsafe {
        println!("r1: {}, r2: {}", *r1, *r2);
    }
}