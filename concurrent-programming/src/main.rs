use std::{sync::{mpsc, Arc, Mutex}, thread};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    thread_demo();
    sync_mpsc();
    async_await().await;
    mutex_rwlock();
}


// 使用thread实现多线程
fn thread_demo() {
    let mut handles = vec![];

    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("线程{}正在执行！", i);
        });
        handles.push(handle)
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().expect("线程发生错误！");
    }
    println!("线程执行完毕");
}

// 使用消息传递实现并发
fn sync_mpsc() {
    // 创建通道
    let (tx, rx) = mpsc::channel();
    // 创建发送线程
    thread::spawn(move || {
        let vals = vec!["消息1", "消息2", "消息3"];
        for val in vals {
            tx.send(val).expect("无法发送消息！");
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 在主线程中接收并打印消息
    for received in rx {
        println!("接收到:{}", received);
    }
}

async fn async_await(){
    let task1 = async{
        println!("开始任务1");
        sleep(Duration::from_secs(1)).await;
        println!("任务1完成");
    };

    let task2 = async{
        println!("开始任务2");
        sleep(Duration::from_secs(2)).await;
        println!("任务2完成");
    };

    // 同时运行两个异步任务
    tokio::join!(task1, task2);

    print!("所有任务完成")
}

fn mutex_rwlock(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num +=1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().expect("Failed to join a thread");
    }
    println!("Result: {}",counter.lock().unwrap());
}