pub fn day1(input: &str) {
    let mut calories: Vec<i32> = input
        .split("\r\n\r\n")
        .map(|elf| elf.lines().map(|food| food.parse::<i32>().unwrap()).sum())
        .collect();

    // Part 1
    println!("{}", calories.iter().max().unwrap());

    // Part 2
    calories.sort();
    println!("{}", calories.iter().rev().take(3).sum::<i32>());
}
