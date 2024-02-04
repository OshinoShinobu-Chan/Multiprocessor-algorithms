struct Peterson {
    flag: [bool; 2],
    victim: i32,
}

static mut LOCK: Peterson = Peterson {
    flag: [false, false],
    victim: -1, // -1 means no one is victim
};

fn lock(id: i32) {
    unsafe {
        LOCK.flag[id as usize] = true;
        LOCK.victim = id;
        while LOCK.flag[1 - id as usize] && LOCK.victim == id {}
    }
}

fn unlock(id: i32) {
    unsafe {
        LOCK.flag[id as usize] = false;
    }
}

use std::thread;

fn main() {
    println!("Hello, world!");
    let mut handles = vec![];
    for i in 0..2 {
        let id = i;
        let handle = thread::spawn(move || {
            lock(id);
            // *** start critical section
            println!("Thread {id} start");
            thread::sleep(std::time::Duration::from_millis(100));
            println!("Thread {id} end");
            // *** end critical section
            unlock(id);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
