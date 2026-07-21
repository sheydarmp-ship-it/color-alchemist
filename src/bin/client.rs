use color_alchemist::multiplayer::client::Client;
use color_alchemist::multiplayer::protocol::Packet;

#[tokio::main]
async fn main() {

    let mut client =
        Client::connect("127.0.0.1:7878");

    client.send(

        Packet::Join {

            name: "Ali".to_string(),

        }

    );
    client.send(
    Packet::Ready,
    );

client.send(
    Packet::Guess {
        r: 120,
        g: 80,
        b: 220,
    }
);
let packet = client.receive();

println!("Server replied: {:?}", packet);
let packet =
    client.receive();

match packet {

    Packet::RoundResult {
        accuracy,
        win:_,
    } => {

        println!(
            "Server says: {:.2}%",
            accuracy
        );

    }

    _ => {}
}
}