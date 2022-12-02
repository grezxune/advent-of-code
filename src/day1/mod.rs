pub fn run() {
    let input = include_str!("input.txt");

    let elves: Vec<&str> = input.split("\n\n").collect();

    let elf_calories_int: Vec<Vec<i32>> = elves
        .iter()
        .map(|bag|
        {
            let items: Vec<&str> = bag.split("\n").collect();
            items
                .into_iter()
                .map(|item| {
                    if item.chars().count() > 1 {
                        return item.parse::<i32>().unwrap();
                    }
                    else {
                        return 0;
                    }
                })
                .collect()
        }).collect();

    let mut sums: Vec<i32> = elf_calories_int
        .into_iter()
        .map(|bag| bag.into_iter().sum())
        .collect();

    sums.sort();
    sums.reverse();

    println!("MAX: {}", sums[0]);

    println!("MAX 3: {}", sums[0] + sums[1] + sums[2]);
}
