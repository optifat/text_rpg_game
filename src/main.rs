mod main_hero;

fn main() {
    let main_hero = main_hero::MainHero::create_main_hero();
    println!("Hello there, {}!", main_hero.get_name());
}
