use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

/// ### channel
///
/// > mpsc: Multi-Produce Single-Consumer
///
/// 线程通信使用 channel（异步通道）
///

static NTHREADS: u32 = 3;

fn main() {
    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            // 是一个 block 操作
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);

    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    println!("ids: {:?}", ids);
}
