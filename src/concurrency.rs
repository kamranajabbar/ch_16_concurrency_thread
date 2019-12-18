use std::thread;
use std::time::Duration;

pub fn run() {
    // Single thread
    // for i in 1..10{
    //     println!("executing from i loop: {}", i);
    // }

    // for j in 1..10{
    //     println!("executing from j loop: {}", j);
    // }

    // Multithreading
    for i in 1..10{
        println!("executing from i loop: {}", i);
    }

    for j in 1..10{
        println!("executing from j loop: {}", j);
    }
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