fn main()
{
    let numbers = vec![1,2,3,4,5,6,7,8,9,10];
    let result:Vec<i32> = numbers
        .iter()
        .filter(|x| *x%2==1)
        .map(|x| x*3)
        .collect();
    println!("{:?}",result);
}