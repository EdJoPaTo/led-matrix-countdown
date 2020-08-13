use chrono::offset::Local;

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

        if end.timestamp() - start.timestamp() < 0 {
            panic!("endtime has to be after starttime")
        }

        let now = Local::now();
        if end.timestamp() - now.timestamp() < 0 {
            panic!("endtime has to be in the future")
        }

        println!("Now:   {}", now.time());
        println!("Start: {}", start.time());
        println!("End:   {}", end.time());

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
