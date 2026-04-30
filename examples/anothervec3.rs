fn main()
{
    let scores = vec![45, 82, 91, 38, 76, 55, 88];
    let results = scores
        .iter()
        .find(|x| **x>80);

    println!("First Score {:?} ",results);

    for (score,i) in scores.iter().enumerate()
    {
        println!("{} : {}",i,score);
    }
}