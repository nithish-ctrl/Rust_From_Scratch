use std::fmt;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io;

#[derive(Debug, FromPrimitive)]  
/* 
To avoid the need to implement the Debug trait manually, we can use this attribute to automatically derive the Debug trait for the enum. This allows us to use the {:?} format specifier to print the enum values in a 
human-readable format.But there is less customization available with the Debug trait. If we want to customize the output format of the enum values, we can implement the Display trait instead. Here, 
the name will be straight up printed as it is, but if we want to customize the output format, we can implement the Display trait for the enum. This allows us to define how the enum values should be formatted when 
printed using the {} format specifier. The debug trait is useful for debugging purposes, while the custom trait is useful for user-facing output. Depending on the use case, we can choose to implement either or both traits for our enum.
*/
enum CharacterClass {
// The enum values are assigned integer values starting from 1. The first variant, Warrior, is assigned the value 1, and the subsequent variants are assigned incrementing values (2 for Mage, 3 for Rogue, and so on).      
        Warrior = 1,
        Mage = 2,
        Rogue = 3,
        Assassin = 4,
        Healer = 5,
}

// Custom implementation of the Display trait for the CharacterClass enum. This allows us to define how the enum values should be formatted when printed using the {} format specifier.
impl fmt::Display for CharacterClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CharacterClass::Warrior => write!(f, "Warrior"),
            CharacterClass::Mage => write!(f, "Mage"),
            CharacterClass::Rogue => write!(f, "Rogue"),
            CharacterClass::Assassin => write!(f, "Assassin"),
            CharacterClass::Healer => write!(f, "Healer"),                                                                          
        }
    }
}

// We are gonna write a trait to take a number and return the corresponding enum value. 
// This is useful when we want to convert a number (e.g., from user input or data) into the corresponding enum variant.
impl CharacterClass {
    fn from_number(num: u8) -> CharacterClass {
        match num {
            1 => CharacterClass::Warrior,
            2 => CharacterClass::Mage,
            3 => CharacterClass::Rogue,
            4 => CharacterClass::Assassin,
            5 => CharacterClass::Healer,
            _ => panic!("Invalid character class number"),
        }
    }
} 


fn main() {
    let _character_class1 = CharacterClass::Warrior;
    let _character_class2 = CharacterClass::Mage;
    let _character_class3 = CharacterClass::Rogue;
    let _character_class4 = CharacterClass::Assassin;
    let _character_class5 = CharacterClass::Healer;

    // Tuples
    let character1 = ("Mongrel", CharacterClass::Warrior, 100, 50);
    let character2 = ("Azure", CharacterClass::Mage, 80, 120);

    // Arrays
    let characters = [character1, character2];

    // Looping through an array

    for character in characters{
        println!("Character Name : {}", character.0); // The result will be based on the implementation of the Display trait for the CharacterClass enum. The {} format specifier will call the fmt method of the Display trait, which will return the string representation of the enum value.
        println!("Character Class : {}", character.1);
        println!("Character Health : {}", character.2);
        println!("Character Mana : {}", character.3);
    }
/* 
    for character in characters.iter(){
        println!("Character Name : {}", character.0);
        println!("Character Class : {:?}", character.1);
        println!("Character Health : {}", character.2);
        println!("Character Mana : {}", character.3);
    }
    */

    rewards();
}

fn rewards(){
    // Array of tuples
    let mobs = [
        ("Goblin", 10),
        ("Orc", 20),
        ("Troll", 30),
        ("Dragon", 100),
    ];      
    println!("Mobs and their reward : {:?}", mobs);

    // Array definition with fixed size and type 
    let mut map = [[0; 5]; 5]; 
    println!("Map : {map:?}");

    map[2][4] = 1;
    map[3][4] = 3;
    map[2][3] = 2;
    map[2][2] = 5;
    map[1][1] = 4;

    for row in map{
        println!("{row:?}");
    }

    // Vector 
    let mut kill: Vec<(String, i32)> = Vec::new();
    kill.push(("Goblin".to_string(), 10));
    kill.push(("Orc".to_string(), 20));

    println!("Killed Mobs : {:?}", kill);

    let mut unlocked: Vec<CharacterClass> = Vec::new();
    unlocked.push(CharacterClass::Warrior);
    unlocked.push(CharacterClass::Mage);
    println!("Unlocked Classes : {:?}", unlocked);

    let mut unlocked_custom: Vec<CharacterClass> = Vec::new();
    /*  This could be confusing - Actually Im supposed use the gem class instead of character class. The map value 
    stores the location and we take in data from it and update the unlocked character using our custom trait. 
    But I did not make it in such a way so how we can visualize it for now is that we can randomly unlock some characters
    by moving through the map, I will fix it by making an enum for mobs and defining a custom trait for them and taking the
    data from the map and then pushing it to the unlocked characters. */ 
    let mut data = map[2][4];
    unlocked_custom.push(CharacterClass::from_number(data));
    println!("Unlocked Classes with Custom trait : {:?}", unlocked_custom);

    /* Converting from a number to an enum value and vice versa is hard so theres a crate we can use for that. 
    In terminal : 
    cargo add num-derive num-traits

    Then 
    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;
    */
    data = map[3][4];
    unlocked_custom.push(CharacterClass::from_u8(data).unwrap()); // replace our custom trait with the FromPrimitive trait from the num-derive and num-traits crates. This allows us to convert from a number to an enum value using the from_u8 method provided by the FromPrimitive trait. 
    ///The unwrap() method is used to handle the Option returned by from_u8, assuming that the conversion will always succeed for valid numbers.
    println!("unlocked classes with num-derive and num-traits : {:?}", unlocked_custom);


    // Looping the character unlocking by walking through the map. 
    while unlocked_custom.len() < 5 {
    // Taking input 
    println!("Search an X Y position in the map 0-4 (example input : 2 4) : ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {

        }
        Err(_) =>
        {
            println!("Failed to read the line.");
        }
    }

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        println!("Invalid inpput, please provide two numbers separated by a space.");
        continue;
    }

    let (x, y) = match (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
        (Ok(x), Ok(y)) => (x, y),
        _ => {
            println!("Invalid input !");
            continue;
        }

    };

    if x>=5 || y>=5 {
        println!("Invalid index");
        continue;
    }

    data = map[x as usize][y as usize];

    let _character = match CharacterClass::from_u8(data){
        Some(_character) => _character,
        None => {
            println!("No character found at this position.");
            continue;
        }
    };
    unlocked_custom.push(_character);
    map[x as usize][y as usize] = 0;
    // unlocked_custom.push(CharacterClass::from_u8(data).expect("Invalid input is given")); Removed this and added the above match to prevent from panicing. 
    println!("{data:?} is found at the position ({x}, {y}) and the corresponding classes are unlocked : {:?}", unlocked_custom);
    }
    println!("You found the required number of characters, Congrats !!");

    // Hash maps
    let mut character_values: HashMap<Gem, f64> = HashMap::new();
}

