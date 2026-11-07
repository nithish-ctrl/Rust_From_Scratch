# Rust from Scratch 

Lets learn Rust from Scratch 
The speed through of the concepts or stright up implementation

I will write the though process here and the content I'm learning here with small code snippets. 

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
