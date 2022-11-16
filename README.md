# Simple Twitter API implementation in Rust

In this sample project, I will create a small part of Twitter API to be able to post, read, and like tweets. 

## Pre-requisites

1. Cargo 
2. PostgreSQL v14 or higher
3. Environment variable **DATABASE_URL** configured with postgre database URL

## Tools

Following tools are used:

- **actix_rt:** Tokio-based single threaded async runtime for the Actix ecosystem
- **actix_web:** Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust
- **serde:** A generic serialization/deserialization framework
- **serde_json:** A JSON serialization file format
- **chrono:** Data and time library
- **uuid:** A library to generate and parse UUID
- **env_logger:** A logging implementation for ==log== which is configured via an environment variable
- **diesel:** A safe, extensible ORM framework and Query builder
- **r2d2:** A generic connection pool
- **r2d2-diesel:** r2d2 support for Diesel ORM

## Setup

### PostgreSQL Table schema

We need to create below tables in **test** schema in PostgreSQL DB 

```
create table test.tweets (
id uuid not null primary key, 
created_at timestamp not null, 
message varchar(500)
);

create table test.likes (
id uuid not null primary key, 
created_at timestamp not null, 
tweet_id uuid not null references test.tweets(id));
```

In my case, I have used custom schema named **test**. When you create environment variable, make sure you pass the schema name in the URL

Ex: 

```
set DATABASE_URL=postgres://postgres:postgres@localhost:5432/testdb?options=-c%20search_path%3Dtest
```

### Launch the application

Once the pre-requsites are well configured, please run **cargo run** to launch the application. You can use below CURL commands to test the application

**Post a tweet**

```
curl -X POST -d "{\"message\": \"This is a tweet\"}" -H "Content-type: application/json" http://localhost:9090/tweets
```

**Retrieve all tweets**

```
curl http://localhost:9090/tweets
```

**Get a specific tweet**

```
curl http://localhost:9090/tweets/{id}
```

**Like a tweet**

```
curl -X POST http://localhost:9090/tweets/{id}/likes
```

**Unlike a tweet**

```
curl -X DELETE http://localhost:9090/tweets/{id}/likes
```

**Delete a tweet**

```
curl -X DELETE http://localhost:9090/tweets/{id}
```
