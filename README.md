# Twitter Bot

A Rust-based Twitter bot application using Twitter API v2 for posting tweets both via web interface and directly from your terminal.

## Features

- 🐦 **Tweet from Terminal**: Interactive tweeting directly from your command line
- 🌐 **Web Interface**: OAuth2 authentication and web-based tweeting
- 🔒 **Secure Authentication**: OAuth2 flow with PKCE for enhanced security
- 🔄 **Token Management**: Automatic token refresh and revocation
- ⚡ **Real-time**: Post tweets instantly from terminal after authentication

## Setup

### Prerequisites

- **Rust** (latest stable version)
- **Twitter Developer Account** with API v2 access
- **Cargo** package manager

### Twitter Developer Setup

1. Go to [Twitter Developer Portal](https://developer.twitter.com/en/portal/dashboard)
2. Create a new app or use existing one
3. Navigate to your app's "Keys and Tokens" section
4. Generate OAuth 2.0 Client ID and Client Secret
5. Set up OAuth 2.0 settings:
   - **Type of App**: Web App
   - **Callback URI**: `http://127.0.0.1:3000/callback`
   - **Website URL**: `http://127.0.0.1:3000` (or your domain)

### Installation

1. **Clone this repository:**

   ```bash
   git clone <your-repo-url>
   cd twitter-bot
   ```

2. **Install dependencies:**

   ```bash
   cargo build
   ```

3. **Set up environment variables:**
   Create a `.env` file in the root directory with your Twitter API credentials:

   ```env
   CLIENT_ID=your_twitter_client_id_here
   CLIENT_SECRET=your_twitter_client_secret_here
   RUST_LOG=info
   ```

4. **Run the application:**
   ```bash
   cargo run
   ```

## Usage

### Initial Authentication

1. **Start the server:**
   ```bash
   cargo run
   ```
2. **Follow the authentication flow:**

   - The server will start on `http://127.0.0.1:3000`
   - Open `http://127.0.0.1:3000/login` in your browser
   - Authorize the app with Twitter
   - You'll be redirected back to the callback URL

3. **Start tweeting from terminal:**
   - After successful authentication, the terminal will show: "login finish"
   - You can now type tweets directly in the terminal

### Terminal Tweeting

Once authenticated, you can tweet directly from your terminal:

```
enter tweet: Hello from my Rust Twitter bot! 🦀
Tweet posted successfully! ID: 1234567890123456789

enter tweet: This is amazing! I can tweet from my terminal now! ✨
Tweet posted successfully! ID: 1234567890123456790

enter tweet: quit
Goodbye! (Server will keep running)
```

**Commands:**

- Type any message (up to 280 characters) to tweet
- Type `quit` or `exit` to stop terminal tweeting
- Server continues running for web API access

### Web API Endpoints

- **`GET /login`** - Start OAuth2 authentication
- **`GET /callback`** - OAuth2 callback (automatic redirect)
- **`GET /tweets`** - View tweet status or post via URL parameters
- **`GET /tweets?text=Hello%20World`** - Post a tweet via web
- **`GET /revoke`** - Revoke OAuth token

### Examples

**Tweet via URL:**

```
http://127.0.0.1:3000/tweets?text=Hello%20from%20the%20web%20interface!
```

**Check authentication status:**

```
http://127.0.0.1:3000/tweets
```

## Configuration

The application supports OAuth2 authentication by default. You can configure different authentication methods through Cargo features:

- **Default**: OAuth2 enabled
- **OAuth1 only**: Use `cargo build --no-default-features` for OAuth1 only

## Troubleshooting

### Common Issues

1. **"could not find CLIENT_ID" error:**

   - Make sure your `.env` file is in the project root
   - Verify the CLIENT_ID and CLIENT_SECRET are correct

2. **Authentication fails:**

   - Check that your callback URL in Twitter Developer Portal matches: `http://127.0.0.1:3000/callback`
   - Ensure your app has proper permissions (Read and Write)

3. **Tweet posting fails:**

   - Verify your Twitter app has "Read and Write" permissions
   - Check that tweets are under 280 characters
   - Ensure you're authenticated (complete the web login first)

4. **Server won't start:**
   - Make sure port 3000 is available
   - Check if another instance is already running

### Logs

Enable detailed logging by setting in your `.env`:

```env
RUST_LOG=tweetterminal=debug,tower_http=debug
```

## Attribution

This project was built with significant help from the [twitter-v2-rs](https://github.com/jpopesculian/twitter-v2-rs) library and examples. Most of the Twitter API integration code is based on or inspired by that excellent project.

## Documentation

Additional resources:

- Request handling: https://docs.rs/reqwest/latest/reqwest/
- Rust programming: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/documentation.html
