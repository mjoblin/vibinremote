# `vibinremote`

An application for interacting with the [Vibin] music streaming controller with keyboard key
presses. `vibinremote` intercepts keyboard key presses and converts them into Vibin HTTP requests.

## Uses

`vibinremote` has been used with a wireless keyboard like the one shown below (a Rii K25
Multifunction Remote Control), connected to a Raspberry Pi.

<img src="https://github.com/mjoblin/media/blob/main/vibin/images/Rii_K25.jpg" width="250" />

## Configuration

`vibinremote` expects to be given the path to a JSON file containing the application configuration.
This configuration includes the location of the Vibin server on the network, as well as the keymap
details mapping key presses to Vibin URLs. Supported keys can be found in
[`validate.rs`](src/validate.rs).

### Example

```json
{
    "vibin": "vibin.local:8080",
    "url_timeout": 1,
    "keymap": {
        "UpArrow": {
            "url": "/api/system/amplifier/volume/up"
        },
        "DownArrow": {
            "url": "/api/system/amplifier/volume/down"
        }
    }
}
```

`"url_timeout"` is optional and defaults to `1` (seconds).

## Usage

```
$ vibinremote --help
Control the Vibin music streamer server with keyboard key presses.

Usage: vibinremote --config <JSON File>

Options:
  -c, --config <JSON File>  Configuration filename (JSON)
  -h, --help                Print help
  -V, --version             Print version
```

[Vibin]: https://github.com/mjoblin/vibin