use std::fs;

use adventofcode_2022::{AnyResult, CustomError};
use indextree::{Arena, Node, NodeId};

enum FileSystemEntity<'a> {
    File(&'a str, usize),
    Directory(&'a str),
}

enum Command<'a> {
    ChangeDir(CdArgument<'a>),
    List,
}

enum CdArgument<'a> {
    DirectoryName(&'a str),
    ParentDirectory,
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = CustomError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "$ ls" => Ok(Command::List),
            "$ cd .." => Ok(Command::ChangeDir(CdArgument::ParentDirectory)),
            other => other
                .rsplit_once(" ")
                .map(|(_, directory_name)| {
                    Command::ChangeDir(CdArgument::DirectoryName(directory_name))
                })
                .ok_or(CustomError {
                    msg: "Tried to parse a malformed command.".into(),
                }),
        }
    }
}

fn construct_file_system<'a, 'b>(
    input: &'a str,
    arena: &'b mut Arena<FileSystemEntity<'a>>,
) -> Result<NodeId, CustomError> {
    let mut lines = input.lines().map(str::trim);

    let Some("$ cd /") = lines.next() else {
        return Err(CustomError { msg: "The first line was malformed.".into()}.into());
    };

    let mut current_node_id = arena.new_node(FileSystemEntity::Directory("/"));

    let root_id = current_node_id;

    for line in lines.map(str::trim) {
        if line.starts_with('$') {
            let command: Command = line.try_into()?;

            if let Command::ChangeDir(cd_argument) = command {
                if let CdArgument::DirectoryName(directory_name) = cd_argument {
                    current_node_id = current_node_id
                        .children(arena)
                        .find_map(|child_id| {
                            arena.get(child_id).and_then(|child| match child.get() {
                                FileSystemEntity::Directory(name) => {
                                    (*name == directory_name).then_some(child_id)
                                }
                                _ => None,
                            })
                        })
                        .ok_or(CustomError {
                            msg: format!("Tried to cd into directory {directory_name}, which we've never seen").into(),
                        })?;
                } else {
                    let parent_id =
                        arena
                            .get(current_node_id)
                            .and_then(Node::parent)
                            .ok_or(CustomError {
                                msg: "Tried to reach a parent we've never seen.".into(),
                            })?;

                    current_node_id = parent_id;
                }
            }
        } else {
            let (descriptor, entity_name) = line.split_once(' ').ok_or(CustomError {
                msg: "Found malformed `ls` output.".into(),
            })?;

            let new_file_system_entity_id = if descriptor == "dir" {
                arena.new_node(FileSystemEntity::Directory(entity_name))
            } else if let Ok(file_size) = descriptor.parse::<usize>() {
                arena.new_node(FileSystemEntity::File(entity_name, file_size))
            } else {
                return Err(CustomError {
                    msg: "Found malformed `ls` descriptor.".into(),
                }
                .into());
            };

            current_node_id.append(new_file_system_entity_id, arena);
        }
    }

    Ok(root_id)
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day7/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> AnyResult {
    let arena = &mut Arena::new();

    let root_id = construct_file_system(input, arena)?;

    let small_directory_size_sum: usize = root_id
        .descendants(arena)
        .filter_map(|node_id| {
            let file_system_entity = arena[node_id].get();

            match file_system_entity {
                FileSystemEntity::File(_, _) => None,
                FileSystemEntity::Directory(_) => Some(node_id),
            }
        })
        .map(|directory_id| {
            directory_id
                .descendants(arena)
                .filter_map(|node_id| {
                    let file_system_entity = arena[node_id].get();

                    match file_system_entity {
                        FileSystemEntity::File(_, size) => Some(*size),
                        FileSystemEntity::Directory(_) => None,
                    }
                })
                .sum::<usize>()
        })
        .filter(|&size| size <= 100_000)
        .sum();

    println!("Part 1 answer = {small_directory_size_sum}");

    Ok(())
}

fn part2(input: &str) -> AnyResult {
    let arena = &mut Arena::new();

    let root_id = construct_file_system(input, arena)?;

    let mut directory_sizes: Vec<usize> = root_id
        .descendants(arena)
        .filter_map(|node_id| {
            let file_system_entity = arena[node_id].get();

            match file_system_entity {
                FileSystemEntity::File(_, _) => None,
                FileSystemEntity::Directory(_) => Some(node_id),
            }
        })
        .map(|directory_id| {
            directory_id
                .descendants(arena)
                .filter_map(|node_id| {
                    let file_system_entity = arena[node_id].get();

                    match file_system_entity {
                        FileSystemEntity::File(_, size) => Some(*size),
                        FileSystemEntity::Directory(_) => None,
                    }
                })
                .sum()
        })
        .collect();

    let total_disk_space = 70_000_000;
    let required_disk_space = 30_000_000;

    let root_directory_size = directory_sizes[0];

    let unused_disk_space = total_disk_space - root_directory_size;

    let space_to_be_freed = required_disk_space - unused_disk_space;

    directory_sizes.sort();

    let smallest_big_directory = directory_sizes
        .into_iter()
        .find(|&size| size > space_to_be_freed)
        .ok_or(CustomError {
            msg: "No directory big enough to make space was found.".into(),
        })?;

    println!("Part 2 answer = {smallest_big_directory}");

    Ok(())
}
