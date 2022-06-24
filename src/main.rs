use futures::future::Future;
use paho_mqtt as mqtt;
use futures::FutureExt;
use paho_mqtt::DeliveryToken;
use std::process;
/// Simple program to greet a person
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    url: String,

    #[clap(subcommand)]
    command: Commands,
 }
 
 #[derive(Subcommand)]
 enum Commands {
    Publish{
        #[clap(value_parser)]
        topic: String,

        #[clap(value_parser)]
        payload: String,

        #[clap(value_parser)]
        qos: i32,
    },
 }

fn publish(msg:mqtt::Message, client:&mqtt::AsyncClient) -> DeliveryToken{
    client.publish(msg.clone())
}

fn main() {
    let args = Args::parse();

    let url = args.url;

    let client = mqtt::AsyncClient::new(url.clone()).unwrap();

    if let Err(e) = client.connect(mqtt::ConnectOptions::new()).wait() {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    println!("Connecting to {} ({})", url.clone(), client.client_id());


    match &args.command {
        Commands::Publish { topic, payload, qos } => {
            let msg = mqtt::Message::new(topic, payload.as_str(), qos.clone());
            let token = publish(msg, &client);
            if let Err(e) = token.wait() {
                println!("Error sending message: {:?}", e);
            }
        }
    }

    // Disconnect from the broker
    let tok = client.disconnect(mqtt::DisconnectOptions::new());
    tok.wait().unwrap();

    /*let token = publish("topic".to_string(), "payload".to_string(), 0, client);
    println!("Publishing to topic {}", "topic");
    token.wait().unwrap();
    // Wait for the async operation to complete.
    */

    
}