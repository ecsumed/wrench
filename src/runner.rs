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
                     .iter()
                     .map(|n|{
                         thread::spawn(|| {
                         engine.run(*n);
                         })
                     })
                     .collect()
        }
    }

}
