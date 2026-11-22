use std::fmt;

#[derive(Debug)]  
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
    data = map[1][2];
}

