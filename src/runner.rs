use super::engine::Engine;
use super::plan::Plan;
use super::message::Message;
use super::stats::Fact;
use std::{thread, sync::mpsc::Sender};


pub struct Runner {
    handles: Vec<thread::JoinHandle<()>>,
}

impl Runner {

    pub fn start(plan: Plan, engine: &Engine, collector: &Sender<Message<Fact>>) -> Runner {
        Runner {
            handles: plan.distribute()
                .into_iter()
                .map(|requests| {
                    let collector = collector.clone();
                    let eng = engine.clone();
                    print!(".");
                    thread::spawn(move || Runner::run(requests, &eng, &collector))
                })
                .collect()
        }
    }

    pub fn join(self) {
        self.handles
        .into_iter()
        .for_each(|handle| {
            handle.join().expect("Sending thread to finish");
        });
    }

    fn run(requests: usize, engine: &Engine, collector: &Sender<Message<Fact>>) {
        engine.run(requests, |fact| {
            collector
                .send(Message::Body(fact))
                .expect("to send the fact correctly");
        });
        collector
            .send(Message::EOF)
            .expect("to send None correctly");
    }
}
