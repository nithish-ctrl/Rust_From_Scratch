# Rust from Scratch 

Lets learn Rust from Scratch 
Will speed through the concepts or stright up implementation

I will write the thought process here and the content I'm learning here with small code snippets. 

I am following this youtube video : link
The implementation may contain the mix of the code of this video and my personal implementation around them. 

Cargo is built on top of rustc
We can straight up make a rust file in a dir and run it using rustc <file_name>.
We can also create a project using cargo new <folder_name>
The folder will have a src file and a cargo.toml. cargo.lock file.
src file is where all the code goes.
Cargo.toml file is used to list all the versionings and dependencies to use in the code. 
Cargo.lock is to lock the package to any specific versioning. 
src file will contain the main.rs file. 
Also there will be a target folder created everytime run the code. 
There will also be a .gitignore file created with /target in it to avoid push the target folder to git. 

Lets start coding now, 
``` rust
fn main(){
    println!("Hello World");
}
```

Println! is a macro, any macro can be indicated using the '!' mark behind it. 

``` rust
fn main(){
    let name = "Sniggy";
    println!("Name : {name}");
}
```

Variables are immutable by default by rust unless we expilictly say so while we define them. Any tries to change a immutable variable results in an error as it should. 

``` rust
fn main(){
    let mut name = "Sniggy";
    println!("Name : {name}");

    name = "Snigdha";
    println!("Name : {name}");
}
```

Printing a varible within a string can be done two ways : 

``` rust 
println!("Her name is {name}");

OR 

println("Her name is {}", name);
```

When to use which one : 
1. Direct usage of the name inside the curly braces can be used when we are directly using the variable. 

2. Using the variable followed by comma is to be followed when we call a function or any operation is carried out instead of just directly calling the variable.

``` rust 
fn main(){
    let mut first_name = "Nithish";
    let mut last_name = "Ravichandran";
    println!("My first name is {first_name} and my last name is {last_name}.");
    println!("My first name is {} and my last name is {}", first_name.to_lowercase(), last_name);
}
```

Let's implement some arrays
``` rust 
fn main(){
    let data = [1, 2, 4, 5, 6];
    println!("{data}");
}
```
This will give an error, we can use :? to avoid this. This will enable debug format, it is not good practise to use it to print in some application. This is merely to print the variable to the terminal to see it without getting any custom control for the data showing up. 

``` rust
println!("{data:?}");
```
Any string we made earlier are called a literal, that will be a constant value in your executable. Theres a way to make dynamic strings. 

``` rust
fn main(){
    let mut name = String::from("Snigdha");
    println!("");
}
```
This String method can be used to define the value like above or while we are taking input from the user when we don't know the value in advance. 

``` rust 
fn main(){
    let name = String::new();
    println!("{name}"); // This will print an empty string. 
}
```

Let's read some input from the user now, 

``` rust 
use std::io;
fn main(){
    println!("Whats your name :");
    let mut name = String::new();
    io::stdin().read_line(name);
}
```
This gives an error because we need to pass the mutable reference and not the string itself. Also the variable has to be defined mutable. 

``` rust 
io.stdin().read_line(&mut name);
```

We can a expect to reply when a error pops up. 

``` rust
use std::io; 
fn main(){
    io::stdin().read_line(&mut name).expect("Error reading the name.");
    println!("The name is : {}", name.trim());
}
```
### Loops

1. loop 
This loop will run till the break command is reached. It has to be explicitly broken. 
``` rust 
loop {
    break;
}
```

2. for
This loop will run for specific number of times.
``` rust
// With an iter : '=' sign indicates that the upper limit is also included. 
for i in 1..=5 {
    println!("{i}"); 
}

// To loop through an array
for number in numbers{
    println!("{number}");
}
```

3. while
This loop will run as long as the condition is true. 
``` rust 
while iter <= max_number {
    println("The iter:{} is smaller than max_num:{}", iter, max_number);
}
```


### Structs and Collections 
Covering some collections and structs 

Lets build a small game and implement them. 
The file for this is 'Shadow_Lord'. 

Enum Definition : 
``` rust 
enum CharacterClass {
    warrior = 1,
    Mage,
    Rogue,
}
```

We can assign values to them, like how we did to warrior. Other values will get incremented to one in sequential order even if we did not assign them values. 

Usage : 
``` rust
let character1 = ("Mongrel", CharacterClass::Warrior, 100, 50);
let character2 = ("Azure", CharacterClass::Mage, 75, 55);
```
The elements inside the tuple are Character name, Character class, Health of the Character, Mana of the character. 

#### Traits 

We will look into Display Trait 
- We can custom implement a trait for the character class. This enables us to define how the enum values should be formatted when printed using the {} format specifier. 

- We can also use the Debug trait, this is useful for debugging purposes, while the custom trait is useful for user-facing output. 

