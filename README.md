

### Settings example


~/.docker-statistics-ui



```yaml
envs:
  env-1:
  - url: http://10.0.0.2:7999
  - url: http://10.0.0.3:7999
  - url: http://10.0.0.4:7999

  env-2:
  - url: http://10.0.1.2:7999
  - url: http://10.0.1.3:7999
  - url: http://10.0.1.4:7999
```


```yaml
envs:
  env-1:
  - url: ssh:gateway@10.0.0.0:22->http://10.0.0.2:7999
  - url: ssh:gateway@10.0.0.0:22->http://10.0.0.3:7999
  - url: ssh:gateway@10.0.0.0:22->http://10.0.0.4:7999

  env-2:
  - url: ssh:gateway@10.0.0.1:22->http://10.0.0.2:7999
  - url: ssh:gateway@10.0.0.1:22->http://10.0.0.3:7999
  - url: ssh:gateway@10.0.0.1:22->http://10.0.0.4:7999

ssh_private_keys:
  "*":
    cert_path: /root/cert
    cert_pass_prase: password
```


```yaml
envs:
  env-1:
  - url: ssh:gateway@10.0.0.0:22->http://10.0.0.2:7999
  - url: ssh:gateway@10.0.0.0:22->http://10.0.0.3:7999
  - url: ssh:gateway@10.0.0.0:22->http://10.0.0.4:7999

  env-2:
  - url: ssh:gateway@10.0.0.1:22->http://10.0.0.2:7999
  - url: ssh:gateway@10.0.0.1:22->http://10.0.0.3:7999
  - url: ssh:gateway@10.0.0.1:22->http://10.0.0.4:7999

ssh_private_keys:
  "gateway@10.0.0.0:22":
    cert_path: /root/cert-1
    cert_pass_prase: password

  "gateway@10.0.0.1:22":
    cert_path: /root/cert-2
    cert_pass_prase: password
```

ssh_private_keys - can be missing. In this case SshAgent will be used.


## Not Setting private key passkey to settings

In this case - application would ask to enter pass_phrase before it starts working.
```yaml
prompt_pass_phrase: true

ssh_private_keys:
  "*":
    cert_path: /root/cert
```