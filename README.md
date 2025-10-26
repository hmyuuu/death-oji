# Death-Oji

> *"Death-Oji escaped Live-Hime's samsara,*
> *then immediately reincarnated as the same karma:*
> *different body, same closed-source prison;*
> *the wheel knows only one song.."*

A modern web service for managing gugugaga live streaming codes, built with Rust backend and Astro frontend.

## Prerequisites

- Rust (latest stable)
- Bun or Node.js
- gugugaga account with live streaming permissions

## Quick Start

### Backend (Port 11451)

```bash
cd backend
cargo run
```

### Frontend (Port 11452)

```bash
cd frontend
bun install
bun run dev
```

Open `http://localhost:11452` in your browser.

## Usage

### 1. Setup Credentials

- **Room ID**: Your gugugaga live room ID
- **Cookies**: Session cookies from gugugaga
- **CSRF Token**: bili_jct token from cookies

**How to get credentials:**
1. Log in to gugugaga and open your live room
2. Press F12 → Network tab
3. Send a chat message
4. Find the "send" request
5. Copy Cookie header and csrf_token in `Payload`

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

- **Save/Load Credentials**: Save credentials to JSON file for reuse
- **Auto-save Settings**: Stream title and categories are automatically saved
- **Quick Links**: Direct links to your gugugaga space and live room (shown after loading user info)
- **Responsive Layout**: All blocks align properly whether 2 or 3 are visible

## API Endpoints

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
