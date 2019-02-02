extern crate reqwest;

use std::thread;
use std::time::Duration;

mod reciever {
    use std::sync::mpsc;
  
    struct Messages<T,U> {
        numbers: Vec<i32>, 
        tx: T,
        rx: U,
    }
    impl<T,U> Messages {
        pub fn new() -> Messages {
            let (tx, rx) = mpsc::channel();
            Messages {
                numbers: vec![],
                tx: tx,
                rx: rx
            }
        }

        pub fn send(&self, num: i32) {
            self.tx.send(num).unwrap();
        }

        pub fn recieve(&self, num: i32) {
            for received in self.rx {
                self.numbers.push(num);
                println!("Got: {}", received);
            }
        }
    }


}

fn main() {

    let channel = reciever::Messages::new();    
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
//=======
//    let resp = client.get("http://httpbin.org/status/404")
//        .send()?;
//    println!("{}", resp.status().as_u16());
//    println!("{:?}", resp.content_length());
//    Ok(())
//>>>>>>> ddf48e714fa81301837f89bcbd2b110910d53b07
}
