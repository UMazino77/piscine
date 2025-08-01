use::chrono::Utc ;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FormError<'a> {
    pub form_values : (&'a str, String) ,
    pub date : String ,
    pub err : &'a str,
}

impl FormError <'_> {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Utc::now();

        FormError{form_values :(field_name, field_value), date : now.format("%Y-%m-%d %H:%M:%S").to_string(), err : err}
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name : String ,
    pub password : String ,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        let mut a = false;
        let mut b = false ;
        let mut c = false ;
        if self.name.is_empty() {
            let a = FormError::new("name", "".to_owned() , "Username is empty");
            return Err(a); 
        } else if self.password.len() < 8 {
            let a = FormError::new("name", self.password.clone()  , "Password should be at least 8 characters long");
            return Err(a);
        } 
        let _ = self.password.chars().for_each(|x| {
            if x.is_ascii_alphabetic() {
                a = true
            }else if x.is_ascii_digit() {
                b = true 
            } else {
                c = true
            }
        });
        if !a || !b || !c {
            let a = FormError::new("name", self.password.clone() , "Password should be a combination of ASCII numbers, letters and symbols");
            return Err(a);
        }
        Ok(())
    }
}