# Zero to Production in Rust

This is a project based on the [Zero To Production](https://www.zero2prod.com/) book by Luca Palmieri.

The goal is to learn more about building a production ready web application in Rust,
while creating a newsletter subscription service.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- Some brain cells.

## Running

#### Database

```bash
docker run --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres
```

#### Server

`cargo run`

License [MIT](./LICENSE)