macro_rules! hashmap {
    ($($key: expr => $val: expr), *) => {{
        let mut hashmap = std::collections::HashMap::new();
        $(
            hashmap.insert(String::from($key), String::from($val));
        )*
        hashmap
    }}
}

fn main() {
    let instance = construct();
    let result = instance.clone().calculate();
    let recalculate = || {
        let alphabet_to_number = |numbers: &Vec<String>| {
            const ALPHABET: [&str; 26] = [
                "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
                "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
            ];
            let alphabet_convert_table = hashmap!["A"=> "10", "B"=> "11", "C"=> "12", "D"=> "13", "E"=> "14", "F"=> "15", "G"=> "16", "H"=> "17", "I"=> "18", "J"=> "19", "K"=> "20", "L"=> "21", "M"=> "22", "N"=> "23", "O"=> "24", "P"=> "25", "Q"=> "26", "R"=> "27", "S"=> "28", "T"=> "29", "U"=> "30", "V"=> "31", "W"=> "32", "X"=> "33", "Y"=> "34", "Z"=> "35"];
            let mut number = Vec::<String>::new();
            'first: for character in numbers {
                for alphabet in ALPHABET {
                    if character == alphabet {
                        number.push(alphabet_convert_table[&character.to_string()].clone());
                        continue 'first;
                    } else {
                        continue;
                    }
                }
                number.push(character.to_string());
            }
            number
        };
        let number_to_alphabet = |numbers: Vec<String>| {
            let mut output = Vec::<String>::new();
            let alphabet_convert_table = hashmap!["10"=> "A", "11"=> "B", "12"=> "C", "13"=> "D", "14"=> "E", "15"=> "F", "16"=> "G", "17"=> "H", "18"=> "I", "19"=> "J", "20"=> "K", "21"=> "L", "22"=> "M", "23"=> "N", "24"=> "O", "25"=> "P", "26"=> "Q", "27"=> "R", "28"=> "S", "29"=> "T", "30"=> "U", "31"=> "V", "32"=> "W", "33"=> "X", "34"=> "Y", "35"=> "Z"];
            'first: for number in numbers {
                for (figure, alphabet) in alphabet_convert_table.clone() {
                    if number == figure {
                        output.push(alphabet);
                        continue 'first;
                    } else {
                        ()
                    }
                }
                output.push(number);
            }
            return output;
        };
        let confirmation = Number {
            number: alphabet_to_number(&result),
            from: instance.to,
            to: instance.from,
            minus: instance.minus,
            is_integer: instance.is_integer,
        }
        .calculate();
        let number = number_to_alphabet(instance.number);
        if number == confirmation {
            return true;
        } else {
            return false;
        }
    };
    if recalculate() {
        println!("{}\n(Accuracy verified)", finalize(result));
    } else {
        println!("This conversion may be inaccurate. Do you still want to output? ('y' or 'n') >");
        match input_y_or_n().as_str() {
            "y" => println!("{}\n(Possibly inaccurate)", finalize(result)),
            "n" => println!("Canceled. If you don't mind, could you please bring this issue to the attention of the developer?"),
            "error" => error(3),
            _ => (),
        }
    }
}

#[derive(Clone)]
struct Number {
    number: Vec<String>,
    from: u8,
    to: u8,
    minus: bool,
    is_integer: bool,
}

