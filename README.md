setup ec2

- t4g.nano
- network setting > auto-assign public ip > enabled
- existing key-pair
- existing security group

connect via ssh

```shell
ssh -i ~/keys/ec2-key.pem ec2-user@ec2-13-233-130-227.ap-south-1.compute.amazonaws.com
```

install dependencies

```shell
sudo yum upgrade-minimal
sudo yum install nginx make certbot python3-certbot-nginx
sudo systemctl start nginx
sudo systemctl enable nginx # auto-start on boot
sudo systemctl status nginx
```

setup ssl

```shell
# update route 53, certbox will need to use domain to reach to this machine. verify with nginx home page
# stop nginx now, certbot needs port 80
sudo systemctl stop nginx
sudo systemctl status nginx

# nginx-pre.conf as nginx configuration, certbot needs server_name from there
sudo nano /etc/nginx/nginx.conf
sudo nginx -t


sudo certbot --nginx  # when prompted, use email: janyajoshi18@gmail.com, domain name(s): janya.joshi-rj.in, use method [1] (if prompted for verification
# by now, above command might have messed up with nginx config, will fix it next.
sudo certbot renew --dry-run  # simulate dry run for renewal
# we can start nginx back now, verify your website now, it should be fine with https.
# take note of certificate path there, that is important. It should look something like below
```

fix nginx config

```shell
sudo nano /etc/nginx/nginx.conf
# remove everything
# go to start of file, press control + shift + 6, essentially control + ^, to begin selection.
# select till the end, press control + k to cut it.

# paste from nginx.conf

sudo nginx -t # validate changes

# till now, certbot has occupied ports, need to free them and start back nginx
sudo ss -tulpn | grep -E ':80|:443'
# tcp   LISTEN 0      511                          0.0.0.0:443       0.0.0.0:*    users:(("nginx",pid=25847,fd=8),("nginx",pid=25846,fd=8),("nginx",pid=25704,fd=8))
# tcp   LISTEN 0      511                          0.0.0.0:80        0.0.0.0:*    users:(("nginx",pid=25847,fd=6),("nginx",pid=25846,fd=6),("nginx",pid=25704,fd=6))

sudo fuser -k 80/tcp
sudo fuser -k 443/tcp

sudo systemctl start nginx
sudo systemctl status nginx
```

exit from ssh

copy folder

```shell
cd /Users/janyajoshi/projects
export ec2ip=13.233.130.227
scp -r -i ~/keys/ec2-key.pem ./make-handson ec2-user@$ec2ip:~/.
```

log back in using ssh

check if we are able to build it

```shell
cd /home/ec2-user/make-handson
make
# check with IP on browser
# ctrl + c
```

create background process

```shell
sudo nano /etc/systemd/system/profileserver.service

sudo systemctl daemon-reload  # reload changes to service configuration

sudo systemctl start profileserver.service  # start

# verify on browser if it appears
# other helpful commands
sudo systemctl stop profileserver.service  # stop
sudo systemctl restart profileserver.service  # restart
sudo journalctl -u profileserver.service  # view process logs
systemctl list-unit-files --type=service --state=enabled # list enabled services
```

auto restart when server starts

```shell
sudo systemctl enable profileserver.service
```

```txt
[Unit]
Description=Service to host profile

[Service]
WorkingDirectory=/home/ec2-user/make-handson
ExecStart=make
Restart=on-failure
RestartSec=3
StandardOutput=file:/home/ec2-user/profile-server.log
StandardError=file:/home/ec2-user/profile-server-error.log

[Install]
WantedBy=multi-user.target
```

for some reason, incomming requests are logged into profile-server-error.log => check later

If getting an error while deleting make-handson

```shell
sudo chown -R ec2-user:ec2-user make-handson # change ownership to ec2-user
rm -rf make-handson # should work now
```
