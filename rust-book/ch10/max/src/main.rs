fn main() {
    try_largest();
    try_longest();
}

fn try_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn try_longest() {
    let a = String::from("hello");
    let mut c = "";
    {
        let b = String::from("world!");
        c = longest(a.as_str(), b.as_str());
    }

    println!("{}", c);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut m = &list[0];
    for i in list {
        if i > m {
            m = i
        }
    }

    m
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}