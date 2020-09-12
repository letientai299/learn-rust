#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() takes ownership of vector's elements. Hence, after collect,
    // shoe has type Shoe.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()

    // iter() doesn't, hence, after collect, shoe has type &Shoe.
    // Because they're references, they need to bind with a lifetime.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("s1") },
            Shoe { size: 11, style: String::from("s2") },
            Shoe { size: 12, style: String::from("s3") },
            Shoe { size: 10, style: String::from("s4") },
        ];

        let my_shoes = shoe_in_my_size(shoes, 10);

        assert_eq!(my_shoes, vec![
            Shoe { size: 10, style: String::from("s1") },
            Shoe { size: 10, style: String::from("s4") },
        ]);
    }
}
