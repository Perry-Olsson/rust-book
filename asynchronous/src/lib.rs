use trpl::Html;
use std::time::Duration;
use std::pin::{pin, Pin};
use std::thread;
use std::time::Instant;

pub async fn run() {
    benchmark_sleep_vs_yield().await
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