fn construct() -> Number {
    let condition_checker = |arguments: &Vec<String>| {
        const OK: [char; 38] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '-', 'A', 'B', 'C', 'D', 'E',
            'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
            'W', 'X', 'Y', 'Z',
        ];
        if arguments.len() == 4 {
            'first: for character in arguments[1].chars() {
                for ok in OK {
                    if character == ok {
                        continue 'first;
                    } else {
                        continue;
                    }
                }
                return false;
            }
            return true;
        } else {
            return false;
        }
    };
    let alphabet_to_number = |arguments: &Vec<String>| {
        const ALPHABET: [char; 26] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let alphabet_convert_table = hashmap!["A"=> "10", "B"=> "11", "C"=> "12", "D"=> "13", "E"=> "14", "F"=> "15", "G"=> "16", "H"=> "17", "I"=> "18", "J"=> "19", "K"=> "20", "L"=> "21", "M"=> "22", "N"=> "23", "O"=> "24", "P"=> "25", "Q"=> "26", "R"=> "27", "S"=> "28", "T"=> "29", "U"=> "30", "V"=> "31", "W"=> "32", "X"=> "33", "Y"=> "34", "Z"=> "35"];
        let mut number = Vec::<String>::new();
        'first: for character in arguments[1].chars() {
            for alphabet in ALPHABET {
                if character == alphabet {
                    number.push(alphabet_convert_table[&character.to_string()].clone());
                    continue 'first;
                } else {
                    continue;
                }
            }
            number.push(character.to_string());
        }
        number
    };
    let range_checker = |arguments: &Vec<String>| {
        let from = arguments[2].parse::<u8>().unwrap();
        let to = arguments[3].parse::<u8>().unwrap();
        let judge = |from_or_to: u8| {
            if from_or_to < 2 || 36 < from_or_to {
                return false;
            } else {
                return true;
            }
        };
        if judge(from) || judge(to) {
            return (from, to);
        } else {
            return (0, 0);
        }
    };
    let minus_checker = |number: Vec<String>| {
        let mut numbers = number;
        if numbers[0] == "-" {
            numbers.remove(0);
            return (numbers, true);
        } else {
            return (numbers, false);
        }
    };
    let is_integer = |arguments: &Vec<String>| {
        for character in arguments[1].chars() {
            if character == '.' {
                return false;
            } else {
                continue;
            }
        }
        return true;
    };
    let number_checker = |number: &Vec<String>, from: u8, is_integer: bool| {
        let mut numbers = number.clone();
        if is_integer {
            ()
        } else {
            let searcher = || {
                for (index, number) in numbers.iter().enumerate() {
                    if number == "." {
                        return index;
                    } else {
                        continue;
                    }
                }
                return 0;
            };
            numbers.remove(searcher());
        }
        for number in numbers {
            if number.parse::<u8>().unwrap() >= from {
                return false;
            } else {
                continue;
            }
        }
        return true;
    };
    let arguments = std::env::args().collect::<Vec<String>>();
    if condition_checker(&arguments) {
        let number = alphabet_to_number(&arguments);
        let (from, to) = range_checker(&arguments);
        if from != 0 {
            let (number, minus) = minus_checker(number);
            let integer = is_integer(&arguments);
            if number_checker(&number, from, integer) {
                return Number {
                    number,
                    from,
                    to,
                    minus,
                    is_integer: integer,
                };
            } else {
                error(2);
            }
        } else {
            error(1);
        }
    } else {
        error(0);
    }
    return Number {
        number: Vec::<String>::new(),
        from: 0,
        to: 0,
        minus: false,
        is_integer: false,
    };
}

