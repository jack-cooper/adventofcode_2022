use std::{fs, iter::Peekable, str::FromStr};

use adventofcode_2022::{AnyResult, CustomError};

#[derive(Debug)]
enum PacketData {
    Int(u32),
    List(Vec<PacketData>),
}

impl PacketData {
    fn parse_recursive(
        chars: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Result<Self, CustomError> {
        chars
            .next()
            .and_then(|char| match char {
                '[' => {
                    let mut packet_data = Vec::new();

                    while let Ok(packet_datum) = Self::parse_recursive(chars) {
                        packet_data.push(packet_datum);

                        if let Some(']') = chars.next() {
                            break;
                        }
                    }

                    Some(PacketData::List(packet_data))
                }

                digit if digit.is_ascii_digit() => {
                    if digit == '1' && chars.peek() == Some(&'0') {
                        chars.next();
                        Some(Self::Int(10))
                    } else {
                        digit.to_digit(10).map(Self::Int)
                    }
                }

                _ => None,
            })
            .ok_or(CustomError {
                msg: "Malformed packet data detected.".into(),
            })
    }
}

impl FromStr for PacketData {
    type Err = CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_recursive(&mut s.chars().peekable())
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(int1), Self::Int(int2)) => int1 == int2,
            (&Self::Int(int), Self::List(list)) | (Self::List(list), &Self::Int(int)) => {
                list == &vec![Self::Int(int)]
            }
            (Self::List(list1), Self::List(list2)) => list1 == list2,
        }
    }
}

impl Eq for PacketData {}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketData::Int(int1), PacketData::Int(int2)) => int1.cmp(int2),
            (&PacketData::Int(int), PacketData::List(list)) => vec![Self::Int(int)].cmp(list),
            (PacketData::List(list), &PacketData::Int(int)) => list.cmp(&vec![Self::Int(int)]),
            (PacketData::List(list1), PacketData::List(list2)) => list1.cmp(list2),
        }
    }
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day13/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> AnyResult {
    let mut correct_index_sum = 0;

    for (index, packet_pair) in input.split("\n\n").enumerate() {
        let (packet1, packet2) = packet_pair.split_once('\n').ok_or(CustomError {
            msg: "Found a packet pair not separated by a newline.".into(),
        })?;

        let (packet1, packet2): (PacketData, PacketData) = (packet1.parse()?, packet2.parse()?);

        if packet1 < packet2 {
            correct_index_sum += index + 1;
        }
    }

    println!("Part 1 answer = {correct_index_sum}");

    Ok(())
}

fn part2(input: &str) -> AnyResult {
    let packets: Result<Vec<PacketData>, CustomError> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|str| str.parse())
        .collect();

    let mut packets = packets?;

    packets.push(PacketData::List(vec![PacketData::List(vec![
        PacketData::Int(2),
    ])]));
    packets.push(PacketData::List(vec![PacketData::List(vec![
        PacketData::Int(6),
    ])]));

    packets.sort();

    let start_index = packets
        .binary_search(&PacketData::List(vec![PacketData::List(vec![
            PacketData::Int(2),
        ])]))
        .unwrap()
        + 1;

    let end_index = packets
        .binary_search(&PacketData::List(vec![PacketData::List(vec![
            PacketData::Int(6),
        ])]))
        .unwrap()
        + 1;

    let index_sum = start_index * end_index;

    println!("Part 2 answer = {index_sum}");

    Ok(())
}
