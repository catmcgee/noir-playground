<p align="center">
  <img width="300" height="300" src="https://i.ibb.co/3msWR5y/favicon-white-min.png">
</p>

# Noir Playground

Noir Playground allows developers to learn Noir through a set of interactive
challenges in a GUI. 

<p align="center">
  <img src="https://github.com/catmcgee/noir-playground/blob/main/screenshot.png">
</p>

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

If you would just like to use the Playground, there is a deployed version [here](https://google.com), however it is much faster to run locally!

### Prerequisites

#### Server

1. [Rust and Cargo](https://www.rust-lang.org/tools/install)
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
2. [Noir and Nargo](https://noir-lang.org/getting_started/nargo_installation)
```curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash``` then ```noirup```

### Installing and Running

Clone the repo

```
git clone https://github.com/catmcgee/noir-playground.git
```

Install client dependencies

```
cd client
npm install
```

Install server dependencies
```
cd server
cargo build
```

Run app

```
npm run client
npm run server
```

This runs `cargo run` on the server and `npm run dev` on the client.

The app will be running on `localhost:3000` with the server running on `localhost:8080`.


## Environment variables

The client repo uses [dot-env](https://github.com/motdotla/dotenv) to manage environment variables.

```
NEXT_PUBLIC_SERVER_URL
```

This is refilled in `.env` as `"http://localhost:8080/"` but you may need to change it if you have specified a different port for your server.


## Deployment

Any push to `main` will deploy the project onto []().

## Built With

* [Next](https://nextjs.org/) - Next.js with React
* [Rust](https://www.rust-lang.org/) - Backend written in React
* [Noir and Nargo](https://noir-lang.org/) - Noir written on frontend compiles on the backend using Nargo

## Contributing

Please read [CONTRIBUTING.md]() for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

* **Cat McGee** 

See also the list of [contributors](https://github.com/noir-playground/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

