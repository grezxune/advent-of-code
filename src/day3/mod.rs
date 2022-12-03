pub fn run() {
    let input = include_str!("input.txt");
    let rucksacks: Vec<&str> = input.split("\n").collect();

    round_one(&rucksacks);
    round_two(&rucksacks);
}

fn round_one(rucksacks: &Vec<&str>) {
    let mut total: u32 = 0; 

    for rucksack in rucksacks.iter() {
        if rucksack.len() > 0 {
            let compartments = split_rucksack(rucksack);
            let common_item = find_common_item(compartments);

            match common_item {
                Ok(x) => total += convert_to_priority(x),
                Err(err) => println!("Error! {}", err)
            }
        }
    }

    println!("Total priority misplaced: {}", total);
}

fn round_two(rucksacks: &Vec<&str>) {
    let mut total: u32 = 0;
    let mut iter = rucksacks.iter();
    let mut groups: Vec<Vec<&str>> = vec![];

    for _ in 0..rucksacks.len() / 3 {
        groups.push(vec![
            iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()
        ]);
    }

    for group in groups.iter() {
        let common_item = find_common_item(group.to_vec());
        match common_item {
            Ok(x) => total += convert_to_priority(x),
            Err(err) => println!("Error! {}", err)
        }
    }

    println!("Badge priority total: {}", total);
}

fn split_rucksack(rucksack: &str) -> Vec<&str> {
    let compartment_size = rucksack.len() / 2;
    let compartment_one = &rucksack[..compartment_size];
    let compartment_two = &rucksack[compartment_size..];

    vec![compartment_one, compartment_two]
}

fn find_common_item(compartments: Vec<&str>) -> Result<char, String> {
    if compartments.len() == 2 {
        let compartment_one = compartments[0];
        let compartment_two = compartments[1];

        for x in compartment_one.chars() {
            if compartment_two.contains(x) {
                return Ok(x)
            }
        }
    } else if compartments.len() == 3 {
        let compartment_one = compartments[0];
        let compartment_two = compartments[1];
        let compartment_three = compartments[2];

        for x in compartment_one.chars() {
            if compartment_two.contains(x) && compartment_three.contains(x) {
                return Ok(x)
            }
        }
    }

    Err("Invalid compartment length.".to_string())
}

fn convert_to_priority(item_type: char) -> u32 {
    let ascii_value = item_type as u32;

    let mut priority: u32 = 0;

    if ascii_value >= 65 && ascii_value <= 90 {
        priority = ascii_value - 38;
    } else if ascii_value >= 97 && ascii_value <= 122 {
        priority = ascii_value - 96;
    }

    priority
}
