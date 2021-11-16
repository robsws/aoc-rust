use json::JsonValue;
use crate::input_file::read_all_to_string;

pub fn part1(input_file_path: &str) {
    let s = read_all_to_string(input_file_path);
    let j = json::parse(&s).expect("Json failed to parse.");
    let n = count_numbers(&j, false);
    println!("{}", n);
}

pub fn part2(input_file_path: &str) {
    let s = read_all_to_string(input_file_path);
    let j = json::parse(&s).expect("Json failed to parse.");
    let n = count_numbers(&j, true);
    println!("{}", n);
}

fn count_numbers(j: &json::JsonValue, ignore_red: bool) -> i32 {
    let mut total: i32 = 0;
    match j {
        JsonValue::Number(n) => {
            let float_num: f64 = (*n).into();
            total += float_num as i32;
        }
        JsonValue::Array(_) => {
            for e in j.members() {
                total += count_numbers(e, ignore_red);
            }
        },
        JsonValue::Object(_) => {
            for (_, v) in j.entries() {
                if ignore_red {
                    match v {
                        JsonValue::Short(s) => {
                            if s.eq("red") {
                                // Whole object is ignored
                                return 0;
                            }
                        },
                        _ => ()
                    }
                }
                total += count_numbers(v, ignore_red);
            }
        },
        _ => ()
    };
    total
}