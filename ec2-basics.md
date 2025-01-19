Connect:

```shell
ssh -i ec2-key.pem ec2-user@13.233.138.34
```

Delete:

```shell
rm -rf profile
```

Copy from local to ec2:

```shell
scp -r -i keys/ec2-key.pem janya ec2-user@13.233.138.34:~/
```
