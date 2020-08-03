use std::io;

pub struct MainHero {
    name: String,
    intelligence: u8,
    charisma: u8
}

impl MainHero{

    //pub fn create_main_hero(name: String, intelligence: u8, charisma: u8)-> MainHero{
    //    MainHero{name, intelligence, charisma}
    //}

    pub fn create_main_hero()-> MainHero{
        let skill_points = 10;
        loop {
            let mut name = String::new();
            let mut intelligence: u8;
            let mut charisma: u8;
            let mut skill_points_left = skill_points;

            println!("Input your name");
            io::stdin().read_line(&mut name).expect("Failed to read line");
            name.pop();

            println!("Input your intelligence up to 7 ({} points left)", skill_points_left);
            let mut intelligence = String::new();
            io::stdin().read_line(&mut intelligence).expect("Failed to read line");
            let intelligence: u8 = intelligence.trim().parse().expect("Please type a number!");

            if intelligence > 7{
                println!("Intelligence max value is 7 ({} is input)", intelligence);
                continue;
            } else if intelligence > skill_points_left{
                println!("You don't have enough skill points!");
                continue;
            }
            skill_points_left -= intelligence;

            println!("Input your charisma up to 7 ({} points left)", skill_points_left);
            let mut charisma = String::new();
            io::stdin().read_line(&mut charisma).expect("Failed to read line");
            let charisma: u8 = charisma.trim().parse().expect("Please type a number!");

            if charisma > 7{
                println!("Charisma max value is 7 ({} is input)", charisma);
                continue;
            } else if charisma > skill_points_left{
                println!("You don't have enough skill points!");
                continue;
            }
            skill_points_left -= charisma;

            if skill_points_left == 0{
                return MainHero{name, intelligence, charisma}
            } else{
                println!("You have {} skill points left", skill_points_left);
            }
        }
    }

    pub fn get_name(self) -> String{
        self.name
    }
}
