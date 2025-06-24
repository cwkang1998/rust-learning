use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel::<u8>();

    let send_result = tx.send(100);
    println!("Send result {}", send_result.is_ok());
    let send_result = tx.send(152);
    println!("Send result {}", send_result.is_ok());

    let t1 = thread::spawn(move || {
        loop {
            println!("Attempting to receive message");
            let data = rx.recv_timeout(Duration::from_millis(500));
            println!("data {:?}", data);
        }
    });

    t1.join().unwrap();
}
