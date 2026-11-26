struct Rectangle{
        width: u32,
        height: u32,
    }

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

    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.",
        area(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is using tuple struct is {} square pixels.", 
    area_with_tuples(rect1));


    let rectangle1 = Rectangle{
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle using struct tuples is {} square pixels.", 
    area_with_struct_tuples(&rectangle1));

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
    println!("Hello, {frstName} {lstName}. It is nice meeting you.");
}

fn area(width : u32, height : u32) -> u32 {
    width * height
}

fn area_with_tuples(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_with_struct_tuples(rectangle: &Rectangle) ->u32 {
    rectangle.width * rectangle.height
}