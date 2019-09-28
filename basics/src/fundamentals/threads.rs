use std::sync::{mpsc, Arc, LockResult, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn main() {
    simple_example();
    move_example();
    builder_example();
    simple_channel_example();
    channel_multiple_messages();
    channel_multiple_producers_and_messages();
    mutex_examples();
}

fn simple_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join();
}

fn move_example() {
    let value = 5;
    // move keyword force ownership to move
    let handle = thread::spawn(move || {
        println!("Value: {} ", value);
    });

    handle.join();
}

fn builder_example() {
    // threads will we terminated when main thread will terminate
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 1..5 {
        let handle = thread::Builder::new()
            .name(format!("Worker-{}", i).into())
            .spawn(|| {
                let current_thread = thread::current();
                for i in 1..10 {
                    println!("Thread {:?} nbr {:?}", current_thread.name().unwrap(), i);
                    thread::sleep(Duration::from_millis(1));
                }
            });
        match handle {
            Ok(x) => handles.push(x),
            Err(e) => eprintln!("Unable to launch thread: {}", e),
        }
    }

    for handle in handles {
        handle.join();
    }
}

pub fn simple_channel_example() {
    // mpsc stands for multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // ownership moved into thread
    });

    // recv() will wait until message received
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn channel_multiple_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // rx can be used as an iterator
    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn channel_multiple_producers_and_messages() {
    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // rx can be used as an iterator
    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex_examples() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // here we try to acquire mutex lock to access value
            match counter.lock() {
                Ok(mut x) => *x += 1,
                Err(e) => eprintln!("Error: {:?}", e),
            }
            // mutex lock will be released by drop trait
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
