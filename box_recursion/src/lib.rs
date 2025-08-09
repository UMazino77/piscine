#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(a: &str) -> Role {
        if a.to_ascii_lowercase().contains("worker") {
            return Role::Worker
        } else if a.to_ascii_lowercase().contains("ceo") {
            return Role::CEO
        } else if a.to_ascii_lowercase().contains("manager") {
            return Role::Manager
        }
        panic!("no such role {}", a)
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}


pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment{grade : None}
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let mut a = Worker{role: Role::from(role),name : name.to_owned(), next : None } ;
        match &self.grade {
            Some(b) => {a.next = Some(Box::new(*b.clone())) ; self.grade = Some(Box::new(a))}, 
            None => self.grade = Some(Box::new(a)),
        }

    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let name ;
        match &self.grade {
            None => return Some(String::new()) ,
            Some(work) => {
                name  = self.grade.clone().unwrap().name ;
                match work.next {   
                    None => self.grade = None,
                    Some(_) => self.grade = work.next.clone(),
                }
            },
        }
        Some(name)
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        return match &self.grade {
            None => None ,
            Some(work)=>Some((work.name.clone(), work.role.clone()))
        }
    }
}