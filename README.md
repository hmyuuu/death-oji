# Death-Oji

> *"Death-Oji escaped Live-Hime's samsara,*
> *then immediately reincarnated as the same karma:*
> *different body, same closed-source prison;*
> *the wheel knows only one song.."*

A modern web service for managing gugugaga live streaming codes, built with Rust backend and Astro frontend.

## Prerequisites

- [mise](https://github.com/jdx/mise) - Tool version manager (recommended)
- Rust (latest stable)
- Bun or Node.js
- gugugaga account with live streaming permissions

### Installing mise

mise automatically manages tool versions specified in `mise.toml`. Install it with:

```bash
# Using just (macOS uses Homebrew, Linux uses curl)
just install

# Or using make
make install

# Or manually
curl https://mise.run | sh  # Linux/macOS
brew install mise           # macOS only
```

**Shell Integration:**
- **Fish**: Tools are automatically activated (no config needed)
- **Bash/Zsh**: Add to your shell config:
  ```bash
  echo 'eval "$(mise activate bash)"' >> ~/.bashrc  # for bash
  echo 'eval "$(mise activate zsh)"' >> ~/.zshrc    # for zsh
  ```

After installing mise, run `mise install` in the project directory to install all required tools.

## Quick Start

### Using mise (Recommended)

```bash
mise install        # Install tools (first time only)
mise run dev        # Start both backend and frontend
```

Alternatively, use `just dev` or `make dev` for the same functionality.

### Manual Setup

**Backend (Port 11451):**
```bash
cd backend
cargo run
```

**Frontend (Port 11452):**
```bash
cd frontend
bun install
bun run dev
```

Open `http://localhost:11452` in your browser.

## Usage

### 1. Setup Credentials

**Option A: QR Code Login (Recommended)**
1. Click "QR Login" button in Account Credentials section
2. Scan QR code with Bilibili mobile app
3. Confirm login on your phone
4. Credentials auto-fill automatically

**Option B: Manual Entry**
- **Room ID**: Your gugugaga live room ID
- **Cookies**: Session cookies from gugugaga
- **CSRF Token**: csrf_token from cookies

**How to get credentials manually:**
1. Log in to gugugaga and open your live room
2. Press F12 → Network tab
3. Send a chat message
4. Find the "send" request
5. Copy Cookie in `Header` and csrf_token in `Payload`

### 2. Check User Info

Click "Check User Info" to load your profile and verify credentials.

### 3. Configure Stream

- **Title**: Your stream title (auto-saved)
- **Main Category**: Select from dropdown
- **Sub Category**: Auto-populated based on main category

### 4. Start Streaming

1. Click "Start Live Stream"
2. Copy the Server URL and Stream Key
3. Configure in OBS:
   - Settings → Stream
   - Service: Custom
   - Server: (paste Server URL)
   - Stream Key: (paste Stream Key)
4. Click "Start Streaming" in OBS

### 5. Stop Streaming

Click "Stop Live Stream" when done.

## Features Detail

- **QR Code Login**: Scan QR code with Bilibili app for instant login
- **Save/Load Credentials**: Save credentials to JSON file for reuse
- **Auto-save Settings**: Stream title and categories are automatically saved
- **Quick Links**: Direct links to your gugugaga space and live room (shown after loading user info)
- **Responsive Layout**: All blocks align properly whether 2 or 3 are visible

## API Endpoints

- `GET /api/qrcode/generate` - Generate QR code for login
- `POST /api/qrcode/poll` - Poll QR code login status
- `POST /api/stream/start` - Start stream, get RTMP credentials
- `POST /api/stream/stop` - Stop stream
- `PUT /api/stream/update` - Update stream title
- `POST /api/user/info` - Get user profile
- `GET /api/partitions` - Get category list

## Technology Stack

**Backend:**
- Axum - Web framework
- Reqwest - HTTP client
- Serde - JSON serialization
- Tokio - Async runtime

**Frontend:**
- Astro - Static site framework
- Tailwind CSS v4 - Styling
- Dracula Theme - Color scheme
- Vanilla JavaScript - Client logic

## Security Notes

- Credentials are only used for API requests
- No server-side storage of credentials
- Use HTTPS in production
- Keep cookies and tokens private

## License

MIT
