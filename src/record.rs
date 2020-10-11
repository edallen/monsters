pub struct Monster {
    pub name: String
    ,pub species: String
    ,pub description: String
}

pub fn blank_monster() -> Monster {
    Monster {
        name: String::from("no name")
        ,species: String::from("no species" )
        ,description: String::from("no description" )
    }
}
