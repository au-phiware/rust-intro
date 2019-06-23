




























let Deque {
    head,
    tail,
    buffer,
} = self;

let x = buffer[tail];

buffer[head] = a;
head += 1;

















if let Some(x) = x {
    println!("Got something: {}", x);
}





















pub fn unwrap_or_default(self) -> T {
    match self {
        Some(x) => x,
        None => Default::default(),
    }
}










