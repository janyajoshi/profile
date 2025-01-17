##   Setup Amazon Linux ec2 image
target: x86_64-unknown-linux-gnu, build with [cross](https://github.com/cross-rs/cross)

```shell
sudo dnf update -y
sudo yum groupinstall "Development Tools"
exec $SHELL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
exec $SHELL
sudo dnf install git -y
git clone https://github.com/janyajoshi/profile.git
git checkout vanilla
git pull
cargo build --release
cd /home/ec2-user
cp /home/ec2-user/profile/target/release/janya .
```
There are 2 ways to setup a background task
  -   Background service with ```systemctl``` (preferred for long running tasks)
  -   using ```bg``` with ```jobs``` and ```kill``` 
##  systemctl
```shell
cd /etc/systemd/system/
sudo nano profileserver.service
sudo systemctl daemon-reload  # reload changes to service configuration
sudo systemctl start profileserver.service  # service
sudo systemctl restart profileserver.service  # restart
sudo systemctl enable profileserver.service # auto restart when server starts
sudo journalctl -u profileserver.service  # view process logs
```

```txt
[Unit]
Description=Service to host profile

[Service]
ExecStart=/home/ec2-user/janya
Restart=on-failure
RestartSec=3

[Install]
WantedBy=multi-user.target
```
##  bg

```shell
/home/ec2-user/janya
# press ctrl + z now
# type bg
# press enter, task will be pushed to background now
jobs  # view background tasks
kill %1 # kill first item on the list
kill -9 %1  # kill forcefully
```