impl Number {
    fn calculate(self) -> Vec<String> {
        let convert_to_dec_int = || {
            let convert_to_u128 = || {
                let mut numbers = Vec::<u128>::new();
                for number in &self.number {
                    numbers.push(number.parse::<u128>().unwrap());
                }
                return numbers;
            };
            let mut length = self.number.len() as u32;
            let numbers = convert_to_u128();
            let mut new_number = 0;
            for number in numbers {
                let converted_number = (self.from as u128).pow(length - 1) * number;
                new_number = converted_number + new_number;
                length -= 1;
            }
            return new_number;
        };
        let convert_to_dec_float = || {
            let convert_to_f64 = || {
                let searcher = || {
                    for (index, number) in self.number.iter().enumerate() {
                        if number == "." {
                            return index;
                        } else {
                            continue;
                        }
                    }
                    return 0;
                };
                let dot_index = searcher();
                let mut numbers = self.number.clone();
                numbers.remove(dot_index);
                let mut converted_number = Vec::<f64>::new();
                for number in numbers {
                    converted_number.push(number.parse::<f64>().unwrap());
                }
                return (converted_number, dot_index as i32);
            };
            let (numbers, mut length) = convert_to_f64();
            let mut new_number = 0.0;
            for number in numbers {
                let converted_number = (self.from as f64).powi(length - 1) * number;
                new_number = converted_number + new_number;
                length -= 1;
            }
            return new_number;
        };
        let number_to_alphabet = |number: u128| {
            let number = number.to_string();
            let alphabet_convert_table = hashmap!["10"=> "A", "11"=> "B", "12"=> "C", "13"=> "D", "14"=> "E", "15"=> "F", "16"=> "G", "17"=> "H", "18"=> "I", "19"=> "J", "20"=> "K", "21"=> "L", "22"=> "M", "23"=> "N", "24"=> "O", "25"=> "P", "26"=> "Q", "27"=> "R", "28"=> "S", "29"=> "T", "30"=> "U", "31"=> "V", "32"=> "W", "33"=> "X", "34"=> "Y", "35"=> "Z"];
            for (figure, alphabet) in alphabet_convert_table {
                if number == figure {
                    return alphabet;
                } else {
                    continue;
                }
            }
            return "!".to_string();
        };
        let int_calculate = |number| {
            let mut number = number;
            let mut output = Vec::<String>::new();
            while number >= self.to as u128 {
                let division = number / self.to as u128;
                let remainder = number % self.to as u128;
                if remainder >= 10 {
                    let remainder = number_to_alphabet(remainder);
                    output.push(remainder);
                    number = division;
                    continue;
                } else {
                    output.push(remainder.to_string());
                    number = division;
                    continue;
                }
            }
            if number >= 10 {
                let number = number_to_alphabet(number);
                output.push(number);
            } else {
                output.push(number.to_string());
            }
            if self.minus {
                output.push("-".to_string());
            } else {
                ()
            }
            output.reverse();
            output
        };
        let float_calculate = |number: f64| {
            let mut output = Vec::<String>::new();
            let mut integer = number.floor();
            if integer > number {
                integer -= 1.0;
            } else {
                ()
            }
            let mut float = number - integer;
            let integer = integer as u128;
            if integer >= 1 {
                output = int_calculate(integer);
                output.push(".".to_string());
            } else {
                output.push("0".to_string());
                output.push(".".to_string());
            }
            while float != 0.0 {
                let decimal = float * self.to as f64;
                let mut integer = decimal.floor();
                if integer > decimal {
                    integer -= 1.0;
                } else {
                    ()
                }
                if integer >= 10.0 {
                    let integer = number_to_alphabet(integer as u128);
                    output.push(integer);
                } else {
                    output.push((integer as u128).to_string());
                }
                float = decimal - integer;
            }
            if output[output.len() - 1] == ".".to_string() {
                output.pop();
            } else {
                ()
            }
            return output;
        };
        if self.is_integer {
            let number = int_calculate(convert_to_dec_int());
            return number;
        } else {
            let number = float_calculate(convert_to_dec_float());
            return number;
        }
    }
}

fn finalize(vector: Vec<String>) -> String {
    let mut string = String::new();
    for str in vector {
        string += str.as_str();
    }
    return string;
}

fn input_y_or_n() -> String {
    let mut y_or_n = String::new();
    std::io::stdin().read_line(&mut y_or_n).unwrap();
    y_or_n = y_or_n.replace("\r\n", "");
    if y_or_n == "y".to_string() || y_or_n == "n".to_string() {
        return y_or_n;
    } else {
        return "error".to_string();
    }
}

fn error(errorcode: u8) {
    match errorcode {
        0 => println!("The string entered contains characters that cannot be used."),
        1 => println!("The second or third argument is invalid."),
        2 => println!("The first argument is invalid."),
        3 => println!("A character other than 'y' or 'n' was entered."),
        _ => (),
    }
    std::process::exit(0);
}
