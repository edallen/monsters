extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn generate() -> String{
   // String::from("Generated Description")

    let descriptions = vec![
        "tattered"
        ,"ugly"
        ,"beautiful"
        ,"dark"
        ,"cancerous"
        ,"grim"
        ,"playful"
        ,"horny"
        ,"curious"
        ,"angry"];
    let mut rng = thread_rng();
    let selection = descriptions.choose(&mut rng);

    selection.unwrap().parse().unwrap()
}
