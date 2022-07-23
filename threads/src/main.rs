use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..11 {
            println!(
                "spawned thread > is logging number {} of 10 and sleeping 1ms",
                i
            );
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..6 {
        println!(
            "main thread > is logging number {} of 5 and sleeping 1ms",
            i
        );
        thread::sleep(Duration::from_millis(1))
    }

    handle.join().unwrap();
}
