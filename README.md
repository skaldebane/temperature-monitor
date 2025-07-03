# `temperature-monitor`
[![No Maintenance Intended](https://unmaintained.tech/badge.svg)](https://unmaintained.tech/)
I usually don't notice when my laptop's struggling to cool down due to the fan being blocked,
often leading to sudden emergency shutdowns from sustained overheating.

This is a small personal Linux utility to alert me when the system starts overheating, so I can 
check and improve the fan's ventilation way before that happens.

![[Notification screenshot]](img/screenshot.png)

To install, use the `systemd-install` script: it'll automatically build the project, copy the binary to `/usr/local/bin`,
then set up and start a systemd service to run at startup.

Assumptions:
- `acpitz temp1` exists and is a good sensor for overall temperature measure.
- 80°C is a reasonable threshold.
- The desktop environment supports XDG notifications properly.

All potential errors are handled / ignored, so this shall normally never crash.

[No maintenance intended](https://unmaintained.tech/), use at your own risk.

## License
`temperature-monitor` is [MIT-licensed](LICENSE).
