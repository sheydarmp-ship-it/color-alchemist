use color_alchemist::multiplayer::client::Client;
use color_alchemist::multiplayer::protocol::Packet;

#[tokio::main]
async fn main() {

    let mut client =
        Client::connect("127.0.0.1:7878")
            .await;

    client.send(

        Packet::Join {

            name: "Ali".to_string(),

        }

    ).await;

client.send(
    Packet::Guess {
        r: 120,
        g: 80,
        b: 220,
    }
).await;
}