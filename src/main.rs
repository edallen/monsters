use monsters::monster_species;
use monsters::name;
use monsters::description;
use monsters::record;

fn main() {
    let mut monster = record::blank_monster();
    monster.name = name::generate();
    monster.species = monster_species::generate();
    monster.description = description::generate();
    println!("{} the {} is {}.",monster.name, monster.species, monster.description);
}

