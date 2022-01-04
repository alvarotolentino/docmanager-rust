# Document Manager Rest API - Rust implementation

A Rust implementation of the ASP.NET Core 3.1 Web API

This is a work in progress...

## How to run the app?
 Install the crate __cargo-make__ in order to automate some provisioning tasks:

 ```
 cargo install cargo-make
 ```

Create the .env file at the root path with the following content:

```
PORT=8080
RUST_LOG=docmanager=info
DATABASE_URL=postgres://root:password@localhost/docmanager
POSTGRES_USER=root
POSTGRES_PASSWORD=password
POSTGRES_PORT=5432
POSTGRES_DB=docmanager
```

Execute the following command to provisioning the database service and run the migration script:

```
makers reset
```

and the following to run in release mode:
```
makers build-release
```

or this one for debug mode:
```
makers build-debug
```

The server is running at: http://127.0.0.1:8080

