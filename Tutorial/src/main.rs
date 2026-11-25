fn main(){
    let name = String::from("Snigdha");
    let name2 = &name;
    println!("{name} is the original name and {name2} is the referenced name.");

    let mut first_name = first_name_extracter(&name);
    println!("The first name of the given name is {first_name}."); 

    first_name = first_name_extracter(&"Snigdha Itla");
    println!("The first name of the name Snigdha Itla is {first_name}.");

    let mut initial_name = String::from("Snigdha");
    change_name_with_mutable_reference(&mut initial_name);

    let firstName = String::from("Snigdha");
    let lastName = String::from("Itla");
    greet(&firstName, &lastName);
}

fn first_name_extracter(input_string : &str) -> &str {
    for (i, &item) in input_string.as_bytes().into_iter().enumerate(){
        if item == b' ' {
            return &input_string[..i];
        }
    }
    return &input_string[..];
}

fn change_name_with_mutable_reference(input_string: &mut String){
    *input_string = String::from("Snigdha Itla");
    println!("The name as been changed to {input_string}.");

}

fn greet(frstName : &String, lstName: &String){
    println!("Hello, {frstName} {lstName}. It was nice meeting you.");
}

/* 
Structs to Structure Realted Data 

 */