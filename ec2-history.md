```text
    1  ls
    2  jobs
    3  ls
    4  /home/ec2-user/janya
    5  sudo nano /etc/systemd/system/profileserver.service
    6  cat nano /etc/systemd/system/profileserver.service
    7  sudo systemctl daemon-reload
    8  sudo systemctl start profileserver.service
    9  sudo journalctl -u profileserver.service
   10  sudo systemctl enable profileserver.service
   11  exit
   12  ls
   13  ./janya
   14  exit
   15  sudo systemctl stop profileserver.service
   16  sudo systemctl status profileserver.service
   17  ls
   18  sudo /janya
   19  sudo ./janya
   20  sudo systemctl start profileserver.service
   21  yum search nginx
   22  sudo iptables nat -l
   23  iptables
   24  clear
   25  sudo yum install iptables-services -y
   26  sudo systemctl start iptables
   27  sudo systemctl enable iptables
   28  sudo systemctl status profileserver.service
   29  sudo iptables -t nat -A PREROUTING -p tcp --dport 8080 -j REDIRECT --to-port 80
   30  sudo iptables -t nat -L -v -n --line-numbers
   31  sudo systemctl disable iptables
   32  sudo systemctl stop iptables
   33  sudo systemctl start iptables
   34  sudo systemctl stop iptables
   35  sudo yum uninstall iptables-services
   36  sudo yum autoremove iptables-services
   37  clear
   38  echo "refresh"
   39  clear
   40  sudo yum install nginx
   41  sudo systemctl start nginx
   42  sudo systemctl enable nginx
   43  sudo systemctl status nginx
   44  sudo nano /etc/nginx/nginx.conf
   45  sudo -t nginx
   46  sudo nginx -t
   47  sudo systemctl reload nginx
   48  history
```
