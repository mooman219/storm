use rand;
use rand::distributions::{Range, Sample};

#[derive(Clone)]
pub struct Character {
    pub name: String,
    //current is the active in use value, total is the highest it can go without some other system in place
    pub current_health: i32,
    pub total_health: i32,

    pub current_attack: i32,
    pub total_attack: i32,

    pub current_speed: i32,
    pub total_speed: i32,
}

impl Character {
    pub fn new(name: String, health: i32, attack: i32, speed: i32) -> Character {
        Character {
            name,
            current_health: health,
            total_health: health,
            current_attack: attack,
            total_attack: attack,
            current_speed: speed,
            total_speed: speed,
        }
    }

    //regular random character, feel, well, random, so what if we could do a more genetics based way
    //if we can break down each character into a string of characterics, standardizes this into format
    //then use a human version of child generation, create two parents, "mate" them to create a child
    //and that is the character we genreate
    //allow for both passing traits, names, characterics, down to childern, but also allow them to
    //mutate, and mutate in different ways
    //example, Jean-Louis Жуков, has a child with 诸葛亮 Grant
    //possible childern, Jean-Louis Grant, Jean-Louis Жуков, 诸葛亮 Жуков, Freddie Grant, George Жуков

    pub fn create_random_character() -> Character {
        let mut rng = rand::thread_rng();

        let mut num_range = Range::new(0, 16);
        let first_name = num_range.sample(&mut rng);
        let second_name = num_range.sample(&mut rng);

        let first_names = [
            "Jean-Louis",
            "George",
            "诸葛亮",
            "Freddie",
            "Billy-Joe",
            "Robert",
            "Uylsses",
            "Simon",
            "Георгий",
            "Никита",
            "იოსებ",
            "Ἀλέξανδρος",
            "Toussaint",
            "이",
            "嬴",
            "秦",
        ];
        let last_names = [
            "육군 원수",
            "올리언즈",
            "孔明",
            "Patton",
            "The Great",
            "Marquis",
            "Grant",
            "Bolivar",
            "Жуков",
            "Хрущёв",
            "სტალინი",
            "Μέγας",
            "Louverture",
            "순신",
            "政",
            "始皇",
        ];
        let random_name = first_names[first_name].to_owned() + " " + last_names[second_name];

        Character {
            name: random_name,
            current_health: 5,
            total_health: 5,
            current_attack: 2,
            total_attack: 2,
            current_speed: 2,
            total_speed: 2,
        }
    }
}
