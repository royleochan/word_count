use std::error::Error;
use std::fs::read_to_string;
use std::io::{self, Read};

#[derive(Debug)]
enum Flag {
    Default,
    Size,
    Line,
    Word,
    Char,
}

impl Flag {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "-l" => Some(Flag::Line),
            "-c" => Some(Flag::Size),
            "-w" => Some(Flag::Word),
            "-m" => Some(Flag::Char),
            "" => Some(Flag::Default),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    file_path: String,
    command: Flag,
    is_stdin: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        if args.len() == 2 {
            let argument = args[1].clone();
            let potential_flag = Flag::from_str(&argument);

            if let Some(flag) = potential_flag {
                return Ok(Config {
                    file_path: "".to_string(),
                    command: flag,
                    is_stdin: true,
                });
            } else {
                return Ok(Config {
                    file_path: argument,
                    command: Flag::from_str("").unwrap(),
                    is_stdin: false,
                });
            }
        }

        if args.len() == 3 {
            let flag = args[1].clone();
            let file_path = args[2].clone();
            let command_flag = Flag::from_str(&flag);
            match command_flag {
                Some(command_flag) => {
                    return Ok(Config {
                        file_path,
                        command: command_flag,
                        is_stdin: false,
                    })
                }
                None => return Err("Flag not recognized"),
            }
        } else {
            return Err("Too many arguments");
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_name = config.file_path.clone();
    let mut contents: String = String::new();
    if config.is_stdin {
        io::stdin().read_to_string(&mut contents)?;
    } else {
        contents = read_to_string(config.file_path)?;
    }

    let size = get_size_in_bytes(&contents);
    let num_lines = get_num_lines(&contents);
    let num_words = get_num_words(&contents);
    let num_chars = get_num_char(&contents);
    match config.command {
        Flag::Size => println!("{size} {file_name}"),
        Flag::Line => println!("{num_lines} {file_name}"),
        Flag::Word => println!("{num_words} {file_name}"),
        Flag::Char => println!("{num_chars} {file_name}"),
        Flag::Default => println!("{num_lines}  {num_words} {size} {file_name}"),
    }
    Ok(())
}

pub fn get_size_in_bytes(contents: &str) -> usize {
    return contents.len();
}

pub fn get_num_lines(contents: &str) -> usize {
    return contents.lines().count();
}

pub fn get_num_words(contents: &str) -> usize {
    return contents.split_ascii_whitespace().count();
}

pub fn get_num_char(contents: &str) -> usize {
    return contents.chars().count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_size_empty() {
        let contents = "";

        assert_eq!(0, get_size_in_bytes(contents));
    }

    #[test]
    fn test_get_size() {
        let contents = "test";

        assert_eq!(4, get_size_in_bytes(contents));
    }

    #[test]
    fn test_get_num_lines() {
        let contents = "\test
        lorem
        ipsum";

        assert_eq!(3, get_num_lines(contents));
    }

    #[test]
    fn test_get_num_words() {
        let contents = "lorem ipsum dolor";

        assert_eq!(3, get_num_words(contents));
    }
}
