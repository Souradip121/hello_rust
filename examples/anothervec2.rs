fn main()
{
    let scores = vec![45, 82, 11, 78, 76, 55, 88];
    let result: i32 = scores
        .iter()
        .sum();
    println!("{}", result);
    println!("The average is {}", result/(scores.len() as i32));
    println!("The ans is {}", scores.iter().any(|x| *x < 30 ));
    println!("The ans is {}", scores.iter().all(|x| *x > 30));
}