``` rust 
// Debug trait 
#[derive(Debug)]

// Custom implementation
impl fmt::Display for CharacterClass{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::result{
        match self{
            CharacterClass::Warrior => write!(f, "Warrior - Tank or the elegant sword type."),
            CharacterClass::Mage => write!(f, "Mage - Uses magic as the weapon"),
            CharacterClass::Rogue => write!(f, "Rogue - Brawler, no elegancy, ruthless swings and dirty moves."),
            CharacterClass::Assassin => write!(f, "Assassin - One attack to eliminate the enemy, more of hide and attack."),
            CharacterClass::Healer => write!(f, "Healer - To heal the troops in the army, does minimal to no damage and has insane health stats."),
        }
    }
}
```
The string above will be displayed for its corresponding character class. 

### Arrays 

Collections of items. 

Defining an array : 
``` rust 
let characters = [character1, character2];
```

Accesing elements from an array : 

``` rust 
for character in characters{
    println!("Character Name : {}", character.0);
    println!("Character Class : {}", character.1);
    println!("Character Health : {}", character.2);
    println!("Character Mana : {}", character.3);
}
```
We use dot notation to access elements from an array. 
The output of them will be dependent on the trait we used for display.

Array defintion with fixed size and type
``` rust
let mut map = [[0; 5]; 5];
println!("Map : {map:?}");

map[2][4] = 1;
map[3][4] = 3;
```

Accessing elements in a array 
``` rust
for row in map{
    println!("{row:?}");
}
```

### Vector definition and pushing elements into vectors
``` rust 
let mut kill: Vec<(String, i32)> = Vec::new();
kill.push(("Goblin".to_string(), 10));
kill.push(("Orc".to_string(), 20));

println!("Killed Mobs : {:?}", kill);
```

Lets make a vector called unlocked pokemons and walk through the map and push the the pokemon founds in the map into it. 

``` rust
enum PokemonClass {
    Pikachu = 1,
    Charmander = 2,
    Chikorita = 3,
    Abra = 4,
}

impl PokemonClass{
    fn from_map(num : u8) -> PokemonClass {
        match num {
            1 => PokemonClass::Pikachu,
            2 => PokemonClass::Charmander,
            3 => PokemonClass::Charizard,
            4 => PokemonClass::Blastoise,
            _ => panic!("Invalid Pokemon "),
        }
    }
}

let mut unlocked: Vec<PokemonClass> = Vec::new();

let mut data = map[2][4];
unlocked.push(PokemonClass::from_map(data));
unlocked.push(PokemonClass::from_map(data));
println!("Unlocked Pokemons : {:?}", unlocked);
```

This could also be done with a rust crate 

``` rust 
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

data = map[3][4];
unlocked.push(PokemonClass::from_u8(data).unwrap());
println!("Unlocked Pokemons from map : {:?}", unlocked);
```

Lets let a loop now to walk through the map and unlock 5 pokemons. 

``` rust
// Pokedex logic
while unlocked.len() < 5 {
    println!("Search for an X Y position in the map 0-4 (example input : 2 3) : ");
    
    let mut input = String::new();
    match io::stdin().readline(&mut input){
        Ok(_) => {

        }
        Err(_) => {
            println!("Failed to read the line");
        }
    }

    let parts: Vec<&str> = input.trim().split_whitespaces().collect();

    if parts.len() != 2 {
        println!("invalid input, please provide two numbers separtely");
        continue;
    }

    let (x,y) = match (parts[0].parse::<u8>(), part[1].parse::<u8>()){
        (Ok(x), Ok(y)) => (x, y);
        _ => {
            println!("Invalid Input ");
            continue;
        }
    };

    if x>=5 || y>=5 {
        println!("Invalid index");
        continue;
    }

    data = map[x as usize][y as usize];

    let pokemon = match PokemonClass::from_u8(data){
        some(pokemon) => pokemon,
        None => {
            println!("No character found at this position.");
            continue;
        }
    };

    unlocked.push(pokemon);
    map[x for usize][y for usize] = 0;

    println!("{data:?} is found at the position {{x}, {y}} and the corresponding pokemons are unlocked : {unlocked:?}", unlocked);
}
println!("You found the required numbers of pokemon Congrats");
```

### Hash map and its definition









### Scope 
Basically where the defined variables and functions or constants are valid or could be called and utilised. 

### Reference

```  rust 
fn main(){
    let name = String::from("Snigdha");
    let n = &name;
    println!("{n}");
}
```

### Lifetime 

``` rust 
fn main(){
    let n : &String;
    {
        let name = String::from("Snigdha");
        n = &name;
    }
    println!("{n}");
}
```

This would raise an error since the variable name does not does live long enough (Literally thats wot the compiler would say), thats because n is defined in a wider scope while name is not as referencing name to n will not exist after the scope of name exhausts - Rust wont allow this.

This can be avoided by two ways : 
1. Both of them existing in the same scope. 
``` rust 
fn main(){
    let n: &String;
    let name = String::from("Snigdha");
    n = &name;
    println("{n}");
}
```

2. Usage of the terms is within the existing of the variable with a smaller scope. 
``` rust 
fn main(){
    let n: &String;
    {
        let name = String::from("Snigdha");
        n = &name;
        println!("{n}");
    }
}
```

We have define them in the same scope or ensure the usage of the variables are within the lifetime of the variables involved. 

### Ownership 
This is an approach to memory management that rust uses instead of a garbage collector. 

