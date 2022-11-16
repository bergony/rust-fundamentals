fn add(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two
}

fn main() {
    let mut total: i32 = add(10, 15);
    let mut free_shipping = false;

    if total > 50 {
        println!("You qualify for free shipping!");
        free_shipping = true;
    } else if total > 20 {
        println!("If you add more items, you cna qualify for free shipping")
    } else {
        println!("No free shipping")
    }
    total = match free_shipping {
        true => total + 0,
        false => total + 5,
    };

    match total {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("No Match found"),
    }

    println!("Total: {:?}", total);

    let items: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", items);

    let vector_items = vec![1, 2, 3, 4, 5];
    let mut vector_items2 = Vec::new();
    vector_items2.push(1);
    vector_items2.push(2);
    vector_items2.push(3);
    vector_items2.push(4);
    vector_items2.push(5);

    println!("{:?}", vector_items);

    println!("{:?}", vector_items2);
}
