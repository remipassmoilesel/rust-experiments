use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

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

pub fn main() {
    simple_example();
    builder_example();
}
