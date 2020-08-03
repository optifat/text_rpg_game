pub struct NPC{
    name: String,
    race: String,
    has_quest: bool
}

impl NPC {
    pub fn new(name: String, race: String, has_quest: bool) -> NPC{
        NPC{name, race, has_quest}
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }
}
