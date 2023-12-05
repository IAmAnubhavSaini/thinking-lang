use futures::executor::block_on;

fn main() {
    block_on(hello());
}

async fn hello() {
    println!("Hello, world!");
}

