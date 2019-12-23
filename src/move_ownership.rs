use std::thread;

pub fn run() {
    let v = vec![1,2,3,4];

    let handle = thread::spawn(move || {
        println!("vector : {:?}", v);
    });

    handle.join().unwrap();
}

/*
    if we have custome thread and try to access any variable but subjected varibale is defined in main thread, so closure should have "move" keyword to get ownership of subjected variable and pass to that thread to used it. As "move" is used to taking ownership of all envoirment varibale in scope.
*/