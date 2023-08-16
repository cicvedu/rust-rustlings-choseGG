use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Mutex<Queue>>, tx: mpsc::Sender<u32>) {
    let thread1 = {
        let q = Arc::clone(&q);
        let tx = tx.clone();
        thread::spawn(move || {
            let q = q.lock().unwrap();
            for val in &q.first_half {
                println!("sending {:?}", val);
                tx.send(*val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        })
    };

    let thread2 = {
        let q = Arc::clone(&q);
        let tx = tx.clone();
        thread::spawn(move || {
            let q = q.lock().unwrap();
            for val in &q.second_half {
                println!("sending {:?}", val);
                tx.send(*val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        })
    };

    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new()));
    let queue_length = queue.lock().unwrap().length;

    send_tx(Arc::clone(&queue), tx);

    let mut total_received: u32 = 0;
    let timeout = Duration::from_secs(queue_length as u64 * 2); // increase timeout to avoid prematurely exit
    while let Ok(received) = rx.recv_timeout(timeout) {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}