use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Node {ref_list : ref_list}
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element.clone())
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        let mut i = 0;
        for _ in 0..self.ref_list.len() {
            if  Rc::ptr_eq(&self.ref_list[i], &element) {
                self.ref_list.remove(i);
            } else {
                i+=1
            }
        }
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}
