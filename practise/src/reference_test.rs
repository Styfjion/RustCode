use std::cell::RefCell;
use std::collections::HashMap;

struct Test {
    inner: HashMap<i32, HashMap<i32, i32>>,
    ref_cell_inner: RefCell<HashMap<i32, HashMap<i32, i32>>>
}

impl Test {
    fn get_inner_val(&mut self) -> &mut HashMap<i32, i32> {
        self.inner.get_mut(&1).unwrap()
    }

    // fn get_ref_cell_inner(&self) -> &mut HashMap<i32, i32> {
    //     self.ref_cell_inner.borrow_mut().get_mut(&1).unwrap()
    // }
}