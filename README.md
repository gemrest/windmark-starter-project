# Windmark Starter Project

## Grab the Project!

Get started by cloning this project with Git and `cd`ing into it!

```shell
git clone https://github.com/gemrest/windmark-starter-project
cd windmark-starter-project
```

## Generate an OpenSSL Key Pair

Next, generate an OpenSSL key pair so you can identify yourself over TLS!

Make sure to have OpenSSL installed on your system before running this command!

You can change the name of the files, but then also make sure to change the
names in the `src/main.rs` file.

Before pushing to production, make sure to change the common name from
"localhost" to your domain!

```shell
openssl req \
  -new \
  -subj /CN=localhost \
  -x509 \
  -newkey ec \
  -pkeyopt ec_paramgen_curve:prime256v1 \
  -days 365 \
  -nodes \
  -out windmark_starter_project_public.pem \
  -keyout windmark_starter_project_private.pem \
  -inform pem
```

## Building the Project

Make sure to have Cargo installed (along with Rust) and run `cargo build` to
build your project, or `cargo run` to run your project!

## Hack On!

Change whatever you'd like! Make sure to checkout the [examples directory](https://github.com/gemrest/windmark/tree/main/examples) for some useful examples of some of Windmark's features!

