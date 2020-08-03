mod main_hero;
mod npc;

fn main() {
    let mut main_hero = main_hero::MainHero::create_main_hero();
    println!("Hello there, {}!", main_hero.get_name());
    main_hero.gain_experience(1500);
    let Shepard = npc::NPC::new("Commander Shepard".to_string(), "Human".to_string(), false);
    main_hero.interact_with_NPC(Shepard);
}
