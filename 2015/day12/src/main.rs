extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::io::Read;


fn calc(json: &Json) -> i64 {
    match *json {
        Json::Array(ref a) => a.iter().fold(0i64, |acum, x| acum + calc(x)),
        Json::Object(ref a) => a.iter().fold(0i64, |acum, x| acum + calc(x.1)),
        Json::I64(ref a) => *a,
        Json::U64(ref a) => *a as i64,
        Json::F64(ref a) => *a as i64,
        _ => 0,
    }
}

fn calc2(json: &Json) -> i64 {
    match *json {
        Json::Array(ref a) => a.iter().fold(0i64, |acum, x| acum + calc2(x)),
        Json::Object(ref a) => {
            if a.values().any(|x| if let Json::String(ref s)=*x {s=="red"} else {false}) {
                0
            } else {
                a.iter().fold(0i64, |acum, x| acum + calc2(x.1))
            }
        },
        Json::I64(ref a) => *a,
        Json::U64(ref a) => *a as i64,
        Json::F64(ref a) => *a as i64,
        _ => 0,
    }
}

fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();

    println!("Sum of numbers (case 1): {}", calc(&json));
    println!("Sum of numbers (case 2): {}", calc2(&json));
}


#[test]
fn test1() {
    assert_eq!(calc(&Json::from_str("[1,2,3]").unwrap()), 6);
    assert_eq!(calc(&Json::from_str(r#"{"a":{"b":4},"c":-1}"#).unwrap()), 3);
}

#[test]
fn test2() {
    assert_eq!(calc2(&Json::from_str(r#"[1,{"c":"red","b":2},3]"#).unwrap()), 4);
    assert_eq!(calc2(&Json::from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap()), 0);
}
