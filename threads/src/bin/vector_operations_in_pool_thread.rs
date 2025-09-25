use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct SharedVec<T> {
    data: Mutex<Vec<T>>,
}

impl<T: PartialEq + std::fmt::Debug + Send + 'static> SharedVec<T> {
    fn new() -> Self {
        SharedVec {
            data: Mutex::new(Vec::new()),
        }
    }

    fn add(&self, value: T) {
        let mut vec = self.data.lock().unwrap();
        vec.push(value);
        println!("Added: {:?}", vec);
    }

    fn remove(&self, value: &T) -> bool {
        let mut vec = self.data.lock().unwrap();
        if let Some(pos) = vec.iter().position(|x| x == value) {
            vec.remove(pos);
            println!("Removed {:?}, new state: {:?}", value, vec);
            true
        } else {
            println!("Value {:?} not found, nothing removed", value);
            false
        }
    }

    fn search(&self, value: &T) -> bool {
        let vec = self.data.lock().unwrap();
        let found = vec.contains(value);
        println!("Search {:?} → {}", value, found);
        found
    }

    fn len(&self) -> usize {
        let vec = self.data.lock().unwrap();
        vec.len()
    }
}

fn main() {
    // Wrap in Arc so multiple threads can own it.
    let shared_vec = Arc::new(SharedVec::<i32>::new());

    // Spawn a pool of worker threads
    let mut handles = vec![];
    for i in 0..5 {
        let vec_clone = Arc::clone(&shared_vec);
        let handle = thread::spawn(move || {
            // Each thread does a different kind of operation
            match i {
                0 => {
                    for n in 1..=5 {
                        vec_clone.add(n);
                        thread::sleep(Duration::from_millis(50));
                    }
                }
                1 => {
                    thread::sleep(Duration::from_millis(100));
                    vec_clone.remove(&3);
                }
                2 => {
                    thread::sleep(Duration::from_millis(150));
                    vec_clone.search(&2);
                }
                3 => {
                    thread::sleep(Duration::from_millis(200));
                    vec_clone.remove(&10); // not in vector
                }
                4 => {
                    thread::sleep(Duration::from_millis(250));
                    vec_clone.add(99);
                }
                _ => {}
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final vector length: {}", shared_vec.len());
}
