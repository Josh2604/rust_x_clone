# (X) Twitter Clone app

## Instalación de diesel cli
```
cargo install diesel_cli
```
En caso de no requerir la instalacion de las features por defecto
* Incluye mysql por defecto y en caso de no tenerlo instalado este fallara

```
cargo install diesel_cli --no-default-features --features mysql
```

## Tablas

```
diesel migration generate create_tweets
diesel migration generate create_likes
```

### Diesel setup
```
diesel setup
```

### Running migratios
```
diesel migration run
```