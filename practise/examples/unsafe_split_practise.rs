use std::ptr::slice_from_raw_parts_mut;

fn main() {
    let mut s = "我爱你！中国".to_string();
    let r = s.as_mut();
    if let Some((s1, s2)) = split(r, '！') {
        println!("s1: {s1}, s2:{s2}")
    }

    if let Some((s1, s2)) = split_mut(r, '！') {
        println!("mut s1: {s1}, mut s2:{s2}");
    }
}

fn split(s: &str, sep: char) -> Option<(&str, &str)> {
    let pos = s.find(sep);
    pos.map(|pos| {
        let sep_len = sep.len_utf8();
        unsafe { (s.get_unchecked(0..pos), s.get_unchecked(pos + sep_len..)) }
    })
}

fn split_mut(s: &mut str, sep: char) -> Option<(&mut str, &mut str)> {
    let pos = s.find(sep);
    pos.map(|pos| {
        let sep_len = sep.len_utf8();
        let raw_ptr = s.as_mut_ptr();
        let slice = slice_from_raw_parts_mut(raw_ptr, s.len());
        unsafe {
            (
                std::str::from_utf8_unchecked_mut(&mut (*slice)[0..pos]),
                std::str::from_utf8_unchecked_mut(&mut (*slice)[pos + sep_len..]),
            )
        }
    })
}
