# Impact

Design and implement “Word of Wisdom” tcp server.

• TCP server should be protected from DDOS attacks with the Prof of Work (https://en.wikipedia.org/wiki/Proof_of_work), the challenge-response protocol should be used.
• The choice of the POW algorithm should be explained.
• After Prof Of Work verification, server should send one of the quotes from “word of wisdom” book or any other collection of the quotes.
• Docker file should be provided both for the server and for the client that solves the POW challenge


# Establish network

```shell
docker network create local
```

# Run server

```shell
docker run --network local --rm --name quote_server -e CRAP_SECRET=123 rozhkovdmitrii/quotes-service:local quote_server listen --config config.yml
```

# Run client

```shell
docker run --network local --rm --name quote_client -e CRAP_SECRET=123 rozhkovdmitrii/quotes-service:local quote_client get-quote --host quote_server --port 8081
```

# Run with docker compose





