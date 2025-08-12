use trpl::{Either, Html, Stream, StreamExt, ReceiverStream};
use std::time::Duration;
use std::pin::{pin, Pin};
use std::thread;
use std::time::Instant;

pub async fn run() {
    interval_and_message_stream().await
}

async fn interval_and_message_stream() {
    let messages = get_messages().timeout(Duration::from_millis(200));
    let intervals = get_intervals()
        .map(|count| format!("Interval: {count}"))
        .throttle(Duration::from_millis(100))
        .timeout(Duration::from_secs(10));
    let mut stream = pin!(messages.merge(intervals).take(20));
    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => println!("{message}"),
            Err(reason) => eprintln!("Error with reason {}", reason)
        }
    }
}

async fn message_stream() {
    let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

    while let Some(result) = messages.next().await {
        match result {
            Ok(message) => println!("{message}"),
            Err(reason) => eprintln!("Error with reason {}", reason)
        }
    }
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(e) = tx.send(count) {
                eprintln!("Unable to send interval {}. Error: {}", count, e);
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep= if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;
            if let Err(e) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Unable to send message {}. error: {}", message, e);
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

async fn stream_example() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let itr = numbers.iter().map(|n| n*2);
    let stream = trpl::stream_from_iter(itr);
    let mut filtered = stream.filter(|n| n % 3 == 0 || n % 5 == 0);

    while let Some(value) = filtered.next().await {
        println!("{}", value);
    }
}


async fn timeout_example() {
    let slow = async {
        trpl::sleep(Duration::from_millis(100)).await;
        "I finished"
    };

    match timeout(slow, Duration::from_millis(200)).await {
        Ok(message) => println!("Succeeded with '{message}'"),
        Err(duration) => {
            println!("Failed after {} seconds", duration.as_secs())
        }
    }
}

async fn timeout<F: Future>(runnable: F, duration: Duration) -> Result<F::Output, Duration> {
    match trpl::race(runnable, trpl::sleep(duration)).await {
        Either::Left(v) => Result::Ok(v),
        Either::Right(_) => Result::Err(duration)
    }
}

async fn benchmark_sleep_vs_yield() {
    let one_ns = Duration::from_nanos(1);
    let start = Instant::now();
    async {
        for _ in 1..1000 {
            trpl::sleep(one_ns).await;
        }
    }
    .await;
    let time = Instant::now() - start;
    println!(
        "'sleep' version finished after {} seconds.",
        time.as_secs_f32()
    );

    let start = Instant::now();
    async {
        for _ in 1..1000 {
            trpl::yield_now().await;
        }
    }
    .await;
    let time = Instant::now() - start;
    println!(
        "'yield' version finished after {} seconds.",
        time.as_secs_f32()
    );
}

async fn future_starving() {
    let a = async {
        println!("'a' started.");
        slow("a", 30);
        trpl::yield_now().await;
        slow("a", 10);
        trpl::yield_now().await;
        slow("a", 20);
        trpl::yield_now().await;
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        trpl::yield_now().await;
        slow("b", 10);
        trpl::yield_now().await;
        slow("b", 15);
        trpl::yield_now().await;
        slow("b", 350);
        trpl::yield_now().await;
        println!("'b' finished.");
    };

    trpl::race(a, b).await;
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

async fn message_passing() {
    let (tx, mut rx) = trpl::channel();
    let tx2 = tx.clone();
    let tx_fut = pin!(async move {

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    let rx_fut = pin!(async {
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        };
    });

    let tx_fut2 = pin!(async move {
        let vals = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("the2"),
            String::from("future2"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            trpl::sleep(Duration::from_millis(1500)).await
        }
    });
    let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut, rx_fut, tx_fut2];
    trpl::join_all(futures).await;
}

async fn async_counter() {
    let fut_1 = async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };
    
    let fut_2 = async {
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await
        }
    };

    trpl::join(fut_1, fut_2).await;
}

async fn race_urls() {
    let args: Vec<String> = std::env::args().collect();
    let title_fut_1 = page_title(&args[1]);
    let title_fut_2 = page_title(&args[2]);
    let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
        trpl::Either::Left(left) => left,
        trpl::Either::Right(right) => right
    };

    println!("Url returned first: {}", url);
    match maybe_title {
        Some(title) => println!("The title was: {}", title),
        None => println!("No title found")
    }
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}
