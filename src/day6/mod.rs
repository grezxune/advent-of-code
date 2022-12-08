pub fn run() {
    let input = std::fs::read_to_string("./day6/input.txt").expect("Should be data");

    find_marker(&input, 4);
    find_marker(&input, 14);
}

fn find_marker(s: &str, marker_len: usize) {
    let mut buffer: Vec<char> = Vec::new();

    for (i, x) in s.char_indices() {
        if buffer.len() < marker_len {
            buffer.push(x);
        } else {
            let (_, new) = buffer.split_at(2);
            buffer = new.to_vec();
            buffer.push(x);
        }

        if buffer.len() == marker_len && check_for_marker(buffer.iter().collect::<String>()) {
            println!("HERE IT IS {}", buffer.iter().collect::<String>());
            println!("Index {} {}", i + 1, x);
            break;
        }
    }
}

fn check_for_marker(s: String) -> bool {
    let mut iter: Vec<char> = vec![];

    for x in s.chars() {
        if iter.contains(&x) {
            return false;
        }

        iter.push(x);
    }

    return true;
}
