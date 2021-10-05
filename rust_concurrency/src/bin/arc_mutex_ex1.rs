// Topic: Arc, Mutex, and Threads
//
// About this example:
// This is a multi-threaded example that uses a global counter shared among the threads.
// For sharing it, `Arc` (from `std` lib) and `Mutex` (from `parking_lot` crate) are used.
// The counter increases by 1 whenever a worker completes a job.
//

use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Job given to workers.
#[derive(Clone)]
enum Job {
    Print(String),
    Sum(isize, isize),
}

/// Message sent to workers.
#[derive(Clone)]
enum Message {
    AddJob(Job),
    Quit,
}

struct Worker<M> {
    tx: Sender<M>,
    _rx: Receiver<M>,
    handle: JoinHandle<()>,
}

impl Worker<Message> {
    fn add_job(&self, job: Job) {
        self.tx
            .send(Message::AddJob(job))
            .expect("failed to add job");
    }
    fn join(self) {
        self.handle.join().expect("failed to join thread");
    }
    fn send_msg(&self, msg: Message) {
        self.tx.send(msg).expect("failed to send message");
    }
}

/// Create a new worker to receive jobs.
fn spawn_worker(counter: Arc<Mutex<usize>>) -> Worker<Message> {
    let (tx, rx) = unbounded();
    // We clone the receiving end here so we have a copy to give to the
    // thread. This allows us to save the `tx` and `rx` into the Worker struct.
    let rx_thread = rx.clone();
    // Spawn a new thread.
    let handle = thread::spawn(move || {
        // VecDeque allows us to get jobs in the order they arrive.
        let mut jobs = VecDeque::new();
        // Outer loop is so we can have a brief delay when no jobs are available.
        loop {
            // Inner loop continuously processes jobs until no more are available.
            loop {
                // Get and process the next job.
                for job in jobs.pop_front() {
                    match job {
                        Job::Print(msg) => println!("{}", msg),
                        Job::Sum(lhs, rhs) => println!("{}+{}={}", lhs, rhs, lhs + rhs),
                    }
                    // Getting the (exclusive lock), so we can increase the counter.
                    let mut counter = counter.lock();
                    *counter += 1;
                }
                // Check for messages on the channel.
                if let Ok(msg) = rx_thread.try_recv() {
                    match msg {
                        Message::AddJob(job) => {
                            // When we receive a new job, add it to the jobs list.
                            jobs.push_back(job);
                            // Continue processing jobs.
                            continue;
                        }
                        Message::Quit => return,
                    }
                } else {
                    // No messages on the channel, break from inner loop
                    // and thread will wait momentarily for more messages.
                    break;
                }
            }
            // Pause to wait for more messages to arrive on channel.
            thread::sleep(Duration::from_millis(100));
        }
    });

    Worker {
        tx,
        _rx: rx,
        handle,
    }
}

fn main() {
    let jobs = vec![
        Job::Print("hello".to_owned()),
        Job::Sum(2, 2),
        Job::Print("world".to_owned()),
        Job::Sum(4, 4),
        Job::Print("two words".to_owned()),
        Job::Sum(1, 1),
        Job::Print("a print job".to_owned()),
        Job::Sum(10, 10),
        Job::Print("message".to_owned()),
        Job::Sum(3, 4),
        Job::Print("thread".to_owned()),
        Job::Sum(9, 8),
        Job::Print("rust".to_owned()),
        Job::Sum(1, 2),
        Job::Print("compiler".to_owned()),
        Job::Sum(9, 1),
    ];

    let jobs_sent = jobs.len();
    let jobs_counter = Arc::new(Mutex::new(0));
    let mut workers = vec![];

    // Spawn 4 workers to process jobs.
    for _ in 0..4 {
        // FYI: An alternative to jobs_counter.clone() is Arc::clone(&jobs_counter).
        let worker = spawn_worker(jobs_counter.clone());
        workers.push(worker);
    }

    // Create an iterator that cycles through each worker endlessly.
    let mut worker_ring = workers.iter().cycle();
    for job in jobs.into_iter() {
        // Get next worker
        let worker = worker_ring.next().expect("failed to get worker");
        // Add the job
        worker.add_job(job);
    }

    // Ask all workers to quit.
    for worker in &workers {
        worker.send_msg(Message::Quit);
    }

    // Wait for workers to terminate.
    for worker in workers {
        worker.join();
    }

    println!("Jobs sent: {}", jobs_sent);
    println!("Jobs done: {}", jobs_counter.lock());
}
