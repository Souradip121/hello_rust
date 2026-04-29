struct Stack<T> {
    item:Vec<T>,
}

impl<T> Stack<T>  {
    fn new() -> Stack<T>
    {
        Stack{
            item: Vec::new(),
        }
    }

    fn push(&mut self, item:T){
        self.item.push(item);
    }

    fn pop(&mut self) -> Option<T>{
        self.item.pop()
    }
    fn is_empty(&self) -> bool{
        self.item.is_empty()
    }
}

fn main()
{
    let mut s1 = Stack::<i32>::new();
    s1.push(1);
    s1.push(2);
    s1.push(3);
    println!("{:?}", s1.pop());   
    println!("{:?}", s1.pop());
    println!("{}", s1.is_empty());
}

