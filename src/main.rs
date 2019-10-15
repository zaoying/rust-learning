extern crate rand;

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    sort();
}

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
            let _pre:&i32 = &mut random_list[outter];
            let _next:&i32 = &mut random_list[inner];
            let pre:&mut i32 = _pre;
            let next:&mut i32 = _next;
            swap(pre, next);
        }
    }

    println!("\n排序后：");
    for sorted in random_list.iter() {
        print!("{},", sorted);
    }
}

fn swap<'a>(pre:&'a mut i32, next:&'a mut i32) {
    match pre.cmp(&next) {
        Ordering::Less => {}
        Ordering::Equal => {}
        Ordering::Greater => {
            let tmp:&'a mut i32 = pre;
            pre = next;
            next = tmp;
        }
    }
}