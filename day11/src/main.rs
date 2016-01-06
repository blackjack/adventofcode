fn inc(s: &mut [u8]) {
    for i in (0..s.len()).rev() {
        if s[i] == 'z' as u8 {
            s[i] = 'a' as u8;
        } else {
            s[i] += 1;
            return;
        }
    }
}

fn check(s: &[u8]) -> bool {
    let mut pairs = 0;
    let mut has_inc = false;
    let mut prev = 0u8;
    let mut prev2 = 0u8;

    for &c in s.iter() {
        if ['i', 'o', 'l'].contains(&(c as char)) {
            return false;
        }

        if c == prev + 1 && prev == prev2 + 1 {
            has_inc = true;
        }

        if c == prev && prev != prev2 {
            pairs += 1;
        }

        prev2 = prev;
        prev = c;
    }

    return has_inc && pairs >= 2;
}

fn main() {
    let mut input = "vzbxkghb".to_string().into_bytes();

    loop {
        inc(&mut input);
        if check(&input) {
            break;
        }
    }
    println!("First password is: {}", std::str::from_utf8(&input).unwrap());

    loop {
        inc(&mut input);
        if check(&input) {
            break;
        }
    }
    println!("Second password is: {}", std::str::from_utf8(&input).unwrap());
}


#[test]
fn test1() {
    assert_eq!(check(&"hijklmmn".to_string().into_bytes()), false);
}

#[test]
fn test2() {
    assert_eq!(check(&"abcdffaa".to_string().into_bytes()), true);
}

#[test]
fn test3() {
    let mut input = "abcdefgh".to_string().into_bytes();
    loop {
        inc(&mut input);
        println!("New password is: {}", std::str::from_utf8(&input).unwrap());
        if check(&input) {
            println!("Passes: {}", std::str::from_utf8(&input).unwrap());
            break;
        }
    }
    assert_eq!("abcdffaa".to_string().into_bytes(), input);
}

#[test]
fn test4() {
    let mut input = "abcdfezz".to_string().into_bytes();
    inc(&mut input);
    assert_eq!("abcdffaa".to_string().into_bytes(), input);
}
