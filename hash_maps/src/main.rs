use rand::Rng;
use std::collections::HashMap;
use std::io;

fn main() {
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // Won't work because the hash map took ownership of the string
    // field_name.push_str("s");

    // Replaces the value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Adds value only if one doesn't exist
    scores.insert(String::from("Orange"), 15);
    scores.entry(String::from("Yellow")).or_insert(50);
    // Nothing will happen here
    scores.entry(String::from("Orange")).or_insert(40);

    println!("{scores:?}");

    // Updating the value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    println!("Pick a game: mm, pig, or dept");

    add_to_dept();

    // loop {
    //     let mut game = String::new();
    //     io::stdin()
    //         .read_line(&mut game)
    //         .expect("Failed to read line");

    //     if game.trim() == "mm" {
    //         println!("You picked median and mode");
    //         let (median, mode) = median_and_mode();
    //         dbg!(median, mode);
    //         break;
    //     } else if game.trim() == "pig" {
    //         println!("You picked pig latin");
    //         pig_latin();
    //         break;
    //     } else if game.trim() == "dept" {
    //         println!("You picked dept");
    //         add_to_dept();
    //         break;
    //     } else {
    //         println!("You must pick temps, fib or carol");
    //     }
    // }
}

// Exercises:
// Given a list of integers, use a vector and return the
// median (when sorted, the value in the middle position) and
// mode (the value that occurs most often; a hash map will be
// helpful here) of the list.
fn median_and_mode() -> (u32, u32) {
    // Create a vector of a random length with random integers
    let range = 1..=100;
    let len_of_vec = rand::thread_rng().gen_range(range);

    let mut nums = Vec::new();

    for _ in 0..len_of_vec {
        let range = 1..=100;
        let random = rand::thread_rng().gen_range(range);
        nums.push(random);
    }

    nums.sort();

    let median_int = nums.get(&len_of_vec / 2);
    let median: u32 = *median_int.unwrap_or(&0) as u32;

    let mut map = HashMap::new();

    for num in &nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mode_int = map.into_iter().max_by_key(|(_, v)| *v).map(|(k, _)| k);
    let mode: u32 = *mode_int.unwrap_or(&0) as u32;

    (median, mode)
}

// Convert strings to pig latin. The first consonant of each
// word is moved to the end of the word and ay is added, so
// first becomes irst-fay. Words that start with a vowel have
// hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!
fn pig_latin() -> String {
    // Updating the value based on the old value
    // let text = "The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.";

    println!("Enter a sentence");

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    println!("You entered: {}", text);

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut pig_str = String::new();

    for word in text.split_whitespace() {
        let first_letter = word.chars().nth(0).unwrap_or('x');
        let is_vowel = &vowels.contains(&first_letter);

        let word_without_first = word
            .chars()
            .next()
            .map(|c| &word[c.len_utf8()..])
            .unwrap_or("x");

        if *is_vowel {
            let pig_wrd = format!("{word}-hay ");
            pig_str.push_str(&pig_wrd);
        } else {
            let pig_wrd = format!("{word_without_first}-{first_letter}ay ");
            pig_str.push_str(&pig_wrd);
        }
    }

    println!("Pig Latin: {pig_str}");
    pig_str
}

// Using a hash map and vectors, create a text interface to
// allow a user to add employee names to a department in a
// company; for example, “Add Sally to Engineering” or “Add
// Amir to Sales.” Then let the user retrieve a list of all
// people in a department or all people in the company by
// department, sorted alphabetically.
fn add_to_dept() {
    let mut dept_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter input");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("You input: {}", input);

        if input.trim() == "show" {
            // dbg!(dept_map);
            // show_dept(&mut dept_map);
            print_departments(&dept_map);
            break;
        } else {
            println!("add to hash");
            let words: Vec<&str> = input.split(' ').collect();
            let name = words
                .get(1)
                .map(|&s| s)
                .unwrap_or("unknown")
                .trim()
                .to_string();
            let department = words
                .get(3)
                .map(|&s| s)
                .unwrap_or("Sales")
                .trim()
                .to_string();

            dbg!(&name);
            dbg!(&department);
            let members_of_dept = dept_map.entry(department).or_insert(Vec::new());
            members_of_dept.push(name);
            dbg!(&dept_map);
        }
    }
}

fn print_departments(dept_map: &HashMap<String, Vec<String>>) {
    for (department, members) in dept_map {
        println!("Department: {}", department);
        for member in members {
            println!(" - {}", member);
        }
    }
}

fn show_dept(map: &mut HashMap<String, Vec<String>>) {
    println!("Enter input");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You input: {}", input);

    if input.trim() == "by dept" {
        dbg!(&map);
    } else {
        dbg!(&map);
    }
}
