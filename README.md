# ths


Small project to read temperature/humidity sensor and store measurements.

## Hardware

* Raspberry Pi Zero W with USB 2.0/USB mini hub
* [RS485-USB adapter](docs/RS485_USB-A_20_Adapter_5-Pin_with_CH340_USB_chip_and_SP3485+TVS.jpg)
* [RS485 MODBUS-RTU Temperature Humidity Transmitter](docs/THT-XYMD03.pdf)


## Usage

* Cross-compile for the relevant target
```sh
just cross-build
```

* Copy `target/aarch64-unknown-linux-gnu/release/ths` to RPi to the dir `/home/<user>/bin/`

* On RPi create `systemd` unit file called `/etc/systemd/system/ths.service`
```
[Unit]
Description=Temperature Humidity Sensor

[Service]
Type=simple
Restart=always
RestartSec=1
User=<user>
Environment="TTY_PATH=/dev/ttyUSB0"
Environment="MEASUREMENT_FILE=/home/<user>/data/measurement.csv"
ExecStart=/home/<user>/bin/ths
StandardOutput=null
```

* Start service
```
systemctl start ths
```

* Copy the file `/home/<user>/data/measurements.csv` and check the data with the notebook in `python/main.ipynb` notebook


 ### Troubleshooting

* You might need to add `<user>` to `dialout` group to read the `tty` port, see [here](https://askubuntu.com/questions/210177/serial-port-terminal-cannot-open-dev-ttys0-permission-denied).
