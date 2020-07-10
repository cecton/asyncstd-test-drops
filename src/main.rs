// test 1 created
// 1
// 2
// test 2 created
// 1
// Dropping 2...
// test 1 dropped
// Dropping 1...
// Dropped 2.
// 2

#![allow(unused_imports)]

use futures::future::FutureExt;
use futures::{select, pin_mut};

struct Test(usize);

impl Drop for Test {
    fn drop(&mut self) {
        println!("Dropping {}...", self.0);
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("Dropped {}.", self.0);
    }
}

async fn task_1() {
    let test = Test(1);
    println!("test 1 created");
    std::thread::sleep(std::time::Duration::from_secs(5));
    async_std::task::sleep(std::time::Duration::from_secs(3)).await;
    println!("test 1 dropped");
    drop(test);
}

async fn task_2() {
    let test = Test(2);
    println!("test 2 created");
    std::thread::sleep(std::time::Duration::from_secs(5));
    tokio::time::delay_for(std::time::Duration::from_secs(3)).await;
    println!("test 2 dropped");
    drop(test);
}

fn main() {
    // with async-std
    async_std::task::block_on(async {
        async_std::task::spawn(task_1());
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        println!("1");
    });
    println!("2");

    with_tokio();
    println!("2");
}

#[tokio::main]
async fn with_tokio() {
    tokio::spawn(task_2());
    tokio::time::delay_for(std::time::Duration::from_secs(2)).await;
    println!("1");
}
