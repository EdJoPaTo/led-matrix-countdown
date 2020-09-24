use crate::timeloop::TIMEFORMAT;
use chrono::Local;

mod cli;
mod mqtt;
mod timeloop;

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(mqtt_and_http_matches) = matches
        .subcommand_matches("mqtt")
        .or_else(|| matches.subcommand_matches("http"))
    {
        let verbose = mqtt_and_http_matches.is_present("verbose");

        let start = mqtt_and_http_matches
            .value_of("starttime")
            .and_then(cli::time_string_to_date_time)
            .expect("starttime could not be read from the command line");

        let end = mqtt_and_http_matches
            .value_of("endtime")
            .and_then(cli::time_string_to_date_time)
            .expect("endtime could not be read from the command line");

        let end_text = mqtt_and_http_matches
            .value_of("end text")
            .expect("end text could not be read from command line");

        let now = Local::now();
        println!("Now:   {}", now.format(TIMEFORMAT));
        println!("Start: {}", start.format(TIMEFORMAT));
        println!("End:   {}", end.format(TIMEFORMAT));

        assert!(
            end.timestamp() - start.timestamp() > 0,
            "endtime has to be after starttime"
        );
        assert!(
            end.timestamp() - now.timestamp() > 0,
            "endtime has to be in the future"
        );

        if let Some(http_matches) = matches.subcommand_matches("http") {
            let server = http_matches
                .value_of("HTTP Server")
                .expect("HTTP Server could not be read from command line");

            assert!(server.starts_with("http"));
            assert!(server.ends_with('/'));

            let client = reqwest::blocking::Client::new();

            timeloop::timeloop(start, end, end_text, verbose, |topic, text| {
                let url = match topic {
                    timeloop::Topic::Hue => format!("{}hue", server),
                    timeloop::Topic::Sat => format!("{}sat", server),
                    timeloop::Topic::Text => format!("{}text", server),
                };

                client
                    .post(&url)
                    .body(text.to_owned())
                    .send()
                    .expect("failed to publish via http");
            });
        }

        if let Some(mqtt_matches) = matches.subcommand_matches("mqtt") {
            let mqtt_server = mqtt_matches
                .value_of("MQTT Server")
                .expect("MQTT Server could not be read from command line");

            let mqtt_base_topic = mqtt_matches
                .value_of("MQTT Base Topic")
                .expect("MQTT Base Topic could not be read from command line");

            let mqtt_qos: i32 = mqtt_matches
                .value_of("MQTT QoS")
                .and_then(|s| s.parse::<i32>().ok())
                .expect("MQTT QoS could not be read from command line. Make sure its 0, 1 or 2");

            let mqtt_file_persistence = mqtt_matches.is_present("MQTT File persistence");

            let mqtt_client = mqtt::connect(mqtt_server, mqtt_file_persistence)
                .expect("failed to connect to MQTT server");

            timeloop::timeloop(start, end, end_text, verbose, |topic, text| {
                let topic_string = match topic {
                    timeloop::Topic::Hue => format!("{}/set/hue", mqtt_base_topic),
                    timeloop::Topic::Sat => format!("{}/set/sat", mqtt_base_topic),
                    timeloop::Topic::Text => format!("{}/set/text", mqtt_base_topic),
                };

                mqtt::publish(&mqtt_client, &topic_string, text, mqtt_qos)
                    .expect("failed to publish to mqtt")
            });
        }
    }
}
