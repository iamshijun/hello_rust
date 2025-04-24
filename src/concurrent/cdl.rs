use std::{sync::{Arc, Condvar, Mutex}, thread, time::{Duration, Instant}};



struct CountDownLatch {
    mutex: Mutex<u32>,//state
    condvar: Condvar,
}

impl CountDownLatch {
    fn new(count: u32) -> Self {
        Self {
            mutex: Mutex::new(count),
            condvar: Condvar::new(),
        }
    }    
    fn count_down(&self) {
        let mut lock = self.mutex.lock().unwrap();
        if *lock == 0 {
            return;//do  nothing
        }
        *lock -= 1;//release shared
        if *lock == 0 {
            self.condvar.notify_all();//唤醒所有线程
        }
    }
    fn wait(&self) {
        let mut lock = self.mutex.lock().unwrap();
        while *lock != 0 { //等待栏栅被撤去 mutex的值为0
            lock = self.condvar.wait(lock).unwrap();
        }
    }
    //如果超时了 暂时返回false TODO
    fn wait_timeout(&self,timeout:Duration) -> bool {
        let mut lock = self.mutex.lock().unwrap();
        let mut remaing_time = timeout;
        while *lock != 0 { 
            if remaing_time == Duration::from_secs(0)  {
                return false;
            }
            let start  = Instant::now();
            let (g,timeout_result) = 
                self.condvar.wait_timeout(lock, remaing_time).unwrap();
            if timeout_result.timed_out() {
               return false;
            }else { //被唤醒了 但是没有超时
                remaing_time = remaing_time.saturating_sub(start.elapsed());
            }
            lock = g;
        }
       return true;
    }
    fn get_count(&self) -> u32 {
        return *self.mutex.lock().unwrap();
    }
}

#[test]
fn test_cdl() {
    
    let cdl = Arc::new(CountDownLatch::new(2));

    let start_time = Instant::now();
    let mut handlers = vec![];
    for i in 0..3 {
        let cdl = cdl.clone();
        let handler = thread::spawn(move || {
            println!("thread {} start", i);
            cdl.wait();
            println!("thread {} end", i);
        });
        handlers.push(handler);
    }
    
    thread::sleep(Duration::from_secs(2));    
    cdl.count_down();

    thread::sleep(Duration::from_secs(3));    
    cdl.count_down();

    for handler in handlers {
        handler.join().unwrap();
    }
    let time_elapsed = start_time.elapsed();
    println!("time elapsed: {:?}", time_elapsed);//~5s


}