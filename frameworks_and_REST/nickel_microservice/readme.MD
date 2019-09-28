# Nickel Microservice - Emails

Similar to the hyper crate, however it's synchronous

## SMTP Server
Need to start up an SMTP server on docker
```shell script
docker run -it --rm --name test-smtp -p 2525:25  \
       -e SMTP_SERVER=smtp.example.com \
       -e SMTP_USERNAME=admin@example.com \
       -e SMTP_PASSWORD=password \
       -e SERVER_HOSTNAME=smtp.example.com \
       juanluisbaptiste/postfix
```

## Testing the Service
```shell script
cargo run

# New terminal
curl -d "to=email@example.com&code=passcode" -X POST http://localhost:8002/send
```