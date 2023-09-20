# (X) Twitter Clone app

## Dependencies

* Install mysql or use a docker instance

## Configs

### Diesel CLI installation
```
cargo install diesel_cli
```
* in error case run the next command

```
cargo install diesel_cli --no-default-features --features mysql
```
### Run
```
cargo run
```

## Diesel usage
### Tables
Create tables
```
diesel migration generate create_tweets
diesel migration generate create_likes
```

### Diesel setup
```
diesel setup
```

### Migrations

Create migrations
```
diesel migration run
```

Rollback
```
diesel migration revert
```
