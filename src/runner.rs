use std::thread;
use super::engine::Engine;
use super::plan::Plan;


pub struct Runner {
    handles: Vec<thread::JoinHandle<()>>,
}

impl Runner {

    pub fn start(plan: Plan, engine: &Engine) -> Runner {
        Runner {
            handles: plan.distribute()
                .into_iter()
                .map(|requests| {
                    let eng = engine.clone();
                    print!(".");
                    thread::spawn(move || Runner::run(requests, &eng))
                })
                .collect()
        }
    }

    pub fn join(self) {
        self.handles
        .into_iter()
        .for_each(|handle| {
            handle.join().unwrap();
        });
    }

    fn run(requests: usize, engine: &Engine) {
        engine.run(requests);
    }
}
