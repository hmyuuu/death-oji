# CLAUDE.md - Development Guide

## Project Overview

Death-Oji is a gugugaga live streaming management tool with:
- **Backend**: Rust (Axum) on port 11451
- **Frontend**: Astro + Tailwind CSS v4 (Dracula theme) on port 11452
- **Features**: QR code login, stream management, user info, auto-save settings, category selection

## Development Workflow

### Initial Setup

```bash
# Backend
cd backend
cargo build
cargo run

# Frontend (separate terminal)
cd frontend
bun install
bun run dev
```

### Development Commands

```bash
# Backend
cargo run              # Run dev server
cargo build --release  # Production build
cargo test            # Run tests

# Frontend
bun run dev           # Dev server (port 11452)
bun run build         # Production build
bun run preview       # Preview production build
```

### Project Structure

```
death-oji/
├── backend/
│   ├── src/
│   │   ├── main.rs        # Server setup, routes
│   │   ├── handlers.rs    # API handlers
│   │   ├── gugugaga.rs    # gugugaga API client
│   │   └── models.rs      # Data structures
│   ├── partition.json     # Category data
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── pages/
│   │   │   └── index.astro  # Main UI
│   │   └── styles/
│   │       └── global.css   # Tailwind imports
│   ├── astro.config.mjs
│   └── package.json
└── README.md
```

## Key Implementation Details

### Backend (Rust)

**API Endpoints:**
- `GET /api/qrcode/generate` - Generate QR code for login
- `POST /api/qrcode/poll` - Poll QR code login status
- `POST /api/stream/start` - Start stream, returns RTMP credentials
- `POST /api/stream/stop` - Stop stream
- `PUT /api/stream/update` - Update stream title
- `POST /api/user/info` - Get user profile (includes UID)
- `GET /api/partitions` - Get category list from partition.json

**Important Notes:**
- All gugugaga API requests require Cookie and CSRF token headers
- App signing uses MD5 hash with wbi_key
- partition.json must be in backend root directory
- QR login polls every 2 seconds, auto-fills credentials on success

### Frontend (Astro)

**Key Features:**
- QR code login with toggle switch (Manual/QR mode)
- Dracula theme colors throughout
- Auto-save stream settings to localStorage
- Responsive 3-column layout with flexbox
- All blocks stretch to same height with buttons at bottom
- Collapsible usage guide at page bottom

**State Management:**
- `userUid` - Stored when user info loads, used for space URL
- `partitionsData` - Loaded from API, used for category dropdowns
- `qrPollInterval` - Interval for polling QR login status
- localStorage saves: title, mainCategory, areaId

**Important CSS Classes:**
- Blocks: `flex flex-col` with `flex-1` spacer before buttons
- Buttons: `min-h-10` for consistent height, `flex-1` for equal width
- Container: `items-stretch` to align all blocks

## Common Development Tasks

### Adding New API Endpoint

1. Add handler in `backend/src/handlers.rs`
2. Add route in `backend/src/main.rs`
3. Update frontend API calls in `<script>` section

### Modifying UI Layout

- All blocks use same structure: `flex flex-col` with `<div class="flex-1"></div>` spacer
- Buttons at bottom use `flex gap-2` with `min-h-10`
- Colors follow Dracula theme (see global.css)

### Updating Categories

Edit `backend/partition.json` - format:
```json
{
  "data": [
    {
      "id": "1",
      "name": "Category Name",
      "list": [
        {"id": "100", "name": "Sub Category"}
      ]
    }
  ]
}
```

## Deployment

### Docker Compose (Recommended)

```bash
# Build and start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Rebuild after changes
docker-compose up -d --build
```

**Ports:**
- Backend: http://localhost:11451
- Frontend: http://localhost:11452

### Manual Build

```bash
# Backend
cd backend
cargo build --release
./target/release/death-oji-backend

# Frontend
cd frontend
bun run build
```

### Production Checklist

- [ ] Update API_BASE in frontend to production backend URL
- [ ] Configure CORS in backend for production domain
- [ ] Ensure partition.json is in backend directory

## Troubleshooting

### Backend Issues

**Port already in use:**
```bash
lsof -ti:11451 | xargs kill -9
```

**Partition loading fails:**
- Ensure partition.json is in backend root directory
- Check JSON format is valid

### Frontend Issues

**Tailwind not working:**
- Ensure using v4 syntax: `@import "tailwindcss";`
- Check tailwindcss plugin in astro.config.mjs

**Sub-category not saving:**
- Increase timeout in loadStreamSettings (currently 500ms)
- Check localStorage in browser DevTools

**Avatar not loading:**
- Check referrerpolicy="no-referrer" is set
- Verify user info API returns valid face URL

## Testing

### Manual Testing Checklist

- [ ] QR code login generates and displays QR code
- [ ] QR code polling updates status correctly
- [ ] QR login auto-fills credentials on success
- [ ] Toggle between Manual/QR login modes works
- [ ] Load/Save credentials to JSON file
- [ ] Check user info loads profile and shows link buttons
- [ ] Select main category populates sub-categories
- [ ] Settings auto-save and restore on refresh
- [ ] Start stream returns RTMP credentials
- [ ] Stop stream works
- [ ] Open gugugaga Space/Live Room buttons work
- [ ] All blocks align at same height
- [ ] Buttons have consistent height
- [ ] Usage guide expands/collapses

### API Testing

```bash
# Health check
curl http://localhost:11451/health

# Get partitions
curl http://localhost:11451/api/partitions

# Generate QR code
curl http://localhost:11451/api/qrcode/generate

# Poll QR code (replace KEY with actual qrcode_key)
curl -X POST http://localhost:11451/api/qrcode/poll \
  -H "Content-Type: application/json" \
  -d '{"qrcode_key":"KEY"}'
```

## Code Style

### Rust
- Use `async/await` for all handlers
- Return `Result<Json<T>, (StatusCode, Json<ErrorResponse>)>`
- Add proper error messages

### Frontend
- Use Dracula theme colors consistently
- All buttons: `min-h-10 font-semibold`
- Spacing: `gap-2` for buttons, `gap-6` for blocks
- Keep JavaScript in single `<script>` block

## Git Workflow

```bash
# Feature development
git checkout -b feature/name
# Make changes
git add .
git commit -m "feat: description"
git push origin feature/name

# Create PR to master branch
```

## Resources

- [Axum Documentation](https://docs.rs/axum)
- [Astro Documentation](https://docs.astro.build)
- [Tailwind CSS v4](https://tailwindcss.com/docs)
- [Dracula Theme](https://draculatheme.com/contribute)
