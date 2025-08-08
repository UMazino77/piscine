use std::cell::{Cell, RefCell};

#[derive(Debug , Clone)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

#[derive(Debug , Clone)]
pub struct Thread <'a> {
    pub pid : usize ,
    pub cmd : String , 
    pub parent : &'a ThreadPool,
}

impl ThreadPool {
    pub fn new() -> Self {
        Self { drops: Cell::new(0), states: RefCell::new(vec![])}
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        let pid = Self::thread_len(self);
        self.states.borrow_mut().push(false) ;
        let thr = Thread {pid : pid, cmd : c , parent : self} ;
        (pid , thr)

    }

    pub fn thread_len(&self) -> usize {
        
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        if id >= self.states.borrow().len() {
            return false;
        }
        self.states.borrow()[id]
    }

    pub fn drop_thread(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{} is already dropped", id);
        }
        self.states.borrow_mut()[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}


impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Thread { pid: p, cmd: c, parent: t }
    }

    pub fn skill(&self) {
            self.parent.drop_thread(self.pid);
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        if !self.parent.is_dropped(self.pid) {
            self.skill();
        } 
    }
}