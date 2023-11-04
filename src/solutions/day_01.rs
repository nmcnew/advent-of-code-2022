pub fn get_packrat(input: &String) -> Option<(usize, i32)>{
    //split on two new lines to get each elf's bags
    input
    .split("\n\n")
    .map(|elf| 
        elf.split("\n")
        .map(|meal| meal.parse::<i32>().expect("Should only have integers on each line"))
        .sum()
    )
    .collect::<Vec<i32>>()
    .iter()
    .enumerate()
    .max_by_key(|(_, &elf)| elf)
    .map(|(idx, &value)| (idx, value))
}

pub fn get_packrats(input: String) -> [i32; 3] {   
    let mut elves = input
    .split("\n\n")
    .map(|elf| 
        elf.split("\n")
        .map(|meal| meal.parse::<i32>().expect("Should only have integers on each line"))
        .sum()
    )
    .collect::<Vec<i32>>();
    let mut r: [i32; 3] = Default::default();
    elves.sort();
    elves.reverse();
    r.clone_from_slice(&elves[0..=2]);
    return r;
}