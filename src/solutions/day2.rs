use anyhow::Result;

pub fn day2(input: &str) -> Result<()> {
    let game = [
        [1 + 3, 2 + 6, 3], // A
        [1, 2 + 3, 3 + 6], // B
        [1 + 6, 2, 3 + 3], // C
    ];
    let plays: Vec<_> = input
        .lines()
        .map(|strat| {
            strat
                .split_whitespace()
                .map(|play| match play {
                    "A" | "X" => 0,
                    "B" | "Y" => 1,
                    "C" | "Z" => 2,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let score = plays.iter().map(|play| game[play[0]][play[1]]).sum::<i32>();
    println!("{}", score);

    let strats = [
        [3, 4, 8], // A
        [1, 5, 9], // B
        [2, 6, 7], // C
    ];

    let score2 = plays
        .iter()
        .map(|play| strats[play[0]][play[1]])
        .sum::<i32>();

    println!("{}", score2);
    Ok(())
}
