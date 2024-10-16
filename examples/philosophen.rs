use std::sync::{Arc, Mutex};

use rand::{rngs::ThreadRng, thread_rng};
use rand_distr::{Distribution, Exp};

fn main() {
    let forks = [
        Arc::new(Mutex::new(())),
        Arc::new(Mutex::new(())),
        Arc::new(Mutex::new(())),
    ];
    let forks1 = forks.clone();
    let forks2 = forks.clone();
    let thread0 = std::thread::spawn(move || {
        philosoph(&forks[0], &forks[1]);
    });
    let thread1 = std::thread::spawn(move || {
        philosoph(&forks1[1], &forks1[2]);
    });

    let thread2 = std::thread::spawn(move || {
        philosoph(&forks2[0], &forks2[1]);
    });
    thread0.join().unwrap();
    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn wait_exp(rng: &mut ThreadRng) {
    std::thread::sleep(std::time::Duration::from_secs_f64(
        Exp::new(1.5).unwrap().sample(rng),
    ));
}

fn philosoph(left: &Mutex<()>, right: &Mutex<()>) {
    let mut rng = thread_rng();

    for _ in 0..10 {
        // thinking
        wait_exp(&mut rng);

        {
            // hungry
            let left = left.lock().unwrap();

            let right = right.lock().unwrap();

            // eating
            wait_exp(&mut rng);
        }
    }
}
