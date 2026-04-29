fn shorter<'a>  (str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len()
      {
               str2
         }
      else
           {
              str1
           }
}
struct Excerpt<'a> {
     text: &'a str,
  }
fn main()
{
let s1: &str = "souradip";
let s2: &str = "baivab";
println!("{}",shorter(s1,s2));
let s3 = Excerpt{text:s1};
println!("{}",s3.text);
}


fn first_or_second<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > 0 {
        x
    } else {
        y
    }
}