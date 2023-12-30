use std::collections::HashMap;

fn main() {
    let map = HashMap::new();
    let mut map = explain("empty", map);
    map.insert("a".to_string(), 1);
    explain("added 1", map);
}

fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let attr: [usize; 6] = unsafe { std::mem::transmute(map) };
    println!(
        "{name} bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
        attr[2], attr[3], attr[4], attr[5]
    );
    unsafe {std::mem::transmute(attr)}
}
