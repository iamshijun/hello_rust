use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

struct CyclicBarrier {
    generation: Mutex<Generation>, //当前的generation   
    condvar: Condvar,
    parties: u32, //期望的线程数量
}
struct Generation {
    val: u32,//generation 编号
    count: u32,//还在等待的线程数量
    broken: bool,//是否被中断了
}
impl Clone for Generation {
    fn clone(&self) -> Self {
        Self {
            val: self.val,
            count: self.count,
            broken: self.broken,
        }
    }
}

impl CyclicBarrier {
    fn new(parties: u32) -> Self {
        Self {
            generation: Mutex::new(Generation {
                val: 1, //generation 编号从1开始
                count: parties,
                broken: false,
            }),
            condvar: Condvar::new(),
            parties,
        }
    }
    fn wait_timeout(&self,timeout:Duration) -> Result<u32,()>{
        todo!()
    }
    fn wait(&self) -> u32 {
        let mut lock = self.generation.lock().unwrap();

        lock.count -= 1;
        let gen = lock.clone(); //cached

        if lock.count == 0 {
            //所有线程都已经到达了
            //重置generation
            lock.count = self.parties;
            lock.broken = false;
            lock.val += 1; // next generation

            self.condvar.notify_all(); //唤醒所有线程
            return gen.val;
        }
        //notify过来 进入到新的generation了 要使用旧的用gen
        match self.condvar.wait(lock) {
            Err(mut e) => {
                e.get_mut().broken = true;
                self.condvar.notify_all();
                eprintln!("wait error:{}", e);
                return 0;
            }
            _ => {} //被唤醒了
        }
        //某个线程被中断了
        return gen.val;
        //check if generation broken?
    }
    fn get_parties(&self) -> u32 {
        self.parties
    }
    fn get_count(&self) -> u32 {
        self.generation.lock().unwrap().count
    }
}

#[test]
fn test_cyclibarrier() {
    let barrier = Arc::new(CyclicBarrier::new(3));

    let mut handles = vec![];
    for idx in 0..6 {
        let thread_id = idx;
        let barrier = Arc::clone(&barrier);
        //thread::sleep(Duration::from_secs(idx as u64));

        let handler = thread::spawn(move || {
            println!("thread {} is waiting", thread_id);

            let generation = barrier.wait();
            println!("thread {} done on generation:{}", thread_id, generation);
        });
        handles.push(handler);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("all threads done");
}