``` rust 
fn main(){
    let first = String::from("Snigdha");
    let name = first;
    println!("{first}") // Raises an error
}
```
This raises an error since rust drops the value of first the moment it is assigned to name.

``` rust 
fn main(){
    let score = 50;
    let points = score;
    println!("{score}") // This works
}
```
This works and does not raise an error, here they are being copied instead of being moved. They are like two independent variables who just happen to have same values. Some type will copy while other wont, how do we know which of them copy and which of them will be moved. 

Whenever copy happens they will have same value but different address in the memory so they can be changed independently. String cannot implement copy. But it does have a clone implementation which have to be explicitly called. 

``` rust 
fn main(){
    let name = String::from("Snigdha");
    let name2 = name.clone();
    println!("{name} and {name2}");
}
```

But the best way to not invoke clone and use them from the same memory address for types that move data is Reference. 

``` rust 
fn main(){
    let name = String::from("Snigdha");
    let name2 = &name
    println!("{name} is the original name and {name2} is the referenced name.");
}
```
We can also make mutliple references of the same data. 

``` rust 
fn main(){
    let name = String::from("Snigdha");
    let name2 = &name
    let name3 = &name
    let name4 = &name
    let name5 = &name
    println!("{name} is the original name and {name2}, {name3}, {name4}, {name5} are the referenced name.");
}
```
There can be multiple references of the same data as long as they are read only aka immutable reference of the data. There are also mutable references but they cannot co exist with other mutable references or immutable refernces in a scope. Multiple immutable references can exist in a scope without any errors. 

``` rust 
fn main(){
    let mut name = String::from("Snigdha"); // Mutuable data
    let name1 = &mut name;
    *name1 = String::from("Snigdha Itla"); // Dereferencing data with a asterik *. 
    println!("Name : {name}");
}
```
Similarly, we cant access the value and its mutable reference at the same time. 

``` rust 
let mut name = String::from("Smigdha");
let name1 = &mut name;
*name1 = String::from("Snigdha Itla");
println!("Name : (name), Referenced Name : {name1}");
```
The above code would raise an error because we tried to use the value and its mutable reference at the same time. 
This is due to non lexical lifetimes. 

``` rust 
let mut name = String::from("Snigdha");
let name1 = &mut name;
*name1 = String::from("Snigdha Itla");
println!("{name1}");
println!("{name}");
```
The above code would work because the lifetime of the reference ends with its last usage which is basically the first print statement. The mutable reference lifetime ends here. So, its fine to use the original data after this. 

So in summary, we can use one mutable reference or multiple immutable references. Also, the value reference should be valid like the reference cannot outlive the data. 

### Functions & Ownership
If we try to pass a parameter to a function, the value gets passed from the value to the function. 

``` rust 
fn greet(frstName : &String, lstName: &String){
    println!("Hello, {frstName} {lstName}. It is nice meeting you.");
}

fn main(){
    let first_name = String::from("Snigdha");
    let last_name = String::from("Itla");
    greet(&first_name, &last_name);
}
```

We used reference here because the value gets moved to the function. The other option is would be to return the input value from the string and assign it back to the input variables. 

``` rust 
fn greet(frstName : String, lstName : String) -> (String, String) {
    println!("Hello, {frstName} {lstName}. It is nice meeting you.");
    return (frstName, lstName)
}

fn main(){
    let mut first_name = "Snigdha";
    let mut last_name = "Itla";
    first_name, last_name = greet(first_name, last_name);
}
```

Lets create a function where we generate data inside that and return it to the variables. 

``` rust 
fn generate_names() -> (String, String){
    let first = String::from("Snigdha");
    let last = String::from("Itla");
}

fn main(){
    let (mut first, mut last) = generate_names();
    println!("Hello {first} {last}.");
}
```
### Mutable Parameters
Basically we pass some data to a function and they have the ability to modify this data. 

``` rust 
use std::mem;

fn swap(first: &mut String, last: &mut String){
    mem::swap(first, last);
}

fn main(){
    let mut first_name = "Snigdha";
    let mut last_name = "Itla";
    swap(&mut first_name, &mut last_name);
}
```

### Slices
Basically access a part of a string or an array instead of the whole string or array. 

``` rust
fn main(){
    let name = String::from("Snigdha Itla");
    let last = &name[9..];

    println!("The name is {name} and last name is {last}");
}
```

This is the second string type and this is what we use as a string literal.

``` rust 
let name = "Snigdha"; // This is a string slice data type. 
```

The String is a dynamic string type. Mostly in the functions, the parameters are defined as string slices instead of String. We can reference this string literals using &str while we use &String for String. 

``` rust 
fn greet(frst: &str, lst: &str){
    println!("Hello {frst} {lst}, nice meeting you.");
}

fn main(){
    let first = "Snigdha";
    let last = "Itla";
    greet(&first, &last)
}
```
Or we can also use the .as_str()

``` rust
fn greet(frst: &str, lst: &str){
    println!("Hello {frst} {lst}, nice meeting you.");
}

fn main(){
    let first = String::from("Snigdha");
    let last = String::from("Itla");
    greet(&first.as_str(), &last.as_str())
}
```