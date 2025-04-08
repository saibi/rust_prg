use std::thread;
use std::time::Duration;

pub struct TimerEvent;

impl TimerEvent {
    pub fn new() -> Self {
        TimerEvent
    }

    pub fn add_event<F>(&self, delay_secs: u64, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(delay_secs));
            func();
        });
    }
}

fn main() {
    let timer = TimerEvent::new();

    timer.add_event(3, || {
        println!("3초 후 실행됨!");
    });

    timer.add_event(5, || {
        println!("5초 후 실행됨!");
    });

    println!("타이머 설정 완료");

    // 메인 스레드가 너무 빨리 끝나지 않게 대기
    thread::sleep(Duration::from_secs(6));
}
