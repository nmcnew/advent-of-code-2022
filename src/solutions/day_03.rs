use std::collections::HashSet;
use std::collections::HashMap;

pub fn run_part1(input: &String) -> i32{
    let priority_map = get_priority_map();
    input
    .lines()
    .map(|backpack| {
        let backpack_size = backpack.len();
        let compartment1 = &backpack[0..backpack_size/2];
        let compartment2 = &backpack[backpack.len()/2..backpack.len()];
        let mut items: HashMap<char, i32> = HashMap::new();

        compartment1.chars().for_each(|character| {
            items
            .entry(character)
            .or_insert(1);
        });
        compartment2.chars().for_each(|character| {
            items
            .entry(character)
            .and_modify(|value| *value += 1);
        });
        items
    })
    .collect::<Vec<HashMap<char, i32>>>()
    .iter()
    .flat_map(|backpack_values| {
        backpack_values.iter()
        .map(|(key, value)| {
            if *value > 1 {
                return *priority_map.get(key).expect("unmapped key value");
            }
            else {
                return 0;
            }
        })
        .collect::<Vec<i32>>()
    })
    .collect::<Vec<i32>>()
    .iter()
    .sum::<i32>()
}

pub fn run_part2(input: &String) -> i32 {
    let priority_map = get_priority_map();
    input
    .lines()
    .enumerate()
    .map(|(iter, line)| ((iter as f32/3.0).floor() as i32, line.chars().collect::<HashSet<char>>()))
    .fold(HashMap::new(), 
        |mut acc:HashMap<i32, HashSet<char>>, item| {
        acc.entry(item.0)
        .and_modify(|result| { *result = result.intersection(&item.1).copied().collect(); })
        .or_insert(item.1);
        acc
        }
    )
    .iter()
    .map(|(_, badge)| priority_map.get(badge.iter().next().unwrap()).unwrap())
    .sum()
}
fn get_priority_map() -> HashMap<char, i32> {
    return HashMap::from([
    ('a', 1),
    ('b', 2),
    ('c', 3),
    ('d', 4),
    ('e', 5),
    ('f', 6),
    ('g', 7),
    ('h', 8),
    ('i', 9),
    ('j', 10),
    ('k', 11),
    ('l', 12),
    ('m', 13),
    ('n', 14),
    ('o', 15),
    ('p', 16),
    ('q', 17),
    ('r', 18),
    ('s', 19),
    ('t', 20),
    ('u', 21),
    ('v', 22),
    ('w', 23),
    ('x', 24),
    ('y', 25),
    ('z', 26),
    ('A', 27),
    ('B', 28),
    ('C', 29),
    ('D', 30),
    ('E', 31),
    ('F', 32),
    ('G', 33),
    ('H', 34),
    ('I', 35),
    ('J', 36),
    ('K', 37),
    ('L', 38),
    ('M', 39),
    ('N', 40),
    ('O', 41),
    ('P', 42),
    ('Q', 43),
    ('R', 44),
    ('S', 45),
    ('T', 46),
    ('U', 47),
    ('V', 48),
    ('W', 49),
    ('X', 50),
    ('Y', 51),
    ('Z', 52)]);
}