use rand::seq::SliceRandom;
use std::{io, prelude::v1::*, vec};

struct NameList<T> {
    first: Vec<T>,
    last: Vec<T>,
}
fn main() {
    let name_list = NameList {
        first: vec![
            "Baby Oil",
            "Bad News",
            "Big Burps",
            "Bill 'Beenie-Weenie'",
            "Bob 'Stinkbug'",
            "Bowel Noises",
            "Boxelder",
            "Bud 'Lite'",
            "Butterbean",
            "Buttermilk",
            "Buttocks",
            "Chad",
            "Chesterfield",
            "Chewy",
            "Chigger",
            "Cinnabuns",
            "Cleet",
            "Cornbread",
            "Crab Meat",
            "Crapps",
            "Dark Skies",
            "Dennis Clawhammer",
            "Dicman",
            "Elphonso",
            "Fancypants",
            "Figgs",
            "Foncy",
            "Gootsy",
            "Greasy Jim",
            "Huckleberry",
            "Huggy",
            "Ignatious",
            "Jimbo",
            "Joe 'Pottin Soil'",
            "Johnny",
            "Lemongrass",
            "Lil Debil",
            "Longbranch",
            "'Lunch Money'",
            "Mergatroid",
            "'Mr Peabody'",
            "Oil-Can",
            "Oinks",
            "Old Scratch",
            "Ovaltine",
            "Pennywhistle",
            "Pitchfork Ben",
            "Potato Bug",
            "Pushmeet",
            "Rock Candy",
            "Schlomo",
            "Scratchensniff",
            "Scut",
            "Sid 'The Squirts'",
            "Skidmark",
            "Slaps",
            "Snakes",
            "Snoobs",
            "Snorki",
            "Soupcan Sam",
            "Spitzitout",
            "Squids",
            "Stinky",
            "Storyboard",
            "Sweet Tea",
            "TeeTee",
            "Wheezy Joe",
            "Winston 'Jazz Hands'",
            "Worms",
        ],
        last: vec![
            "Appleyard",
            "Bigmeat",
            "Bloominshine",
            "Boogerbottom",
            "Breedslovetrout",
            "Butterbaugh",
            "Clovenhoof",
            "Clutterbuck",
            "Cocktoasten",
            "Endicott",
            "Fewhairs",
            "Gooberdapple",
            "Goodensmith",
            "Goodpasture",
            "Guster",
            "Henderson",
            "Hooperbag",
            "Hoosenater",
            "Hootkins",
            "Jefferson",
            "Jenkins",
            "Jingley-Schmidt",
            "Johnson",
            "Kingfish",
            "Listenbee",
            "M'Bembo",
            "McFadden",
            "Moonshine",
            "Nettles",
            "Noseworthy",
            "Olivetti",
            "Outerbridge",
            "Overpeck",
            "Overturf",
            "Oxhandler",
            "Pealike",
            "Pennywhistle",
            "Peterson",
            "Pieplow",
            "Pinkerton",
            "Porkins",
            "Putney",
            "Quakenbush",
            "Rainwater",
            "Rosenthal",
            "Rubbins",
            "Sackrider",
            "Snuggleshine",
            "Splern",
            "Stevens",
            "Stroganoff",
            "Sugar-Gold",
            "Swackhamer",
            "Tippins",
            "Turnipseed",
            "Vinaigrette",
            "Walkingstick",
            "Wallbanger",
            "Weewax",
            "Weiners",
            "Whipkey",
            "Wigglesworth",
            "Wimplesnatch",
            "Winterkorn",
            "Woolysocks",
        ],
    };
    print!("{:?}", repeat_random_name(&name_list));
}

fn get_random_fullname(name_list: &NameList<&str>) -> String {
    let mut rng = rand::thread_rng();
    let rand_first = name_list.first.choose(&mut rng);
    let rand_last = name_list.last.choose(&mut rng);
    match (rand_first, rand_last) {
        (Some(rand_first), Some(rand_last)) => {
            return rand_first.to_string() + rand_last;
        }
        _ => return "error".to_string(),
    }
}

// TODO:入出力のエラー処理改善する
// TODO:()型を返さないようにする
fn repeat_random_name(name_list: &NameList<&str>) -> () {
    let mut input = String::new();
    loop {
        print!(
            "{}\nTry again?(Press Enter else n to quit)\n",
            get_random_fullname(name_list)
        );
        let result = io::stdin().read_line(&mut input);
        match result {
            Ok(s) => {
                if input.to_lowercase() == "n".to_string() {
                    break;
                } else {
                    continue;
                }
            }
            Err(s) => (),
        }
    }
}
