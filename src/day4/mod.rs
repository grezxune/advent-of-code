use std::collections::HashSet;

pub fn run() {
    let input = include_str!("input.txt");
    let pairs: Vec<&str> = input.split("\n").collect();

    round_one(&pairs);
    round_two(&pairs);
}

fn round_one(pairs: &Vec<&str>) {
    let mut total: u32 = 0;

    for pair in pairs.iter() {
        let pair_split: Vec<&str> = pair.split(",").collect();
        if pair_split.len() == 2 {
            let elf_one = get_range(pair_split[0]);
            let elf_two = get_range(pair_split[1]);
            let intersection: Vec<u32> = elf_one.intersection(&elf_two).map(|&x| x.to_owned()).collect();

            let elf_one_vec = elf_one.into_iter().collect::<Vec<u32>>();
            let elf_two_vec = elf_two.into_iter().collect::<Vec<u32>>();

            if intersection == elf_one_vec || intersection == elf_two_vec {
                total += 1;
            }
        }
    }

    println!("Total fully intersected: {}", total);
}

fn round_two(pairs: &Vec<&str>) {
    let mut total: u32 = 0;

    for pair in pairs.iter() {
        let pair_split: Vec<&str> = pair.split(",").collect();
        if pair_split.len() == 2 {
            let elf_one = get_range(pair_split[0]);
            let elf_two = get_range(pair_split[1]);
            let intersection: Vec<u32> = elf_one.intersection(&elf_two).map(|&x| x.to_owned()).collect();

            let elf_one_vec = elf_one.into_iter().collect::<Vec<u32>>();
            let elf_two_vec = elf_two.into_iter().collect::<Vec<u32>>();

            for slot in intersection.iter() {
                if elf_one_vec.contains(slot) || elf_two_vec.contains(slot) {
                    total += 1;
                    break;
                }
            }
        }
    }

    println!("Total with some intersected: {}", total);
}

fn get_range(hyphenated_range: &str) -> HashSet<u32> {
    let polar_values: Vec<&str> = hyphenated_range.split("-").collect();
    let min: u32 = polar_values[0].parse::<u32>().unwrap();
    let max: u32 = polar_values[1].parse::<u32>().unwrap();
    let mut hash_set: HashSet<u32> = HashSet::new();

    for val in min..=max {
        hash_set.insert(val);
    }

    return hash_set;
}
