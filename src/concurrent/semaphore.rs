use std::{
    ops::Deref,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::{Duration, Instant},
};

struct Semaphore {
    //多加一个变量存储原始的permits值
    mutex: Mutex<u32>, //permits
    condvar: Condvar,//rust 的 Condition variable
}

impl Semaphore {
    fn new(value: u32) -> Self {
        Self {
            mutex: Mutex::new(value),
            condvar: Condvar::new(),
        }
    }
    fn acquire(&self) {
        self.acquire_permits(1);
    }
    fn acquire_permits(&self,permits : u32) {
        let mut guard = self.mutex.lock().unwrap();

        while *guard == 0 { // 使用while而不是if，防止虚假唤醒
            println!("waiting...");
            //这里可以理解和java的 condition那样 需要在monitor锁(synchronized{})上/AQS acquire后等待(语言层面的状态判断),而不能直接wait等待.
            guard = self.condvar.wait(guard).unwrap(); 
            
        }
        println!("accquire success");
        //TODO 这里需要保证  permits 小于等于 guard的值;这里先处理 要考虑返回类型
        *guard -= std::cmp::min(permits,*guard);
    }
    // fn accquire_timeout(&self, duration: Duration) -> bool {
        
    // }
    fn release(&self) {
        let mut guard = self.mutex.lock().unwrap();
        *guard += 1;
        //self.condvar.notify_all();
        self.condvar.notify_one();
    }
    fn release_permits(&self, permits: u32) {
        let mut guard = self.mutex.lock().unwrap();
        //保证返回的permits要小于等于原始的permits值
        *guard += permits;
        self.condvar.notify_all();
    }
    
}

#[test]
fn test_semaphore() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut handlers = vec![];

    let start_time = Instant::now();

    for id in 0..12 {
        let thread_id = id;
        let semaphore = Arc::clone(&semaphore);

        let handler = thread::spawn( move || {
            //println!("thread: {} ready", thread_id);
            semaphore.acquire();//这里semaphore是Arc会自动解引用!
            println!("thread: {} running", thread_id);
            thread::sleep(Duration::from_secs(1));

            semaphore.release();
            println!("thread: {} end", thread_id)
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }
    let elapsed = start_time.elapsed();
    println!("time took {:?}", elapsed);

}
