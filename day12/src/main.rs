extern crate rustc_serialize;

use rustc_serialize::json::Json;

use std::io;
use std::io::prelude::*;

use std::collections::btree_map::Values;

fn parse_json(acc: i64, json_obj: &Json) -> i64 {
    match json_obj {
        &Json::U64(num) => acc + num as i64,
        &Json::I64(num) => acc + num,
        &Json::F64(num) => acc + num as i64,
        &Json::Array(ref arr) => acc + arr.iter().fold(0_i64, parse_json),
        &Json::Object(ref obj) => acc + obj.values().fold(0_i64, parse_json),
        _ => acc,
    }
}

fn contains_red(vals: Values<String, Json>) -> bool {
    let red = "red".to_string();

    for v in vals {
        if let &Json::String(ref s) = v {
            if s == &red {
                return true;
            }
        }
    }

    false
}

fn parse_json_no_red(acc: i64, json_obj: &Json) -> i64 {
    match json_obj {
        &Json::U64(num) => acc + num as i64,
        &Json::I64(num) => acc + num,
        &Json::F64(num) => acc + num as i64,
        &Json::Array(ref arr) => acc + arr.iter().fold(0_i64, parse_json_no_red),
        &Json::Object(ref obj) if !contains_red(obj.values()) => {
            acc + obj.values().fold(0_i64, parse_json_no_red)
        }
        _ => acc,
    }
}

fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_to_string(&mut input);

    let json = Json::from_str(&input).unwrap();

    let json_obj = json.as_object().unwrap();

    let total = json_obj.values().fold(0_i64, parse_json);

    let total_no_red = json_obj.values().fold(0, parse_json_no_red);

    println!("Sum: {}", total);
    println!("Sum w/o red: {}", total_no_red);
}


#[test]
fn parser_test() {
    use std::collections::BTreeMap;

    let input1 = Json::Array(vec![Json::U64(1), Json::U64(2), Json::I64(3)]);

    assert_eq!(6, parse_json(0, &input1));


    let mut input2 = BTreeMap::new();
    input2.insert("a".to_string(), Json::U64(2));
    input2.insert("b".to_string(), Json::I64(4));

    assert_eq!(6, parse_json(0, &Json::Object(input2)));


    let input3 = Json::Array(vec![Json::Array(vec![Json::Array(vec![Json::I64(3)])])]);

    assert_eq!(3, parse_json(0, &input3));


    let mut input4 = BTreeMap::new();
    let mut input4_a = BTreeMap::new();
    input4_a.insert("b".to_string(), Json::I64(4));
    input4.insert("a".to_string(), Json::Object(input4_a));
    input4.insert("c".to_string(), Json::I64(-1));

    assert_eq!(3, parse_json(0, &Json::Object(input4)));


    let mut input5 = BTreeMap::new();
    input5.insert("a".to_string(),
                  Json::Array(vec![Json::I64(-1), Json::I64(1)]));

    assert_eq!(0, parse_json(0, &Json::Object(input5)));

    let mut input6_a = BTreeMap::new();
    input6_a.insert("a".to_string(), Json::U64(1));

    assert_eq!(0, parse_json(0, &Json::Array(vec![Json::I64(-1),
                                                  Json::Object(input6_a)])));


    let input7: Vec<Json> = Vec::new();
    assert_eq!(0, parse_json(0, &Json::Array(input7)));

    let input8: BTreeMap<String, Json> = BTreeMap::new();
    assert_eq!(0, parse_json(0, &Json::Object(input8)));

}

#[test]
fn parser_test_red() {
    use std::collections::BTreeMap;

    let input1 = Json::Array(vec![Json::U64(1), Json::U64(2), Json::I64(3)]);

    assert_eq!(6, parse_json_no_red(0, &input1));


    let mut input2_a = BTreeMap::new();
    input2_a.insert("c".to_string(), Json::String("red".to_string()));
    input2_a.insert("b".to_string(), Json::I64(2));

    let input2 = Json::Array(vec![Json::I64(1),
                                  Json::Object(input2_a),
                                  Json::I64(3)]);

    assert_eq!(4, parse_json_no_red(0, &input2));


    let mut input3_a = BTreeMap::new();
    input3_a.insert("d".to_string(), Json::String("red".to_string()));

    input3_a.insert("e".to_string(),
                    Json::Array(vec![Json::I64(1),
                                     Json::I64(2),
                                     Json::I64(3),
                                     Json::I64(4)]));

    input3_a.insert("f".to_string(), Json::I64(5));

    let input_3 = Json::Object(input3_a);

    assert_eq!(0, parse_json_no_red(0, &input_3));


    let input4 = Json::Array(vec![Json::I64(1),
                                  Json::String("red".to_string()),
                                  Json::I64(5)]);

    assert_eq!(6, parse_json_no_red(0, &input4));
}
