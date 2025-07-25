pub fn doubtful(s: &mut String) {
    *s = s.to_owned()+"?" ;
}

//clone to borrow owned data , and to_owned is to give the data an ownr / or create owned data ???!!!
// to_owned returns String  