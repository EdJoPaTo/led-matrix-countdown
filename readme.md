# LED Matrix Countdown

This tool was created in order to learn Rust and to work with [esp-http-neomatrix-text](https://github.com/EdJoPaTo/esp-http-neomatrix-text) and [esp-mqtt-neomatrix-text](https://github.com/EdJoPaTo/esp-mqtt-neomatrix-text).

## Current usecase: Meeting end time

One of these is in the visible webcam area behind me.
This way the matrix can display a countdown until the end of the meeting.
Everyone else can then easily see how much time is left.

## Usage

### MQTT

```
USAGE:
    led-matrix-countdown mqtt [FLAGS] [OPTIONS] <STARTTIME> <ENDTIME>

FLAGS:
    -p, --file-persistence    When enabled the MQTT persistence is done via files within the working directory.
                              Enabling this is more reliable.
    -h, --help                Prints help information
    -V, --version             Prints version information
    -v, --verbose             Show each time tick on stdout

OPTIONS:
    -b, --base-topic <STRING>    MQTT Root Topic of the matrix to publish to [default: espMatrix]
    -q, --qos <INT>              Define the Quality of Service for the MQTT Messages (0, 1 or 2) [default: 2]
    -s, --mqtt-server <URI>      Specify the MQTT Server [default: tcp://localhost:1883]
        --end-text <STRING>      Text which is displayed when the countdown ends. [default: THE END \o/]

ARGS:
    <STARTTIME>    Start time of the Meeting. From then the remaining time is published
    <ENDTIME>      End time of the Meeting. Until then the remaining time is published.
```

### HTTP

```
USAGE:
    led-matrix-countdown http [FLAGS] [OPTIONS] <STARTTIME> <ENDTIME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show each time tick on stdout

OPTIONS:
    -s, --server <URI>         Specify the HTTP Server [default: http://esp-matrix/]
        --end-text <STRING>    Text which is displayed when the countdown ends. [default: THE END \o/]

ARGS:
    <STARTTIME>    Start time of the Meeting. From then the remaining time is published
    <ENDTIME>      End time of the Meeting. Until then the remaining time is published.
```
