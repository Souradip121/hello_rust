trait Summary{
    fn summarise(&self)-> String;
}

struct Article{
    title: String,
    author: String,
}

struct Tweet{
    username: String,
    content: String,
}

impl Summary for Article{
    fn summarise(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Summary for Tweet{
    fn summarise(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

fn notify(item:&impl Summary)
{
    println!("Breaking news: {}", item.summarise());
}

fn main(){
    let article1 = Article{title: String::from("souradip"), author:String::from("souradip")};
    let tweet1 = Tweet{username: String::from("souradip"),content: String::from("souradip")};
    notify(&article1);
    notify(&tweet1);
}