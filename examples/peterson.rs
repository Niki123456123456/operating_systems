use rand::{rngs::ThreadRng, thread_rng};
use rand_distr::{Distribution, Exp};

fn main() {
    let mut variables = Variables {
        thread_states: [State::Default, State::Default],
        turn: 0,
    };

    let v0 = unsafe { &mut *((&mut variables) as *mut Variables) };
    let v1 = unsafe { &mut *((&mut variables) as *mut Variables) };

    let thread0 = std::thread::spawn(|| {
        peterson(v0, 0, 1);
    });
    let thread1 = std::thread::spawn(|| {
        peterson(v1, 1, 0);
    });

    thread0.join().unwrap();
    thread1.join().unwrap();
}

fn wait_exp(rng: &mut ThreadRng) {
    std::thread::sleep(std::time::Duration::from_secs_f64(
        Exp::new(1.5).unwrap().sample(rng),
    ));
}

fn peterson(var: &mut Variables, my_id: usize, other_id: usize) {
    let mut rng = thread_rng();

    for _ in 0..10 {
        // non critical section
        wait_exp(&mut rng);

        println!("requests critical {}", my_id);
        var.thread_states[my_id] = State::WantEnterCriticalSection;
        var.turn = other_id;
        // wait
        loop {
            if var.thread_states[other_id] == State::Default || var.turn == my_id {
                break;
            }
        }
        
        println!("enter critical {}", my_id);
        // critical section
        wait_exp(&mut rng);

        var.thread_states[my_id] = State::Default;

        println!("exit critical {}", my_id);
    }
}

struct Variables {
    thread_states: [State; 2],
    turn: usize,
}

#[derive(PartialEq)]
enum State {
    WantEnterCriticalSection,
    Default,
}
