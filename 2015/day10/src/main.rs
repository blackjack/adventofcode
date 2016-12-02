fn main() {
    let input = "1113122113";
    let itercount = 50;

    let mut result: Vec<_> = input.chars().map(|c|c.to_digit(10).unwrap()).collect();

    for iter in 0..itercount {
        let mut tmp = Vec::new();
        let mut current = result[0];
        let mut count = 1;

        for (i,&n) in result.iter().enumerate() {
            if i==0 {continue}
            if n == current {
                count += 1;
            } else {
                tmp.push(count);
                tmp.push(current);
                count = 1;
            }
            current = n;
        }
        tmp.push(count);
        tmp.push(current);
        result = tmp.clone();

        if iter==40 {
            println!("Result after 40 iterations is  length is: {}",result.len());
        }
    }
    println!("Result after {} iterations is  length is: {}",itercount,result.len());

}
