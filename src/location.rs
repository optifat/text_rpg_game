use crate::npc;

struct Location {
    name: String,
    loc_type: String, // planet, station, smth else
    characters: Vec<crate::npc::NPC>
}

impl Location {
    pub fn new(name: String, loc_type: String, characters: Vec<crate::npc::NPC>) -> Location{
        Location{name, loc_type, characters}
    }
}
