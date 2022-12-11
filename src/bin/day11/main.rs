mod monkey;

use std::fs;

use adventofcode_2022::AnyResult;
use monkey::Monkey;

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day11/input.txt")?;

    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(str::parse::<Monkey>)
        .collect::<Result<_, _>>()?;

    part1(monkeys.clone())?;
    part2(monkeys)?;

    Ok(())
}

fn part1(mut monkeys: Vec<Monkey>) -> AnyResult {
    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let thrower_monkey = unsafe { monkeys.as_mut_ptr().add(index).as_mut().unwrap() };

            thrower_monkey.increase_inspection_count();

            for mut item in thrower_monkey.items.drain(..) {
                let mut operation = thrower_monkey.operation.borrow_mut();

                operation(&mut item);

                item.apply_relief();

                let test = &thrower_monkey.test;
                let receiver_monkey_index = test(&item);

                let receiver_monkey = unsafe {
                    monkeys
                        .as_mut_ptr()
                        .add(receiver_monkey_index)
                        .as_mut()
                        .unwrap()
                };

                receiver_monkey.items.push_back(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| std::cmp::Reverse(monkey.inspection_count));

    let monkey_business: usize = monkeys[..2]
        .iter()
        .map(|monkey| monkey.inspection_count)
        .product();

    println!("Part 1 answer = {monkey_business}");

    Ok(())
}

fn part2(mut monkeys: Vec<Monkey>) -> AnyResult {
    for _ in 0..10_000 {
        for index in 0..monkeys.len() {
            let thrower_monkey = unsafe { monkeys.as_mut_ptr().add(index).as_mut().unwrap() };

            thrower_monkey.increase_inspection_count();

            for mut item in thrower_monkey.items.drain(..) {
                let mut operation = thrower_monkey.operation.borrow_mut();

                operation(&mut item);

                item.manage_worry_level();

                let test = &thrower_monkey.test;
                let receiver_monkey_index = test(&item);

                let receiver_monkey = unsafe {
                    monkeys
                        .as_mut_ptr()
                        .add(receiver_monkey_index)
                        .as_mut()
                        .unwrap()
                };

                receiver_monkey.items.push_back(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| std::cmp::Reverse(monkey.inspection_count));

    let monkey_business: usize = monkeys[..2]
        .iter()
        .map(|monkey| monkey.inspection_count)
        .product();

    println!("Part 1 answer = {monkey_business}");

    Ok(())
}
