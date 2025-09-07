use std::{
    thread,
    sync::{Arc, Mutex, mpsc}
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
    should_gracefully_shutdown: bool
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "{}", "Thread pool size must be greater than 0");
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers: workers,
            sender: Option::Some(sender),
            should_gracefully_shutdown: true
        }
    }

    pub fn execute<F>(&self, func: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(func);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    pub fn join(&mut self) {
        for worker in self.workers.drain(..) {
            worker.thread.join().unwrap()
        }
    }

    pub fn disable_graceful_shutdown(&mut self) {
        self.should_gracefully_shutdown = false
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        if !self.should_gracefully_shutdown {
            return
        }

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap()
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("worker {} got a job. executing.", id);
                        job()
                    }
                    Err(_) => {
                        println!("Worker {} disconnected. shutting down", id);
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;


#[cfg(test)]
mod tests {
    use std::{time::Duration};

    use super::*;

    #[test]
    #[should_panic]
    fn creating_a_thread_pool_panics_if_size_is_0() {
        ThreadPool::new(0);
    }

    #[test]
    fn thread_pool_should_create_the_specified_number_of_threads() {
        let mut pool = ThreadPool::new(4);
        pool.disable_graceful_shutdown();

        let mut states = Vec::new();
        for _ in 0..5 {
            let task_state = TaskState::new();
            let copy = Arc::clone(&task_state);
            states.push(task_state);
            pool.execute(move || {
                copy.lock().unwrap().is_started = true;
                while copy.lock().unwrap().shut_down == false {
                    thread::sleep(Duration::from_millis(5))
                }
            });
        }

        thread::sleep(Duration::from_millis(50));

        assert!(!states[4].lock().unwrap().is_started);
        states[4].lock().unwrap().shut_down = true;
        for i in 0..4 {
            let task_state = &states[i];
            assert!(task_state.lock().unwrap().is_started);
        }
        for state in &states {
            println!("{}", state.lock().unwrap().shut_down);
            state.lock().unwrap().shut_down = true
        };
    }

    struct TaskState {
        is_started: bool,
        shut_down: bool
    }

    impl TaskState {
        pub fn new() -> Arc<Mutex<TaskState>> {
            Arc::new(Mutex::new(TaskState { is_started: false, shut_down: false }))
        }
    }
}
