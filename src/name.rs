extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn generate() -> String {

    let names = vec![
        "Bob"
        ,"Joe"
        ,"Vermithrax"
        ,"Jigglypop"
        ,"Assurkanipal"
        ,"Urman"
        ,"Roscoe"
        ,"Vindaloo"
        ,"Ziggy"
        ,"Walter"
    ];
    let mut rng = thread_rng();
    let selection = names.choose(&mut rng);

    selection.unwrap().parse().unwrap()
}
