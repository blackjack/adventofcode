fn inc(s: &mut [u8]) {
    for i in (0..s.len()).rev() {
        if s[i] == 'z' as u8 {
            if i == 0 {
                return;
            }

            if s[i - 1] != 'z' as u8 {
                s[i - 1] += 1;
                s[i] = 'a' as u8;
                return;
            }
        } else {
            s[i] += 1;
            return;
        }
    }
}

fn check(s: &[u8]) -> bool {
    if !s.windows(3).any(|x| x[1] == x[0] + 1 && x[2] == x[1] + 1) {
        return false;
    }
    if s.iter().any(|&x| ['i', 'o', 'l'].contains(&(x as char))) {
        return false;
    }
    let accumulator = |acc: u32, x: &[u8]| {
        if x.len() < 2 || x[0] != x[1] {
            return acc;
        } else {
            return acc + 1;
        }
    };

    let pairs_odd = s.chunks(2).fold(0, &accumulator);
    let pairs_even = s[1..s.len()].chunks(2).fold(0, &accumulator);
    if pairs_odd + pairs_even < 2 {
        return false;
    }

    return true;
}

fn main() {
    let mut input = "vzbxkghb".to_string().into_bytes();

    loop {
        inc(&mut input);
        if check(&input) {
            break;
        }
    }
    println!("New password is: {}", std::str::from_utf8(&input).unwrap());
}
