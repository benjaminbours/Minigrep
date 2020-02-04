use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut random_list = create_random_list();
    println!("{:?}", random_list);

    let mode = mode(&random_list);
    println!("mode: {:?}", mode);
    let mode = average(&random_list);
    println!("average: {:?}", mode);
    let mode = median(&mut random_list);
    println!("mean: {:?}", mode);
}

fn create_random_list() -> Vec<u32> {
    let list_length = rand::thread_rng().gen_range(100, 500);
    let mut list = Vec::new();
    // let mut list: Vec<u32> = Vec::with_capacity(list_length);

    for _i in 0..list_length {
        let random_number = rand::thread_rng().gen_range(1, 100);
        list.push(random_number);
    }

    list
}

fn mode(list: &Vec<u32>) -> u32 {
    let mut map = HashMap::new();

    for &i in list {
        println!("{}", i);
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);

    map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val).expect("Cannot compute the mode of the numbers")
}

fn average(list: &Vec<u32>) -> u32 {
    list.iter().sum::<u32>() as u32 / list.len() as u32
}

fn median(list: &mut Vec<u32>) -> u32 {
    list.sort();
    let mid = list.len() / 2;
    list[mid]
}

