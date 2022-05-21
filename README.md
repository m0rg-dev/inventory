# wolfe inventory

Hacky little Vue.js / Rust-based inventory system for personal use.

This project isn't finished yet, and you might have a hard time running it on
your own hardware. Currently, it hard-assumes that PostgreSQL is available on
localhost:5432 with no authentication and that you have a Brother QL-500 printer
at `/dev/usb/lp0` (if you want to print labels).

If you want to try it anyways, go ahead and run PostgreSQL in a Docker container
locally with something like `docker run --rm -it -p 5432:5432 -e
POSTGRES_HOST_AUTH_METHOD=trust postgres`, then start the backend with a `cargo
run` (in `./backend`), then start the dev frontend server with a `npm install`
and `npm run dev`. Assuming you have enough terminal windows open, it should pop
up the interface on `localhost:3000`.