use std::rc::Rc;

pub struct Tracker {

    pub messages: std::cell::RefCell<Vec<String>>,
    pub value: usize,
    pub max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Self {messages: std::cell::RefCell::new(Vec::new()),value: 0,max}
    }

    pub fn peek(&self, rc: &Rc<i32>) {
        let count = Rc::strong_count(rc);
        let mut track_msgs = self.messages.borrow_mut();

        // println!("{} ====> {}", count , self.max);
        track_msgs.push(format!("Info: This value would use {}% of your quota",((count as f64 /self.max as f64)* 100.) as i32 ));
    }

    pub fn set_value(&self, rc: &Rc<i32>) {
        // that was bad by me
        let count = Rc::strong_count(rc);
        let mut track_msgs = self.messages.borrow_mut();
        if count > self.max {
            track_msgs.push(format!("Error: You can't go over your quota!" ));
        } else if count >= (7*self.max) /10{
            track_msgs.push(format!("Warning: You have used up over {}% of your quota!",((count as f64 / self.max as f64)*100.) as i32,));
        }
    }
}