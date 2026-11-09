# Rust from Scratch 

Lets learn Rust from Scratch 
The speed through of the concepts or stright up implementation

I will write the thought process here and the content I'm learning here with small code snippets. 

I am following this youtube video : link
The implementation may contain the mix of the code of this video and my personal implementation around them. 

Cargo is built on top of rustc
We can stright up make a rust file in a dir and run it using rustc <file_name>.
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

We can assign values to them, like how we did to warrior. Other values will get incremented to one in sequential order.

While usage 
``` rust
let character1 = ("Mongrel", CharacterClass::Warrior, 100, 50);
let character2 = ("Azure", CharacterClass::Mage, 75, 55);
```

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
            CharacterClass::Warrior => write!(f, "Warrior"),
            CharacterClass::Mage => write!(f, "Mage Solider"),
            CharacterClass::Rogue => write!(f, "Rogue"),
            CharacterClass::Assassin => write!(f, "Assassin"),
            CharacterClass::Healer => write!(f, "Healer"),
        }
    }
}
```


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
The output of them will be dependent on the trait we used for display. 