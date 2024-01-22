use rand::Rng;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    send_sometimes(&tx, "owo");
    send_sometimes(&tx, "uwu");

    thread::spawn(move || loop {
        let messages: Vec<_> = rx.try_iter().collect();
        if messages.is_empty() {
            println!("Zzz…");
        } else {
            println!("Got messages: {}", messages.join(", "));
        }

        sleep(Duration::from_secs(1));
    })
    .join()
    .unwrap();
}

fn send_sometimes(tx: &Sender<String>, message: &str) {
    let own_tx = tx.clone();
    let own_message = message.to_owned();
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            own_tx.send(own_message.clone()).unwrap();

            let timeout = rng.gen_range(200..=3000);
            sleep(Duration::from_millis(timeout));
        }
    });
}
