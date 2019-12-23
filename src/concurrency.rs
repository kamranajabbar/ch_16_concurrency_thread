use std::thread;
use std::time::Duration;

pub fn run() {
    // Singlethread
    // for i in 1..10{
    //     println!("executing from i loop: {}", i);
    // }

    // for j in 1..10{
    //     println!("executing from j loop: {}", j);
    // }

    // Multithreading
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("executing from i loop from custome thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    /*
        If we want to execute first custome thread completely, and then main thread, so we have to join() with handle after custome thread.
    */
    //handle.join().unwrap();

    for j in 1..5 {
        println!("executing from j loop from main thread: {}", j);
        thread::sleep(Duration::from_millis(1));
    }

    /*
        Note:
            When main thread executed completely, then custome thread will be stoped, if we want to execute custome thread completely either main thread completed its looping/cycle. So we have to used join() with handle in main tread in last. if we want to execute theards simultaneously so we need to used join() in last in main thread.
    */
    handle.join().unwrap();
}

/*
    What is concurrency?
    It is ability of a program to be decomposed into parts that can run independently of each ohter.

    What is thread?
    A thread is the smallest unit of a process.

    What is process?
    A single process may contain multiple threads.

    What is multitasking?
    Multitasking allows CPU to perform multiple task (pograms, processs, tasks, theards) simultaneously.

    What is multithreading?
    Multithreading allows multiple threads of the same process to execute simultaneously.

    Benefits of multithreading:
    1. Splitting the computation in your program into multiple threads can improve performance, as the program does multiple task at same time.
    2. Other benefits include resource sharing, responsiveness, and utilization of multiprocessor architecture. 

    Drawbacks of multithreading:
    1. It adds complexity.
    2. There is no guarantee about the order in which parts of 3. your code on different threads will run.
    4. This can leads to problems such as race conditions, deadlocks, and irreproduciable bugs.
*/