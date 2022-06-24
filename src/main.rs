use futures::future::Future;
use paho_mqtt as mqtt;
use futures::FutureExt;
use paho_mqtt::DeliveryToken;
use std::process;
/// Simple program to greet a person
use clap::{Parser, Subcommand};
extern crate redis;
use futures::executor::block_on;
use std::time::Duration;
use futures::StreamExt;

const TOPICS: &[&str] = &["test", "hello"];
const QOS: &[i32] = &[1, 1];

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
    Subscribe{
        #[clap(value_parser)]
        topic: String,
        #[clap(value_parser)]
        qos: i32,
    },
 }


fn main() {
    let args = Args::parse();

    let url = args.url;

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(url.clone())
        .client_id("rust_async_subscribe")
        .finalize();

    let mut client = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    

    match &args.command {
        Commands::Publish { topic, payload, qos } => {
            let msg = mqtt::Message::new(topic, payload.as_str(), qos.clone());
            if let Err(err) = block_on(async {
                // Connect with default options and wait for it to complete or fail
                println!("Connecting to the MQTT server");
                client.connect(None).await?;
        
                // Create a message and publish it
                println!("Publishing a message on the topic '{}'", topic);
                client.publish(msg).await?;
        
                // Disconnect from the broker
                println!("Disconnecting");
                client.disconnect(None).await?;
        
                Ok::<(), mqtt::Error>(())
            }) {
                eprintln!("{}", err);
            }
        },
        Commands::Subscribe { topic, qos } => {
            if let Err(err) = block_on(async {
                // Get message stream before connecting.
                let mut strm = client.get_stream(25);
        
                // Define the set of options for the connection
                let lwt = mqtt::Message::new("test", "Async subscriber lost connection", mqtt::QOS_1);
        
                let conn_opts = mqtt::ConnectOptionsBuilder::new()
                    .keep_alive_interval(Duration::from_secs(30))
                    .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
                    .clean_session(false)
                    .will_message(lwt)
                    .finalize();
        
                // Make the connection to the broker
                println!("Connecting to the MQTT server...");
                client.connect(conn_opts).await?;
        
                println!("Subscribing to topic: {:?}", topic);
                client.subscribe(topic, qos.clone()).await?;
        
                // Just loop on incoming messages.
                println!("Waiting for messages...");
        
                // Note that we're not providing a way to cleanly shut down and
                // disconnect. Therefore, when you kill this app (with a ^C or
                // whatever) the server will get an unexpected drop and then
                // should emit the LWT message.
        
                while let Some(msg_opt) = strm.next().await {
                    if let Some(msg) = msg_opt {
                        println!("{}", msg);
                    }
                    else {
                        // A "None" means we were disconnected. Try to reconnect...
                        println!("Lost connection. Attempting reconnect.");
                        while let Err(err) = client.reconnect().await {
                            println!("Error reconnecting: {}", err);
                            // For tokio use: tokio::time::delay_for()
                            async_std::task::sleep(Duration::from_millis(1000)).await;
                        }
                    }
                }
        
                // Explicit return type for the async block
                Ok::<(), mqtt::Error>(())
            }) {
                eprintln!("{}", err);
            }
        }
    }

    // Disconnect from the broker
    //let tok = client.disconnect(mqtt::DisconnectOptions::new());
    //tok.wait().unwrap();

    /*let token = publish("topic".to_string(), "payload".to_string(), 0, client);
    println!("Publishing to topic {}", "topic");
    token.wait().unwrap();
    // Wait for the async operation to complete.
    */

    
}