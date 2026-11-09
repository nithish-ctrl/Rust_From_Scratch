use std::fmt;

#[derive(Debug)]  
/* 
To avoid the need to implement the Debug trait manually, we can use this attribute to automatically derive the Debug trait for the enum. This allows us to use the {:?} format specifier to print the enum values in a 
human-readable format.But there is less customization available with the Debug trait. If we want to customize the output format of the enum values, we can implement the Display trait instead. Here, 
the name will be straight up printed as it is, but if we want to customize the output format, we can implement the Display trait for the enum. This allows us to define how the enum values should be formatted when 
printed using the {} format specifier. The debug trait is useful for debugging purposes, while the display trait is useful for user-facing output. Depending on the use case, we can choose to implement either or both traits for our enum.
*/
enum CharacterClass {
        Warrior,
        Mage,
        Rogue,
        Assassin,
        Healer,
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
    let mobs = [
        ("Goblin", 10),
        ("Orc", 20),
        ("Troll", 30),
        ("Dragon", 100),
    ];      

    let mut map = [[false; 5]; 5]; 
    println!("Map : {map:?}");

    map[2][4] = true;
    map[3][4] = true;
    map[2][3] = true;
    map[2][2] = true;
    map[1][1] = true;

    for row in map{
        println!("{row:?}");
    }
}
