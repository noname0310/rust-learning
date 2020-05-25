fn main()
{
    let number_list = vec![34, 50, 25, 100, 65];

    println!("가장 큰숫자: {}", get_largest_number(&number_list));

    
    let number_list = vec![2983, 2198, 10929, 293, 192, 192];

    println!("가장 큰숫자: {}", get_largest_number(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    
    println!("가장 큰문자: {}", get_largest_number(&char_list));
}

fn get_largest_number<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}