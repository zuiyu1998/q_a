use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

#[derive(Debug, Default, Clone)]
pub struct State {
    count: Arc<Mutex<usize>>,
}

impl State {
    pub fn computed(&self) {
        println!("computed");
        let mut count = self.count.lock().unwrap();

        sleep(Duration::from_secs(2));

        *count += 1;
    }

    pub fn print(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;

        println!("{}", count);
    }
}

fn main() {
    let state = State::default();

    let state_clone = state.clone();
    thread::spawn(move || {
        state_clone.computed();
    });


    let state_clone = state.clone();
    thread::spawn(move || {
        state_clone.computed();
    });

    for _ in 0..1000 {
        state.print();
    }

    sleep(Duration::from_secs(3));
}
