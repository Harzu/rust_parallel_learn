extern crate rand;

use rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::{ Mutex, Arc };

fn main() {
    let data = vec![1, 2, 3];
    let data_length = data.len();
    let mutex_data = Arc::new(Mutex::new(data));
    
    let mut handles = vec![];

    for j in 0..10 {
        let mutex_data = mutex_data.clone();
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let ms = rng.gen::<u8>();
            
            thread::sleep(Duration::from_millis(ms.into()));
            
            for i in 0..data_length {        
                let mut data = mutex_data.lock().unwrap();
                data[i] += 1 * j;
            };

            println!("i thread number {} and my result = {:?}\n", j, mutex_data);
            mutex_data
        });

        handles.push(handle);
    }

    for item in handles {
        item.join().unwrap();
    }
}
