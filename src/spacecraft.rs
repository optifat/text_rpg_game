struct Spacecraft {
    name: String,
    hp: u16,
    shields: u16,
    firepower: u16,
    speed: u16,
    max_fuel: u16,
    current_fuel: u16
}

impl Spacecraft{
    pub fn new(name: String, hp: u16, shields: u16, firepower: u16, speed: u16, max_fuel: u16) -> Spacecraft{
        Spacecraft{name, hp, shields, firepower, speed, max_fuel, current_fuel:(max_fuel)}
    }

    pub fn take_damage(&mut self, damage: u16){
        let overdamage = (damage - self.shields)*((damage > self.shields) as u16);
        self.shields -= damage - overdamage;
        self.hp -= overdamage;
    }
}
