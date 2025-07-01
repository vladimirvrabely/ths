# ths


Small project to read temperature/humidity sensor and store measurements.

## Hardware

* Raspberry Pi Zero W
* [RS485-USB adapter](docs/RS485_USB-A_20_Adapter_5-Pin_with_CH340_USB_chip_and_SP3485+TVS.jpg)
* [RS485 MODBUS-RTU Temperature Humidity Transmitter](docs/THT-XYMD03.pdf)


## Usage

* Cross-compile for the relevant target
```sh
just cross-build
```

* Copy on RPi to `/home/<user>/ths`

* Create `systemd` unit file `/etc/systemd/system/ths.service`
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
ExecStart=/home/bin/<user>/ths
StandardOutput=null
```

* Start service
```
systemctl start ths
```