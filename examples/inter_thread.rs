/// Demo of inter-thread communication (with no RPC layer) with a UNIX socket for external clients
use elbus::broker::Broker;
use elbus::client::AsyncClient;
use elbus::QoS;
use std::time::Duration;
use tokio::time::sleep;

const SLEEP_STEP: Duration = Duration::from_secs(1);

#[tokio::main]
async fn main() {
    // create a new broker instance
    let mut broker = Broker::new();
    // spawn unix server for external clients
    broker
        .spawn_unix_server("/tmp/elbus.sock", 8192, Duration::from_secs(5))
        .await
        .unwrap();
    // worker 1 will send to worker2 direct "hello" message
    let mut client1 = broker.register_client("worker.1").unwrap();
    // worker 2 will listen to incoming frames only
    let mut client2 = broker.register_client("worker.2").unwrap();
    // worker 3 will send broadcasts to all workers, an external client with a name "worker.N" can
    // connect the broker via unix socket and receive them as well or send a message to "worker.2"
    // to print it
    let mut client3 = broker.register_client("worker.3").unwrap();
    tokio::spawn(async move {
        loop {
            client1
                .send("worker.2", "hello".as_bytes().into(), QoS::No)
                .await
                .unwrap();
            sleep(SLEEP_STEP).await;
        }
    });
    tokio::spawn(async move {
        loop {
            client3
                .send_broadcast(
                    "worker.*",
                    "this is a broadcast message".as_bytes().into(),
                    QoS::No,
                )
                .await
                .unwrap();
            sleep(SLEEP_STEP).await;
        }
    });
    let rx = client2.take_event_channel().unwrap();
    while let Ok(frame) = rx.recv().await {
        println!(
            "{}: {}",
            frame.sender(),
            std::str::from_utf8(frame.payload()).unwrap()
        );
    }
}