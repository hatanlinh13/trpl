mod exercises;

fn main()
{
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    dbg!(&map);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let arr = vec![1, 2, 5, 2, 2];
    println!("Mean of array is: {}", exercises::mean(&arr));
    println!("Median of array is: {}", exercises::median(&arr));
    println!("Mode of array is: {}", exercises::mode(&arr));

    println!("the lazy fox jumps over\nPig latin: {}",
             exercises::pig_latin("the lazy fox jumps over"));

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    exercises::add_employee("Linh", "Developer", &mut company);
    exercises::add_employee("Someone2", "Developer", &mut company);
    exercises::add_employee("Someone3", "HR", &mut company);
    exercises::add_employee("Someone4", "IT", &mut company);

    exercises::list_employee_in_department("Sales", &company);
    exercises::list_employee_in_department("Developer", &company);

    exercises::list_all_employee(&company);
}
