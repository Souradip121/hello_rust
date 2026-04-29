fn main()
{
    let names = vec!["Souradip", "Arjun", "bob"];
    let results: Vec<String> = names
        .iter()
        .filter(|x| x.len()>3)
        .map(|x| x.to_uppercase())
        .collect();

    println!("{:?}", results);
}