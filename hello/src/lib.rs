use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
	NewJob(Job),
	Terminate,
}

impl ThreadPool {
	/// Create a new `ThreadPool`.
	///
	/// The size is the number of threads in the pool.
	///
	/// # Panics
	///
	/// The `new` function will panic if the size is zero.
	#[must_use = "calling a constructor and throwing away its output"]
	pub fn new(size: usize) -> Self {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		Self { workers, sender }
	}

	/// # Errors
	///
	/// The `execute` function may error when calling `std::sync::mpsc::Sender::send`.
	pub fn execute<F>(&self, f: F) -> Result<(), std::sync::mpsc::SendError<Message>>
	where
		F: FnOnce() + Send + 'static,
	{
		let job = Box::new(f);

		self.sender.send(Message::NewJob(job))?;

		Ok(())
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		println!("Sending terminate message to all workers.");

		for _ in &self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}

		println!("Shutting down all workers.");

		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);

			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}

struct Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
		let thread = thread::spawn(move || loop {
			let message = receiver.lock().unwrap().recv().unwrap();

			match message {
				Message::NewJob(job) => {
					println!("Worker {} got a job; executing.", id);

					job();
				}
				Message::Terminate => {
					println!("Worker {} was told to terminate.", id);

					break;
				}
			}
		});

		Self {
			id,
			thread: Some(thread),
		}
	}
}
