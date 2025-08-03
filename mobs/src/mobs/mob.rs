use std::collections::{HashMap, HashSet};

pub mod boss;
pub mod member;

use boss::Boss;
use member::{Member, Role};

#[derive(Debug, PartialEq, Clone)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn new(name: String, boss: Boss) -> Self {
        Self {
            name,
            boss,
            members: HashMap::new(),
            cities: HashSet::new(),
            wealth: 0,
        }
    }

    pub fn recruit(&mut self, member_info: (&str, u32)) {
        let (name, age) = member_info;
        let member = Member::new(Role::Associate, age);
        self.members.insert(name.to_string(), member);
    }

}