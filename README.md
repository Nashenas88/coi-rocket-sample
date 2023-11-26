# coi-actix-sample

This project is designed to show off [`coi`] and its integration with [`actix-web`] through the
[`coi-actix-web`] crate.

[`coi`]: https://github.com/Nashenas88/coi
[`actix-web`]: https://github.com/actix/actix-web
[`coi-actix-web`]: https://github.com/Nashenas88/coi-actix-web

## Prerequisites
docker is running, `docker-compose` installed, `psql` cli command is available

## Setup
Create a `.env` file with the connection info:
```
export POSTGRES_USER=username
export POSTGRES_PW=changeit
export POSTGRES_DB=postgres
export PGADMIN_MAIL=username@email.com
export PGADMIN_PW=changeit
```

Start off by creating the docker container and seeding it with sample data. Make sure to include the
parentheses so you don't leak the env data into your current environment:1

```
(source .env && cargo xtask seed)
```

Verify that the above executed successfully and that the docker instance is still running:
```
docker ps
```
You should see an output similar to:
```
CONTAINER ID   IMAGE                   COMMAND                  CREATED          STATUS          PORTS                                            NAMES
2b0163af5aef   dpage/pgadmin4:latest   "/entrypoint.sh"         29 minutes ago   Up 29 minutes   443/tcp, 0.0.0.0:5050->80/tcp, :::5050->80/tcp   pgadmin
891533a5a77a   postgres:latest         "docker-entrypoint.s…"   29 minutes ago   Up 29 minutes   0.0.0.0:5432->5432/tcp, :::5432->5432/tcp        postgres
```

## Running

```
(source .env && cargo run --release)
```

And visit http://localhost:8000/data

You should see something like the following if you've visited the above url in firefox:
![{
    0: {
        id: 0,
        name: "Paul"
    }
}](readme_assets/data.png)
