# enodump

A packet sniffer for EnOcean serial dongles.

Work in progress

## usage

<pre>
$ enodump -d /dev/ttyUSB0
Port /dev/ttyUSB0 opened.
EnOcean Device: VersionResponse { app: Version { main: 2, beta: 15, alpha: 0, build: 0 }, api: Version { main: 2, beta: 6, alpha: 9, build: 0 }, chip_id: Address([xx, xx, xx, xx]), chip_version: [69, 79, 1, 3], description: "GATEWAYCTRL" }

TYPE: <span style="color:yellow">01</span> <span style="color:blue">[a5, 0, 0, 92, 8, 1, 81, 92, 91, 0]</span> <span style="color:green">[0, ff, ff, ff, ff, 58, 0]</span>
TYPE: <span style="color:yellow">01</span> <span style="color:blue">[a5, 0, 0, 73, 8, 1, 80, 99, dc, 0]</span> <span style="color:green">[0, ff, ff, ff, ff, 44, 0]</span>
</pre>