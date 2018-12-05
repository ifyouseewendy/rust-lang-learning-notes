fn largest<'a, T>(list: &'a [T]) -> &'a T
    where T: PartialOrd
{
    let mut &largest = &list[0];

    for &item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![3,2,1,4];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
