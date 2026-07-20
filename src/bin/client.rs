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
let mut client =
    Client::connect("127.0.0.1:7878").await;

client.send(
    Packet::Join {
        name: "Sara".to_string(),
    }
).await;
}