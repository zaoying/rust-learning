extern crate rand;

use rand::Rng;
use std::cmp::Ordering;

fn sort() {
    
    println!("生成10个随机数，范围从1到100");

    let mut random_list = [0;10];

    for i in 0..10 {
        let random = rand::thread_rng().gen_range(1, 101);
        println!("生成第{}个随机数：{}", i + 1, random);
        random_list[i] = random;
    }

    let length =  random_list.len();
    for outter in 0..length {
        let inner_start = outter + 1;
        for inner in inner_start..length {
            let mut pre = random_list[outter];
            let mut next= random_list[inner];
            swap(&mut pre, &mut next);
            random_list[outter] = pre;
            random_list[inner] = next;
        }
    }

    println!("\n排序后：");
    for sorted in random_list.iter() {
        print!("{},", sorted);
    }
    println!("");
}

fn swap<'a>(pre:&mut i32, next:&mut i32) {
    match pre.cmp(&next) {
        Ordering::Less => {}
        Ordering::Equal => {}
        Ordering::Greater => {
            let tmp = *pre;
            *pre = *next;
            *next = tmp;
        }
    }
}
