use std::fmt::Debug;

fn main() {
    let v = vec![1, 2, 3, 4];
    print_slice(&v);
    print_slice(&v[..]);

    print_slice1(&v);
    print_slice1(&v[..]);
    print_slice1(v);
}

fn print_slice<T: Debug>(s: &[T]) {
    println!("{s:?}")
}

fn print_slice1<T, U>(s: T) where T: AsRef<[U]>, U: Debug {
    println!("{:?}", s.as_ref())
}