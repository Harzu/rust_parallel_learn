use std::thread;
use std::time::Duration;
use std::sync::mpsc::{ channel, sync_channel };

fn async_channel_example() {
    let (sender, reciver) = channel();
    thread::spawn(move || {
        sender.send(10).unwrap();
    });

    let read = reciver.recv().unwrap();
    dbg!(read);
}

fn sync_channel_example() {
    let (sync_sender, misc_reciver) = sync_channel(100);
    thread::spawn(move || {
        sync_sender.send("Hello").unwrap();
    });

    let sync_message = misc_reciver.recv().unwrap();
    dbg!(sync_message);
}

fn read_in_loop() {
    let timeout_thread = Duration::from_secs(2);
    let (sender, reciver) = channel();
    
    thread::spawn(move || {
        let timeout = Duration::from_secs(3);
        loop {
            match reciver.recv_timeout(timeout) {
                Ok(res) => { println!("read value in thread {}", res); },
                Err(e) => panic!("Timeout send {:?}", e)
            };
        }
    });

    for i in 0..10 {
        let sender = sender.clone();
        thread::spawn(move || { sender.send(i).unwrap(); });
        thread::sleep(timeout_thread);
    }

}

fn main() {
    async_channel_example();
    sync_channel_example();
    read_in_loop();
}
