# Twitter Bot

A Rust-based Twitter bot application using Twitter API v2 for posting and interacting with tweets.

## Setup

### Prerequisites

- Rust (latest stable version)
- Twitter Developer Account with API keys
- Cargo package manager

### Installation

1. Clone this repository:

   ```bash
   git clone <your-repo-url>
   cd twitter-bot
   ```

2. Install dependencies:

   ```bash
   cargo build
   ```

3. Set up environment variables:
   Create a `.env` file in the root directory with your Twitter API credentials:

   ```env
   CLIENT_ID=your_twitter_client_id_here
   CLIENT_SECRET=your_twitter_client_secret_here
   ```

4. Run the application:
   ```bash
   cargo run
   ```

### Configuration

The application supports OAuth2 authentication by default. You can configure different authentication methods through Cargo features:

- Default: OAuth2 enabled
- OAuth1: Use `cargo build --no-default-features` for OAuth1 only

## Usage

The bot runs a web server on `localhost:3000` that handles OAuth2 authentication flow with Twitter. Visit the server to authenticate and start using the bot.

## Attribution

This project was built with significant help from the [twitter-v2-rs](https://github.com/jpopesculian/twitter-v2-rs) library and examples. Most of the Twitter API integration code is based on or inspired by that excellent project.

## Documentation

Additional resources:

- Request handling: https://docs.rs/reqwest/latest/reqwest/
- Rust programming: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/documentation.html
