use std::{cmp, thread, sync::mpsc::{channel, Receiver, Sender}};
use crate::message::Message;
use crate::plan::Plan;


pub fn start<T>(plan: Plan) -> (Sender<Message<T>>, thread::JoinHandle<Vec<T>>)
where
    T: 'static + Send,
{
    let (sender, reciever) = channel::<Message<T>>();

    (sender, thread::spawn(move || collector(&reciever, plan)))
}

fn collector<T>(receiver: &Receiver<Message<T>>, plan: Plan) -> Vec<T>
where
    T: 'static + Send,
{
    let chunk_size = cmp::max(plan.requests() / 10, 1);
    let mut messages: Vec<T> = Vec::with_capacity(plan.requests());
    let mut eof_count = 0;

    while eof_count < plan.distribute().len() {
        match receiver.recv().expect("to receive correctly.") {
            Message::Body(message) => {
                messages.push(message);
                if (messages.len() % chunk_size) == 0 {
                    println!("{} requests completed.", chunk_size)
                }
            },
            Message::EOF => eof_count += 1,
        }
    }
    messages
}

#[cfg(test)]
mod message_collection_tests {
    use super::*;

    #[test]
    fn test_ends_when_all_none_are_recieved() {
        let plan = Plan::new(1, 4);
        let (tx, handle) = start::<usize>(plan);
        for i in 0..4 {
            let _ = tx.send(Message::EOF);
        }
        assert_eq!(handle.join().unwrap(), Vec::<usize>::new());
    } 
    
    #[test]
    fn test_collects_all_data_recieved() {
        let plan = Plan::new(1, 5);
        let (tx, handle) = start::<usize>(plan);
        for i in 0..5 {
            let _ = tx.send(Message::Body(i));
        }
        let _ = tx.send(Message::EOF);
        assert_eq!(handle.join().unwrap(), vec![0, 1, 2, 3, 4]);
    } 
}
