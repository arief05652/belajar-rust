use anyhow::Result;
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    task,
    time::{Duration, sleep},
};

/////////////////////////////////////////////////////////////////
// ENTRY POINT
/////////////////////////////////////////////////////////////////

// #[tokio::main] membuat async runtime otomatis.
// Tanpa ini, async fn main tidak bisa jalan.
#[tokio::main]
async fn main() -> Result<()> {
    println!("=== TOKIO FULL DEMO START ===");

    basic_sleep().await?;
    concurrent_tasks().await?;
    channel_demo().await?;
    async_file_demo().await?;
    spawn_blocking_demo().await?;
    // tcp_server_demo().await?;

    println!("=== DONE ===");
    Ok(())
}

/////////////////////////////////////////////////////////////////
// 1️⃣ BASIC ASYNC + SLEEP
/////////////////////////////////////////////////////////////////

// Contoh async sederhana dengan non-blocking sleep.
async fn basic_sleep() -> Result<()> {
    println!("\n[1] Basic Sleep Start");

    // sleep() tidak mem-block thread.
    // Dia yield control ke runtime.
    sleep(Duration::from_secs(1)).await;

    println!("[1] Basic Sleep End");
    Ok(())
}

/////////////////////////////////////////////////////////////////
// 2️⃣ CONCURRENT TASKS (spawn)
/////////////////////////////////////////////////////////////////

async fn concurrent_tasks() -> Result<()> {
    println!("\n[2] Concurrent Tasks Start");

    // spawn membuat task baru di runtime scheduler
    // kalau mau proses data didalam task harus pake `move`
    // keyword dan bisa juga pakai Arc+mutex atau mpsc
    let t1 = task::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("Task 1 selesai");
        10
    });

    let t2 = task::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 2 selesai");
        20
    });

    // .await pada JoinHandle
    // return type: Result<T, JoinError>
    let r1 = t1.await?;
    let r2 = t2.await?;

    println!("Hasil task: {} + {} = {}", r1, r2, r1 + r2);

    Ok(())
}

/////////////////////////////////////////////////////////////////
// 3️⃣ CHANNEL (Message Passing)
/////////////////////////////////////////////////////////////////

async fn channel_demo() -> Result<()> {
    println!("\n[3] Channel Demo Start");

    // mpsc = multi producer single consumer
    let (tx, mut rx) = mpsc::channel(5);

    // Producer task
    tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(i).await.unwrap();
            sleep(Duration::from_millis(300)).await;
        }
    });

    // Consumer loop
    while let Some(value) = rx.recv().await {
        println!("Received: {value}");
    }

    println!("[3] Channel Demo End");
    Ok(())
}

/////////////////////////////////////////////////////////////////
// 4️⃣ ASYNC FILE I/O
/////////////////////////////////////////////////////////////////

async fn async_file_demo() -> Result<()> {
    println!("\n[4] Async File Demo Start");

    // Write file secara async
    fs::write("demo.txt", "Hello Tokio Async File!").await?;

    // Read file secara async
    let content = fs::read_to_string("demo.txt").await?;

    println!("File Content: {content}");

    Ok(())
}

/////////////////////////////////////////////////////////////////
// 5️⃣ SPAWN BLOCKING (Untuk operasi berat / blocking)
/////////////////////////////////////////////////////////////////

async fn spawn_blocking_demo() -> Result<()> {
    println!("\n[5] Spawn Blocking Demo Start");

    // spawn_blocking dipakai untuk operasi CPU heavy
    // atau library yang blocking (misalnya hashing besar)
    let result = task::spawn_blocking(|| {
        // Simulasi kerja berat
        std::thread::sleep(std::time::Duration::from_secs(2));
        99
    })
    .await?;

    println!("Blocking result: {result}");

    Ok(())
}

/////////////////////////////////////////////////////////////////
// 6️⃣ SIMPLE TCP ECHO SERVER (OPTIONAL DEMO)
/////////////////////////////////////////////////////////////////

// Tidak dipanggil di main supaya tidak infinite loop,
// tapi ini contoh server async sederhana.
#[allow(dead_code)]
async fn tcp_server_demo() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        // Spawn task baru untuk tiap koneksi
        tokio::spawn(handle_client(socket));
    }
}

// Handler client async
async fn handle_client(mut socket: TcpStream) -> Result<()> {
    println!("client baru");
    let mut buf = [0; 1024];

    let n = socket.read(&mut buf).await?;

    socket.write_all(&buf[..n]).await?;

    Ok(())
}
