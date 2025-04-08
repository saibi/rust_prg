use std::sync::{Arc, Mutex};
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

fn simple_timer_test() {
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

trait MyTimerEvent: Send + Sync {
    fn event_handler(&self, id: u32);
}

struct MyTimerEntry {
    id: u32,
    event: Arc<dyn MyTimerEvent>,
    now: std::time::Instant,
    delay: std::time::Duration,
}

struct MyTimer {
    id_counter: u32,
    events: Arc<Mutex<Vec<MyTimerEntry>>>,
}

impl MyTimer {
    fn new() -> Self {
        let events = Arc::new(Mutex::new(Vec::<MyTimerEntry>::new()));
        let events_clone = events.clone();

        std::thread::spawn(move || {
            loop {
                let mut events_to_remove = Vec::new();

                {
                    let mut events = events_clone.lock().unwrap();

                    for (index, entry) in events.iter().enumerate() {
                        if entry.now.elapsed() >= entry.delay {
                            entry.event.event_handler(entry.id);
                            events_to_remove.push(index);
                        }
                    }

                    events_to_remove.sort_by(|a, b| b.cmp(a));

                    for index in events_to_remove {
                        events.remove(index);
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        MyTimer {
            id_counter: 1,
            events,
        }
    }

    fn start_timer(&mut self, delay: std::time::Duration, event: Arc<dyn MyTimerEvent>) -> u32 {
        let now = std::time::Instant::now();
        let id = self.id_counter;
        self.id_counter += 1;
        let mut events = self.events.lock().unwrap();
        events.push(MyTimerEntry {
            id,
            event,
            now,
            delay,
        });
        id
    }
}

#[derive(Clone)]
struct MyTimerEventImpl {
    name: String,
}

impl MyTimerEvent for MyTimerEventImpl {
    fn event_handler(&self, id: u32) {
        println!("이벤트 {} 실행됨!, {}", id, self.name);
    }
}

impl MyTimerEventImpl {
    fn new(name: String) -> Self {
        MyTimerEventImpl { name }
    }

    fn hello(&self) {
        println!("hello, {}", self.name);
    }
}

fn main() {
    let mut timer = MyTimer::new();

    let test = Arc::new(MyTimerEventImpl::new("kim".to_string()));

    let id = timer.start_timer(Duration::from_secs(3), test.clone());
    println!("타이머 시작됨! id: {}", id);

    test.hello();
    println!("메인 스레드 10초 대기");
    std::thread::sleep(Duration::from_secs(10));
}
