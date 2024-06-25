#![allow(dead_code, unused_variables)]

use std::future::Future;

fn main() {
    println!("Hello, world!");

    let x = foo1();
    let y = foo2();
}

async fn foo1() -> usize {
    println!("foo1");
    42
}

fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo2");
        let res = foo1().await + 42;
        println!("foo2: res = {}", res);
        res
    }
}
