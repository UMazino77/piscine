#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

impl Role {
    pub fn power_value(&self) -> u32 {
        match self {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}

impl Member {
    pub fn new(role: Role, age: u32) -> Self {
        Self { role, age }
    }

    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Underboss => panic!("Cannot promote an Underboss!"),
        };
    }
}
