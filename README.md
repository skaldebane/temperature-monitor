# `temperature-monitor`
[![No Maintenance Intended](https://unmaintained.tech/badge.svg)](https://unmaintained.tech/)

A small personal Linux utility to prevent my laptop from overheating without me noticing,
so I can unblock the fan before a forced shutdown from sustained heat.

Use the `systemd-install` script to automatically build it and set up a
systemd service to run at startup.

`temperature-monitor` assumes that:
- `acpitz temp1` exists and is a good overall temperature measure.
- 80°C is a reasonable threshold.
- The desktop environment supports XDG notifications properly.

All potential errors are handled / ignored, so this shall normally never crash.

[No maintenance intended](https://unmaintained.tech/), use at your own risk.

## License
`temperature-monitor` is [MIT-licensed](LICENSE).
