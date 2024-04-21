# Adding a SystemD Service

Edit / Create a service file

```sh
$ nano nano /etc/systemd/system/iboardbot.service
```

Add the following content inside, and make sure that the ExecStart and WorkingDirectory point to the correct places.

```toml
[Unit]
Description=Service that keeps running the ibotbot API-server from startup

[Install]
WantedBy=multi-user.target

[Service]
Type=simple
ExecStart=/root/iboardbot/target/release/iboardbot --port 8080
WorkingDirectory=/root/iboardbot/target/release
Restart=always
RestartSec=5
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=%n
```

Reload systemd

```sh
$ systemd daemon-reload
```

And enable the service

```sh
$ systemctl enable --now iboardbot.service
```

Done!