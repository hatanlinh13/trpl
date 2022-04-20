use std::collections::HashMap;

pub fn mean(arr: &Vec<i32>) -> i32
{
    let mut sum = 0;
    let mut count = 0;
    for elem in arr {
        sum += elem;
        count += 1;
    }
    sum / count
}

pub fn median(arr: &Vec<i32>) -> i32
{
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    match sorted_arr.get(sorted_arr.len() / 2) {
        None => 0,
        Some(value) => *value,
    }
}

pub fn mode(arr: &Vec<i32>) -> i32
{
    let mut appearances: HashMap<i32, i32> = HashMap::new();
    for value in arr {
        let count = appearances.entry(*value).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut max = 0;
    for (key, value) in appearances {
        if value > max {
            max = value;
            mode = key;
        }
    }
    mode
}

pub fn pig_latin(text: &str) -> String
{
    let mut pigged_text = String::new();
    for word in text.split(" ") {
        if !pigged_text.is_empty() {
            pigged_text.push_str(" ");
        }
        let first_char = word.chars().nth(0).unwrap();
        let (prefix, suffix) = match first_char {
            'a' | 'o' | 'e' | 'u' | 'i' => (word.to_string(), "h".to_string()),
            other => (word[1..].to_string(), other.to_string()),
        };
        let pigged_word = format!("{}-{}ay", prefix, suffix);
        pigged_text.push_str(&pigged_word);
    }
    pigged_text
}

pub fn add_employee(name: &str, department: &str, company: &mut HashMap<String, Vec<String>>)
{
    company.entry(department.to_string())
           .or_insert_with(Vec::new)
           .push(name.to_string());
}

pub fn list_employee_in_department(department: &str, company: &HashMap<String, Vec<String>>)
{
    match company.get(department) {
        Some(employees) => {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            println!("Employees of department {}:", department);
            for employee in sorted_employees {
                println!("{}", employee);
            }
        }
        None => {
            println!("Department {} not found.", department);
        }
    }
}

pub fn list_all_employee(company: &HashMap<String, Vec<String>>)
{
    println!("Employees of company:");
    for (key, value) in company.iter() {
        let mut sorted_employees = value.clone();
        sorted_employees.sort();
        for employee in sorted_employees {
            println!("{} - {}", &key, &employee);
        }
    }
}
