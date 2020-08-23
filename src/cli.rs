use chrono::{DateTime, Local, NaiveTime};
use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    let generic_args = [
        Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Show each time tick on stdout"),
        Arg::with_name("starttime")
            .required(true)
            .value_name("STARTTIME")
            .help("Start time of the Meeting. From then the remaining time is published"),
        Arg::with_name("endtime")
            .required(true)
            .value_name("ENDTIME")
            .help("End time of the Meeting. Until then the remaining time is published."),
        Arg::with_name("end text")
            .long("end-text")
            .value_name("STRING")
            .takes_value(true)
            .help("Text which is displayed when the countdown ends.")
            .default_value("THE END \\o/"),
    ];

    App::new("LED Matrix Countdown")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Utility to send the remaining time of something (e.g. a meeting) via MQTT or HTTP to a small display.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("http")
            .about("Send the remaining time of something (e.g. a meeting) via HTTP to a small display.")
            .args(&generic_args)
            .arg(Arg::with_name("HTTP Server")
                .short("s")
                .long("server")
                .value_name("URI")
                .takes_value(true)
                .help("Specify the HTTP Server")
                .default_value("http://esp-matrix/")
            )
        )
        .subcommand(SubCommand::with_name("mqtt")
            .about("Send the remaining time of something (e.g. a meeting) via MQTT to a small display.")
            .args(&generic_args)
            .arg(Arg::with_name("MQTT Server")
                .short("s")
                .long("mqtt-server")
                .value_name("URI")
                .takes_value(true)
                .help("Specify the MQTT Server")
                .default_value("tcp://localhost:1883")
            )
            .arg(Arg::with_name("MQTT Base Topic")
                .short("b")
                .long("base-topic")
                .value_name("STRING")
                .takes_value(true)
                .help("MQTT Root Topic of the matrix to publish to")
                .default_value("espMatrix")
            )
            .arg(Arg::with_name("MQTT QoS")
                .short("q")
                .long("qos")
                .value_name("INT")
                .takes_value(true)
                .help("Define the Quality of Service for the MQTT Messages (0, 1 or 2)")
                .default_value("2")
            )
            .arg(Arg::with_name("MQTT File persistence")
                .short("p")
                .long("file-persistence")
                .help("When enabled the MQTT persistence is done via files within the working directory. Enabling this is more reliable.")
            )
        )
}

pub fn time_string_to_date_time(timestring: &str) -> Option<DateTime<Local>> {
    let today = chrono::offset::Local::now().date();
    let fmt = if timestring.len() > 5 {
        "%H:%M:%S"
    } else {
        "%H:%M"
    };
    NaiveTime::parse_from_str(timestring, fmt)
        .ok()
        .and_then(|t| today.and_time(t))
}
