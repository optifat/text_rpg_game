use crate::npc;

struct Location {
    name: String,
    locType: String, // planet, station, smth else
    characters: Vec<crate::npc::NPC>
}

impl Location {
    pub fn new(name: String, locType: String, characters: Vec<crate::npc::NPC>) -> Location{
        Location{name, locType, characters}
    }
}
