use std::io;

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("src/bin/day1/input.txt")?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &str) {
    let elves = input.split("\n\n");

    let max_calories = elves
        .map(|elf| elf.split('\n').flat_map(str::parse::<u32>).sum::<u32>())
        .max();

    if let Some(calories) = max_calories {
        println!("Part 1 answer = {calories}",);
    }
}

fn part2(input: &str) {
    let elves = input.split("\n\n");

    let maxima = elves
        .map(|elf| elf.split('\n').flat_map(str::parse::<u32>).sum::<u32>())
        .fold([0, 0, 0], |mut maxima, calories| {
            if calories > maxima[2] {
                maxima.swap(0, 1);
                maxima.swap(1, 2);
                maxima[2] = calories;
            } else if calories > maxima[1] {
                maxima.swap(0, 1);
                maxima[1] = calories;
            } else if calories > maxima[0] {
                maxima[0] = calories;
            }

            maxima
        });

    let top_three_calorie_sum: u32 = maxima.into_iter().sum();

    println!("Part 2 answer = {top_three_calorie_sum}");
}
