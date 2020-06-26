extern crate rmp_serde as rmps;

use rumqtt::{MqttClient, MqttOptions, QoS};
use std::io::{stdin,stdout,Write};

use serde::{Serialize, Deserialize};
use rmps::{Serializer};

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: u16 = 1883;

/*** MQTT Structs ***/

#[derive(Serialize, Deserialize, Debug)]
struct GetData {
    table:      &'static str,
    start_ts:    u32,
    end_ts:      u32
}

/********************/

fn main() {
    let mqtt_options = MqttOptions::new("LocalDB", SERVER_IP, SERVER_PORT);
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
            let data = GetData{table: "levels", start_ts: 1577916000, end_ts: 1577926800};
            let mut buf = Vec::new();
            let mut msg_pack = Serializer::new(&mut buf);
            data.serialize(&mut msg_pack).unwrap();
            mqtt_client.publish("GetData", QoS::AtLeastOnce, false, buf).unwrap();
            continue;
        }

        println!("You typed: {}",s);
    }
}
