#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List{head : None}
    }

    pub fn push(&mut self, value: T) {
        match &mut self.head {
            None => self.head = Some(Node {value, next: None}),
            Some(h) => {
                let mut cur = h ;
                while cur.next.is_some() {
                    cur = cur.next.as_mut().unwrap() ; 
                }
                cur.next = Some(Box::new(Node {value, next: None}));
            }
        }
    }

    pub fn pop(&mut self) {
        match &mut self.head {
            None => return ,
            Some(h) if h.next.is_none() => {
                self.head = None;
            },
            Some(h) => {
                let mut cur = h;
                while cur.next.as_ref().unwrap().next.is_some() {
                    cur = cur.next.as_mut().unwrap();
                } 
                cur.next = None ;
            },
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 1 ; 
        match &self.head {
            None => return 0,
            Some(h) => {
                let mut cur = h;
                while cur.next.is_some() {
                    cur = cur.next.as_ref().unwrap();
                    len +=1 ;
                } 
            }
        }
        len
    }
}