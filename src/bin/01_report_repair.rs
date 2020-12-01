const EXPECTED_SUM: usize = 2020;

fn entry_2020_sum_2(entries: &Vec<usize>) -> Option<usize> {
    for (i, entry_1) in entries.iter().enumerate() {
        for entry_2 in entries.iter().skip(i + 1) {
            if entry_1 + entry_2 == EXPECTED_SUM {
                return Some(entry_1 * entry_2);
            }
        }
    }

    None
}

fn entry_2020_sum_3(entries: &Vec<usize>) -> Option<usize> {
    for (i, entry_1) in entries.iter().enumerate() {
        for (j, entry_2) in entries.iter().skip(i + 1).enumerate() {
            for entry_3 in entries.iter().skip(j + 1) {
                if entry_1 + entry_2 + entry_3 == EXPECTED_SUM {
                    return Some(entry_1 * entry_2 * entry_3);
                }
            }
        }
    }

    None
}

fn main() {
    let entries = adventofcode::read_input("inputs/day01.txt", |l| {
        l.parse::<usize>().expect("unable to parse line")
    });

    println!(
        "{}",
        entry_2020_sum_2(&entries).expect("unable to find matching entries")
    );
    println!(
        "{}",
        entry_2020_sum_3(&entries).expect("unable to find matching entries")
    );
}
