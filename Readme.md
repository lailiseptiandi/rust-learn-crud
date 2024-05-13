# Learn Rust

Simple project crud with rust language.

Please create database mysql:

## Run Locally

Go to the project directory

```bash
  cd my-project
```

Setting Database Mysql, open file database.rs and change :

```bash
  mysql://root:password@localhost/rust_dev_api_crud
```

Run command for starting project :

```bash
  cargo build --release
  cargo run
```

## API Reference

#### Get all users

```http
  GET /api/v1/users

```

#### Create Users

```http
  GET /api/v1/users

```

| Body       | Type     | Description       |
| :--------- | :------- | :---------------- |
| `name`     | `string` | **Not Required**. |
| `email`    | `string` | **Required**.     |
| `password` | `string` | **Required**.     |

## Authors

- [@lailiseptiandi](https://www.github.com/lailiseptiandi)
