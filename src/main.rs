use std::{
    sync::{mpsc::channel, Arc, RwLock},
    thread::{self, sleep},
    time::Duration,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn work1(id: u64) -> u64 {
    let mut res = 0;
    for i in 0..10 {
        sleep(Duration::from_millis(10 * i * id));
        res += id * i;
        println!("work {id} : {i}");
    }
    res
}

fn main() {
    let (tx, rx) = channel();
    let th = thread::spawn(move || {
        let output: Arc<RwLock<Vec<u64>>> = Arc::new(RwLock::new(Vec::new()));
        let result = output.clone();
        // rayon::scope(move |_| {
        (0..10u64).into_par_iter().for_each(move |i| {
            let v = i;
            let output = output.clone();
            let r = work1(v);
            if let Ok(mut output) = output.write() {
                output.push(r);
            };
        });
        // });
        if let Ok(output) = result.read() {
            _ = tx.send(Some(output.iter().sum::<u64>()));
            // println!("result: {}", output.iter().sum::<u64>());
        } else {
            _ = tx.send(None);
        };
    });

    match th.join() {
        Ok(_) => println!("done."),
        Err(e) => eprintln!("ERROR: {:?}", e),
    }

    match rx.try_recv() {
        Ok(Some(result)) => println!("sum: {result}"),
        Ok(None) => println!("None"),
        Err(e) => eprint!("ERROR: {:?}", e),
    }
}
