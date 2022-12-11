use std::collections::VecDeque;
use std::str::FromStr;
use std::{cell::RefCell, rc::Rc};

use adventofcode_2022::CustomError;

type RcCell<T> = Rc<RefCell<T>>;

type Operation = RcCell<dyn FnMut(&mut Item)>;
type Test = Rc<dyn Fn(&Item) -> usize>;

#[derive(Clone)]
pub struct Monkey {
    pub inspection_count: usize,
    pub items: VecDeque<Item>,
    pub operation: Operation,
    pub test: Test,
}

#[derive(Clone)]
pub struct Item {
    worry_level: u64,
}

impl Monkey {
    pub fn increase_inspection_count(&mut self) {
        self.inspection_count += self.items.len();
    }

    fn validate_operation(operation: &str) -> Result<Operation, CustomError> {
        let (descriptor, operation) = operation.split_once(": ").ok_or(CustomError {
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

        let operation = Rc::new(RefCell::new(move |item: &mut Item| {
            item.worry_level = operation_function(item.worry_level, operation_operand);
        }));

        Ok(operation)
    }

    fn validate_receiver_monkey(
        receiver_monkey: &str,
        monkey_type: bool,
    ) -> Result<usize, CustomError> {
        let monkey_type = monkey_type.to_string();

        let (descriptor, monkey_index) = receiver_monkey.rsplit_once(' ').ok_or(CustomError {
            msg: format!("Received a {monkey_type} monkey without a space.").into(),
        })?;

        if descriptor != format!("If {monkey_type}: throw to monkey").as_str() {
            return Err(CustomError {
                msg: format!("Received a malformed {monkey_type} monkey descriptor.").into(),
            });
        }

        monkey_index.parse().map_err(|_| CustomError {
            msg: format!("Received a {monkey_type} monkey with a non-numeric index.").into(),
        })
    }

    fn validate_starting_items(items: &str) -> Result<VecDeque<Item>, CustomError> {
        let (descriptor, items) = items.split_once(": ").ok_or(CustomError {
            msg: "Received a monkey item descriptor without `: `.".into(),
        })?;

        if descriptor != "Starting items" {
            return Err(CustomError {
                msg: "Received a malformed monkey item descriptor.".into(),
            });
        }

        items
            .split(", ")
            .map(|worry_level| {
                worry_level
                    .parse()
                    .map(|worry_level| Item { worry_level })
                    .map_err(|_| CustomError {
                        msg: "Received a non-numeric worry level.".into(),
                    })
            })
            .collect::<Result<_, _>>()
    }

    fn validate_test_condition(test_condition: &str) -> Result<u64, CustomError> {
        let (descriptor, divisor) = test_condition.rsplit_once(' ').ok_or(CustomError {
            msg: "Received a test condition without a space.".into(),
        })?;

        if descriptor != "Test: divisible by" {
            return Err(CustomError {
                msg: "Received a malformed test descriptor.".into(),
            });
        }

        divisor.parse().map_err(|_| CustomError {
            msg: "Received a test with a non-numeric divisor.".into(),
        })
    }

    fn validate_title(title: &str) -> Result<(), CustomError> {
        let (title, index) = title.split_once(' ').ok_or(CustomError {
            msg: "Received a monkey title without a space.".into(),
        })?;

        let is_title_correct = title == "Monkey";

        let is_index_correct =
            index.ends_with(':') && index.replace(':', "").parse::<usize>().is_ok();

        if is_title_correct && is_index_correct {
            Ok(())
        } else {
            Err(CustomError {
                msg: "Received a monkey with a malformed title.".into(),
            })
        }
    }
}

impl FromStr for Monkey {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let notes: Vec<&str> = s.lines().map(str::trim).collect();

        Self::validate_title(notes[0])?;

        let items = Self::validate_starting_items(notes[1])?;
        let operation = Self::validate_operation(notes[2])?;
        let test_condition_divisor = Self::validate_test_condition(notes[3])?;

        let true_monkey_index = Self::validate_receiver_monkey(notes[4], true)?;
        let false_monkey_index = Self::validate_receiver_monkey(notes[5], false)?;

        let test = Rc::new(move |item: &Item| {
            if item.worry_level % test_condition_divisor == 0 {
                true_monkey_index
            } else {
                false_monkey_index
            }
        });

        Ok(Monkey {
            inspection_count: 0,
            items,
            operation,
            test,
        })
    }
}

impl Item {
    const LCM: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;

    pub fn apply_relief(&mut self) {
        self.worry_level /= 3;
    }

    pub fn manage_worry_level(&mut self) {
        self.worry_level %= Self::LCM;
    }
}
