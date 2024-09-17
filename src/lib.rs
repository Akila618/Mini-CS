use std::{sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}, usize};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}



//struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize)->ThreadPool{
        assert!(size>0);

        let(sender, reciever) = 
            mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            //create threads
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool{
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f :F)
    where
        F:FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.....");
        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers{
            println!("Shutting down the worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id:usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || loop {

            let message = reciever
            .lock()
            .unwrap()
            .recv()
            .unwrap();

            match message{
                Message::NewJob(job)=>{
                    println!("Worker {}: Executing ...", id);
                    job();
                }
                Message::Terminate=>{
                    println!("Worker {}: Terminating ...", id);
                    break;
                }
            }
        });

        //return Worker struct
        Worker{
            id,
            thread: Some(thread),
        }
    }
}