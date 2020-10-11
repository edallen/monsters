extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn generate() -> String{

    let undead_species = vec![
        "Skeleton"
        ,"Zombie"
        ,"Ghoul"
        ,"Wight"
        ,"Wraith"
        ,"Spectre"
        ,"Mummy"
        ,"Vampire"
        ,"Revenant"
        ,"Draugr"];
    let humanoid_species = vec![
        "Kobold"
        ,"Goblin"
        ,"Orc"
        ,"Hobgoblin"
        ,"Gnolls"
        ,"Ogre"
        ,"Troll"
        ,"Hill Giant"
        ,"Stone Giant"
        ,"Fire Giant"
        ,"Cloud Giant"
    ];
    let dragon_species = vec![
        "Green Dragon"
        ,"Black Dragon"
        ,"White Dragon"
        ,"Blue Dragon"
        ,"Red Dragon"
        ,"Copper Dragon"
        ,"Brass Dragon"
        ,"Silver Dragon"
        ,"Gold Dragon"
    ];

    let mut rng = thread_rng();
    let species_list = [undead_species, humanoid_species, dragon_species];
    let species = species_list.choose( &mut rng).unwrap();
    let selection = species.choose(&mut rng);

    selection.unwrap().parse().unwrap()
}
