enum CharacterClass {
        warrior,
        mage,
        rogue,
        assassin,
        healer,
}
fn main() {
    let character_class1 = CharacterClass::warrior;
    let charcter_class2 = CharacterClass::mage;
    

    // Tuples
    let character1 = ("Mongrel", CharacterClass::warrior, 100, 50_f32);
    let character2 = ("Azure", CharacterClass::mage, 80, 120_f32);

    // Arrays
    let characters = [character1, character2];

    // Looping through an array

    for character in characters.items(){
        println!("Character Name : {}", character.0);
        println!("Character Class : {:?}", character.1);
        println!("Character Health : {}", character.2);
        println!("Character Mana : {}", character.3);
    }
}
