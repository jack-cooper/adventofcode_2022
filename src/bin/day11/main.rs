use std::{cell::RefCell, collections::VecDeque, fs, rc::Rc, str::FromStr};

use adventofcode_2022::{AnyResult, CustomError};

type RcCell<T> = Rc<RefCell<T>>;

#[derive(Clone)]
struct Monkey {
    inspection_count: usize,
    items: VecDeque<Item>,
    operation: RcCell<dyn FnMut(&mut Item)>,
    test: Rc<dyn Fn(&Item) -> usize>,
}

impl Monkey {
    fn increase_inspection_count(&mut self) {
        self.inspection_count += self.items.len();
    }
}

impl FromStr for Monkey {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let notes: Vec<&str> = s.lines().map(str::trim).collect();

        let (title, index) = notes[0].split_once(' ').ok_or(CustomError {
            msg: "Received a monkey title without a space.".into(),
        })?;
        let is_title_correct = title == "Monkey";

        let is_index_correct =
            index.ends_with(':') && index.replace(':', "").parse::<usize>().is_ok();

        if !(is_title_correct && is_index_correct) {
            return Err(CustomError {
                msg: "Received a monkey with a malformed title.".into(),
            });
        }

        let (descriptor, items) = notes[1].split_once(": ").ok_or(CustomError {
            msg: "Received a monkey item descriptor without `: `.".into(),
        })?;

        if descriptor != "Starting items" {
            return Err(CustomError {
                msg: "Received a malformed monkey item descriptor.".into(),
            });
        }

        let items: VecDeque<Item> = items
            .split(", ")
            .map(|worry_level| {
                worry_level
                    .parse()
                    .map(|worry_level| Item { worry_level })
                    .map_err(|_| CustomError {
                        msg: "Received a non-numeric worry level.".into(),
                    })
            })
            .collect::<Result<_, _>>()?;

        let (descriptor, operation) = notes[2].split_once(": ").ok_or(CustomError {
            msg: "Received a monkey operation descriptor without `: `.".into(),
        })?;

        if descriptor != "Operation" {
            return Err(CustomError {
                msg: "Received a malformed monkey operation descriptor.".into(),
            });
        }

        let operation_components: Vec<_> = operation.split(' ').collect();

        if operation_components.len() != 5 {
            return Err(CustomError {
                msg: "Received an operation with an incorrect number of components.".into(),
            });
        }

        if operation_components[..3] != ["new", "=", "old"] {
            return Err(CustomError {
                msg: "Received an operation with a malformed prefix.".into(),
            });
        }

        let operation_function = if operation_components[3] == "+" {
            <u64 as std::ops::Add>::add
        } else if operation_components[3] == "*" {
            if operation_components[4] == "old" {
                |value, exp| u64::pow(value, exp as u32)
            } else {
                <u64 as std::ops::Mul>::mul
            }
        } else {
            return Err(CustomError {
                msg: "Received an operation with a malformed operator.".into(),
            });
        };

        let operation_operand = if operation_components[4] == "old" {
            2
        } else {
            operation_components[4].parse().map_err(|_| CustomError {
                msg: "Received an operation with a non-numeric operand.".into(),
            })?
        };

        let (descriptor, divisor) = notes[3].rsplit_once(' ').ok_or(CustomError {
            msg: "Received a test condition without a space.".into(),
        })?;

        if descriptor != "Test: divisible by" {
            return Err(CustomError {
                msg: "Received a malformed test descriptor.".into(),
            });
        }

        let divisor: u64 = divisor.parse().map_err(|_| CustomError {
            msg: "Received a test with a non-numeric divisor.".into(),
        })?;

        let (descriptor, true_monkey_index) = notes[4].rsplit_once(' ').ok_or(CustomError {
            msg: "Received a true monkey without a space.".into(),
        })?;

        if descriptor != "If true: throw to monkey" {
            return Err(CustomError {
                msg: "Received a malformed true monkey descriptor.".into(),
            });
        }

        let true_monkey_index: usize = true_monkey_index.parse().map_err(|_| CustomError {
            msg: "Received a true monkey with a non-numeric index.".into(),
        })?;

        let (descriptor, false_monkey_index) = notes[5].rsplit_once(' ').ok_or(CustomError {
            msg: "Received a false monkey without a space.".into(),
        })?;

        if descriptor != "If false: throw to monkey" {
            return Err(CustomError {
                msg: "Received a malformed false monkey descriptor.".into(),
            });
        }

        let false_monkey_index: usize = false_monkey_index.parse().map_err(|_| CustomError {
            msg: "Received a false monkey with a non-numeric index.".into(),
        })?;

        Ok(Monkey {
            inspection_count: 0,
            items,
            operation: Rc::new(RefCell::new(move |item: &mut Item| {
                item.worry_level = operation_function(item.worry_level, operation_operand);
            })),
            test: Rc::new(move |item: &Item| {
                if item.worry_level % divisor == 0 {
                    true_monkey_index
                } else {
                    false_monkey_index
                }
            }),
        })
    }
}

#[derive(Clone)]
struct Item {
    worry_level: u64,
}

impl Item {
    const LCM: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;

    fn apply_relief(&mut self) {
        self.worry_level /= 3;
    }

    fn manage_worry_level(&mut self) {
        self.worry_level %= Self::LCM;
    }
}

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
