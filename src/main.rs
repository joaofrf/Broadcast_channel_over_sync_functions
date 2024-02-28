use std::{thread::{self, sleep}, time::Duration};
use crossbeam::channel;
use async_broadcast::broadcast;
use rand::Rng;



//no error handling here... just a small test... leave one uncommented to test
fn main() {

    // crossbeam_library_channel();
    async_broadcast_library_channel();
    
}


/*

All clients receive the message, we cant use tx.broadcast(y) since we're not inside an async function
It still works just fine with try_broadcast(y), same with recv(), we must use try_recv()

*/
fn async_broadcast_library_channel(){
    let (tx, mut rx) = broadcast::<u32>(1024);
    
    
    let sender_thread = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            let y: u32 = rng.gen_range(1..50);
            println!("Sending {y}");
            let _ = tx.try_broadcast(y);
            
            sleep(Duration::from_secs(1));
        }
    });

    let mut rx_clone_1 = rx.clone();
    let receiver_thread_1 = thread::spawn(move || {
        loop {
            match rx_clone_1.try_recv() {
                Ok(msg) => {
                    println!("receiver thread 1 | {}", msg);
                }
                Err(..) => {}
            }
        }
    });

    let receiver_thread_2 = thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Ok(msg) => {
                    println!("receiver thread 2 | {}", msg);
                }
                Err(..) => {}
            }
        }
    });

    let _ = receiver_thread_1.join();
    let _ = receiver_thread_2.join();
    let _ = sender_thread.join();
}


/*

The first thread to try_recv() will receive the value and clear the buffer, other threads won't get the value.

*/
fn crossbeam_library_channel(){
    let (tx, rx) = channel::bounded::<u32>(1024);
    
    
    let sender_thread = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            let y: u32 = rng.gen_range(1..50);
            println!("standard_library_channels {y}");
            let _ = tx.send(y);
            
            sleep(Duration::from_secs(1));
        }
    });

    let rx_clone_1 = rx.clone();
    let receiver_thread_1 = thread::spawn(move || {
        loop {
            match rx_clone_1.try_recv() {
                Ok(msg) => {
                    println!("standard_library_channels receiver thread 1 | {}", msg);
                }
                Err(..) => {}
            }
        }
    });

    let receiver_thread_2 = thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Ok(msg) => {
                    println!("standard_library_channels receiver thread 2 | {}", msg);
                }
                Err(..) => {}
            }
        }
    });

    let _ = receiver_thread_1.join();
    let _ = receiver_thread_2.join();
    let _ = sender_thread.join();
}
