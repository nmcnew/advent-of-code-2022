use std::collections::HashMap;

pub fn get_score(input: &String) -> i32 {
    let mut resolution_chart: HashMap<String, i32> = HashMap::new();
    resolution_chart.insert(String::from("A X"), 3+1);
    resolution_chart.insert(String::from("A Y"), 6+2);
    resolution_chart.insert(String::from("A Z"), 0+3);
    resolution_chart.insert(String::from("B Z"), 6+3);
    resolution_chart.insert(String::from("B Y"), 3+2);
    resolution_chart.insert(String::from("B X"), 0+1);
    resolution_chart.insert(String::from("C X"), 6+1);
    resolution_chart.insert(String::from("C Y"), 0+2);
    resolution_chart.insert(String::from("C Z"), 3+3);
    input.split("\n")
    .map(|round| match resolution_chart.get(round){
        Some(value) => value,
        None => &0,
    })
    .sum::<i32>()
}
struct Hand {
    ref_value: char,
    value: i32,
    beats: char,
    loses: char,
}
pub fn get_score_result(input: &String) -> i32 {
    // A = Rock, B = Paper, C = Scissors
    // X = Lose, Y = Draw, Z = Win
    //EX: 
    /*
        A Y -> opponent picks Rock and I need to draw, so pick rock for a score of 1+3 = 4
        B X -> opponent picks paper and I need to lose, so pick rock for a score  1+0= 1
        C Z -> opponent picks scissors and I need to win so I pick rock, for a score of 1+6 = 7
        For a total score of 12
     */
    let rock = Hand {
        ref_value: 'A',
        value: 1,
        beats: 'C',
        loses: 'B',
    };
    let paper = Hand {
        ref_value: 'B',
        value: 2,
        beats: 'A',
        loses: 'C',
    };
    let scissors = Hand {
        ref_value: 'C',
        value: 3,
        beats: 'B',
        loses: 'A',
    };
    let mut hands: HashMap<char, Hand> = HashMap::new();

    hands.insert(rock.ref_value, rock);
    hands.insert(paper.ref_value, paper);
    hands.insert(scissors.ref_value, scissors);

    input.split("\n").map(|round| {
        let opponent_hand = hands.get(&round.chars().nth(0).expect("Malformed round")).expect("Round contains hand that doesn't exist");
        let resolution = round.chars().nth(2).expect("Malformed Round");
        let round_result = match resolution {
            'X' => hands.get(&opponent_hand.beats).expect("HashMap was not setup correctly").value ,
            'Y' => opponent_hand.value + 3,
            'Z' => hands.get(&opponent_hand.loses).expect("HashMap was not setup correctly").value + 6,
            _ => panic!("Unknown resolution value"),
        };
        return round_result;
    })
    .sum::<i32>()
}