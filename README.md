# Deploy to EC-2
Focus here is to deploy on [t4g.nano](https://aws.amazon.com/ec2/instance-types/t4/)

- build with [cross](https://github.com/cross-rs/cross)
    ```shell
    cross build --target x86_64-unknown-linux-gnu --release
    cross build --target aarch64-unknown-linux-gnu --release
    ```
- install nginx (for redirecting all traffic to our port + support ssl)
    ```shell
    sudo yum install nginx
    sudo systemctl start nginx
    sudo systemctl enable nginx # auto start when machine boots up next
    sudo systemctl status nginx
    sudo nano /etc/nginx/nginx.conf # update server_name from _ to janya.joshi-rj.in
    sudo nginx -t # test updated configuration
    sudo systemctl reload nginx
    ```
- install [certbot](https://certbot.eff.org/instructions?ws=nginx&os=snap), which uses [Let's Encrypt](https://letsencrypt.org/) as a public Certificate Authority (CA)
  ```shell
  sudo yum install certbot python3-certbot-nginx
  # update your route 53, certbox will need to use domain to reach to this machine. verify with nginx home page
  # stop nginx now, certbot needs port 80
  sudo certbot --nginx  # when prompted, use email: janyajoshi18@gmail.com, domain name(s): janya.joshi-rj.in, use method [1] (if prompted for verification
  # by now, above command might have messed up with nginx config, will fix it next.
  sudo certbot renew --dry-run  # simulate dry run for renewal
  # we can start nginx back now, verify your website now, it should be fine with https.
  # take note of certificate path there, that is important. It should look something like below
  ```
  ```text
  Successfully received certificate.
  Certificate is saved at: /etc/letsencrypt/live/janya.joshi-rj.in/fullchain.pem
  Key is saved at:         /etc/letsencrypt/live/janya.joshi-rj.in/privkey.pem
  This certificate expires on 2025-04-19.
  These files will be updated when the certificate renews.
  Certbot has set up a scheduled task to automatically renew this certificate in the background.
  ```
- upload binary
  ```shell
  export ec2ip=<public_ipv4>
  scp -r -i keys/ec2-key.pem janya ec2-user@$ec2ip:~/.
  ssh -i keys/ec2-key.pem ec2-user@$ec2ip # connect if needed
  rm -rf janya # delete a file or directory
  ```
- create process
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
- update nginx config (refactor messed up indentation by certbot + redirect traffic), make note to keep certificate path same as seen during generating with certbot (above)
  ```nginx configuration
    # For more information on configuration, see:
    #   * Official English Documentation: http://nginx.org/en/docs/
    #   * Official Russian Documentation: http://nginx.org/ru/docs/
    
    user nginx;
    worker_processes auto;
    error_log /var/log/nginx/error.log notice;
    pid /run/nginx.pid;
    
    # Load dynamic modules. See /usr/share/doc/nginx/README.dynamic.
    include /usr/share/nginx/modules/*.conf;
    
    events {
    worker_connections 1024;
    }
    
    http {
    log_format  main  '$remote_addr - $remote_user [$time_local] "$request" '
    '$status $body_bytes_sent "$http_referer" '
    '"$http_user_agent" "$http_x_forwarded_for"';
    
        access_log  /var/log/nginx/access.log  main;
    
        sendfile            on;
        tcp_nopush          on;
        keepalive_timeout   65;
        types_hash_max_size 4096;
    
        include             /etc/nginx/mime.types;
        default_type        application/octet-stream;
    
        # Load modular configuration files from the /etc/nginx/conf.d directory.
        # See http://nginx.org/en/docs/ngx_core_module.html#include
        # for more information.
        include /etc/nginx/conf.d/*.conf;
    
        server {
            server_name  janya.joshi-rj.in;
            root         /usr/share/nginx/html;
    
            # Load configuration files for the default server block.
            include /etc/nginx/default.d/*.conf;
    
            location / {
                    proxy_pass http://localhost:8080;
                    proxy_set_header Host $host;
                    proxy_set_header X-Real-IP $remote_addr;
                    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                    proxy_set_header X-Forwarded-Proto $scheme;
            }
    
            error_page 404 /404.html;
            location = /404.html {
            }
    
            error_page 500 502 503 504 /50x.html;
            location = /50x.html {
            }
        
            listen [::]:443 ssl ipv6only=on; # managed by Certbot
            listen 443 ssl; # managed by Certbot
            ssl_certificate /etc/letsencrypt/live/janya.joshi-rj.in/fullchain.pem; # managed by Certbot
            ssl_certificate_key /etc/letsencrypt/live/janya.joshi-rj.in/privkey.pem; # managed by Certbot
            include /etc/letsencrypt/options-ssl-nginx.conf; # managed by Certbot
            ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem; # managed by Certbot
    
        }
    
    # Settings for a TLS enabled server.
    #
    #    server {
    #        listen       443 ssl;
    #        listen       [::]:443 ssl;
    #        http2        on;
    #        server_name  _;
    #        root         /usr/share/nginx/html;
    #
    #        ssl_certificate "/etc/pki/nginx/server.crt";
    #        ssl_certificate_key "/etc/pki/nginx/private/server.key";
    #        ssl_session_cache shared:SSL:1m;
    #        ssl_session_timeout  10m;
    #        ssl_ciphers PROFILE=SYSTEM;
    #        ssl_prefer_server_ciphers on;
    #
    #        # Load configuration files for the default server block.
    #        include /etc/nginx/default.d/*.conf;
    #
    #        error_page 404 /404.html;
    #        location = /404.html {
    #        }
    #
    #        error_page 500 502 503 504 /50x.html;
    #        location = /50x.html {
    #        }
    #    }
    
    
        server {
            #if ($host = janya.joshi-rj.in) {
            #    return 301 https://$host$request_uri;
            #} # managed by Certbot
    
    
            listen       80;
            listen       [::]:80;
            server_name  janya.joshi-rj.in;
            root         /usr/share/nginx/html;
    
            # Load configuration files for the default server block.
            include /etc/nginx/default.d/*.conf;
    
            location / {
                    proxy_pass http://localhost:8080;
                    proxy_set_header Host $host;
                    proxy_set_header X-Real-IP $remote_addr;
                    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                    proxy_set_header X-Forwarded-Proto $scheme;
            }
    
            error_page 404 /404.html;
            location = /404.html {
            }
    
            error_page 500 502 503 504 /50x.html;
            location = /50x.html {
            }
            #return 404; # managed by Certbot
        }
    }
  ```
  For some unknown reason, port now shows occupied and then nginx cannot be reloaded. In this case, ```reboot``` the instance.


### It should be working fine after this, for testing,
- check [http://janya.joshi-rj.in/](http://janya.joshi-rj.in/)
- check [https://janya.joshi-rj.in/](https://janya.joshi-rj.in/)
- ```curl janya.joshi-rj.in```

##  Misc

- background tasks using ```bg```
  ```shell
  /home/ec2-user/janya
  # press ctrl + z now
  # type bg
  # press enter, task will be pushed to background now
  jobs  # view background tasks
  kill %1 # kill first item on the list
  kill -9 %1  # kill forcefully
  ```
- build on ec2 (needs instance with at least 1G RAM)
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
