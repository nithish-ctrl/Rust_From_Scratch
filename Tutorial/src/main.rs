fn main(){
    let name = String::from("Snigdha");
    let name2 = &name;
    println!("{name} is the original name and {name2} is the referenced name.");

    let mut first_name = first_name_extracter(&name);
    println!("The first name of the given name is {first_name}."); 

    first_name = first_name_extracter(&"Snigdha Itla");
    println!("The first name of the name Snigdha Itla is {first_name}.");

}

fn first_name_extracter(input_string : &str) -> &str {
    for (i, &item) in input_string.as_bytes().into_iter().enumerate(){
        if item == b' ' {
            return &input_string[..i];
        }
    }
    return &input_string[..];
}

/* 
Structs to Structure Realted Data 
 
 */