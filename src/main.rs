extern crate rmp_serde as rmps;

use rumqtt::{MqttClient, MqttOptions, QoS};
use rumqtt::client::Notification;
use std::io::{stdin,stdout,Write};
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use rmps::{Serializer, Deserializer};

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: u16 = 1883;

/*** MQTT Structs ***/

#[derive(Serialize, Deserialize, Debug)]
struct GetData {
    table:       String,
    start_ts:    u32,
    end_ts:      u32
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MpdRecordType {
    pub id:         u32,        // Record identifier
    pub datalog:    Vec<u8>,    // Byte array of length 'size'
    pub checksum:   u32,        // CRC-32 checksum of 'datalog'
}

/********************/

fn main() {
    let mqtt_options = MqttOptions::new("LocalClient", SERVER_IP, SERVER_PORT);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    let mut s = String::new();

    loop {
        print!("Please enter some text: ");
        s = String::new();
        let _=stdout().flush();

        // Read text
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        if s == "exit" {
            print!("Quitting.");
            return;
        }

        if s == "delete" {
            let buf = Vec::new();
            mqtt_client.publish("Test-Delete", QoS::AtLeastOnce, false, buf).unwrap();
            continue;
        }

        if s.trim().parse::<u32>().is_ok() {
            let mut buf = Vec::new();
            let mut msg_pack = Serializer::new(&mut buf);
            s.serialize(&mut msg_pack).unwrap();
            mqtt_client.publish("Test-Add", QoS::AtLeastOnce, false, buf).unwrap();
            continue;
        }

        if s == "get data" {
            let data = GetData{table: String::from("levels"), start_ts: 1577916000, end_ts: 1577926800};
            let mut buf = Vec::new();
            let mut msg_pack = Serializer::new(&mut buf);
            data.serialize(&mut msg_pack).unwrap();

            mqtt_client.publish("GetData", QoS::AtLeastOnce, false, buf).unwrap();
            mqtt_client.subscribe("Client", QoS::AtLeastOnce).unwrap();
            // Parse notifications
            for notification in &notifications {
                match notification {
                    Notification::Publish(publish) =>  {
                            // Get payloads
                            let payload = Arc::try_unwrap(publish.payload).unwrap();
                            if Deserialize(payload) {
                                break;
                            }
                        },
                    _ => println!("Received something that's not a publish! {:?}. Ignoring...", notification)
                }
            }

            continue;
        }

        println!("You typed: {}",s);
    }
}

fn Deserialize(payload: Vec<u8>) -> bool {
    let mut count = 0;
    let mut de = Deserializer::new(&payload[..]);
    loop {
        let entry: MpdRecordType = match Deserialize::deserialize(&mut de) {
            Ok(entry) => entry,
            Err(_) => {
                println!("Deserialized {} entries", count);
                break;
            }
        };
        count += 1;
    }

    if count == 0 {
        return true;
    } else {
        return false;
    }
}
