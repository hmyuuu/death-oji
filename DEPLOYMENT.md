# Deployment Guide

This guide covers deploying Death-Oji using a hybrid approach:
- **Frontend (Astro)** → Vercel
- **Backend (Rust)** → Railway

## Prerequisites

- [Vercel Account](https://vercel.com/signup)
- [Railway Account](https://railway.app/)
- [GitHub Account](https://github.com/) (recommended for automatic deployments)

## Part 1: Deploy Backend to Railway

### Option A: Deploy via Railway CLI (Recommended)

1. **Install Railway CLI**
   ```bash
   bun install -g @railway/cli
   ```

2. **Login to Railway**
   ```bash
   railway login
   ```

3. **Initialize Railway project**
   ```bash
   cd backend
   railway init
   ```

4. **Deploy the backend**
   ```bash
   railway up
   ```

5. **Generate a public domain**
   ```bash
   railway domain
   ```

   Railway will provide a URL like: `https://your-app.up.railway.app`

6. **Verify deployment**
   ```bash
   curl https://your-app.up.railway.app/health
   ```
   Should return: `OK`

### Option B: Deploy via Railway Dashboard

1. Go to [Railway Dashboard](https://railway.app/dashboard)
2. Click **"New Project"** → **"Deploy from GitHub repo"**
3. Select your repository
4. Configure build settings:
   - **Root Directory**: `backend`
   - **Build Command**: `cargo build --release`
   - **Start Command**: `./target/release/death-oji-backend`
5. Click **"Deploy"**
6. Go to **Settings** → **Networking** → **Generate Domain**
7. Copy your Railway URL (e.g., `https://your-app.up.railway.app`)

### Railway Configuration

The `railway.toml` file is already configured with:
- Health check endpoint: `/health`
- Auto-restart on failure
- Proper build and start commands

## Part 2: Deploy Frontend to Vercel

### Option A: Deploy via Vercel CLI

1. **Install Vercel CLI**
   ```bash
   bun install -g vercel
   ```

2. **Login to Vercel**
   ```bash
   vercel login
   ```

3. **Set environment variable**
   ```bash
   cd frontend
   cp .env.example .env
   # Edit .env and set PUBLIC_API_BASE to your Railway URL
   ```

4. **Deploy to Vercel**
   ```bash
   vercel
   ```

   For production deployment:
   ```bash
   vercel --prod
   ```

5. **Set production environment variable**
   ```bash
   vercel env add PUBLIC_API_BASE production
   # Enter your Railway URL when prompted: https://your-app.up.railway.app
   ```

### Option B: Deploy via Vercel Dashboard

1. Go to [Vercel Dashboard](https://vercel.com/dashboard)
2. Click **"Add New..."** → **"Project"**
3. Import your GitHub repository
4. Configure project:
   - **Framework Preset**: Astro
   - **Root Directory**: `frontend`
   - **Build Command**: `bun run build` (or leave default)
   - **Output Directory**: `dist` (or leave default)
5. Add Environment Variable:
   - **Key**: `PUBLIC_API_BASE`
   - **Value**: `https://your-app.up.railway.app` (your Railway URL)
6. Click **"Deploy"**

### Update vercel.json (Optional)

If you want to use Vercel's proxy feature instead of environment variables, update `vercel.json`:

```json
{
  "buildCommand": "cd frontend && bun run build",
  "outputDirectory": "frontend/dist",
  "installCommand": "cd frontend && bun install",
  "framework": "astro",
  "rewrites": [
    {
      "source": "/api/:path*",
      "destination": "https://your-actual-railway-url.up.railway.app/api/:path*"
    },
    {
      "source": "/health",
      "destination": "https://your-actual-railway-url.up.railway.app/health"
    }
  ]
}
```

Replace `your-actual-railway-url.up.railway.app` with your actual Railway domain.

## Part 3: Verify Deployment

1. **Test Backend**
   ```bash
   curl https://your-app.up.railway.app/health
   curl https://your-app.up.railway.app/api/partitions
   ```

2. **Test Frontend**
   - Visit your Vercel URL (e.g., `https://your-app.vercel.app`)
   - Try QR code login
   - Check if categories load properly
   - Verify all API calls work

## Environment Variables Reference

### Frontend (.env)

```bash
PUBLIC_API_BASE=https://your-app.up.railway.app
```

### Backend (Railway)

No environment variables needed by default. The backend uses:
- Port: `11451` (automatically exposed)
- CORS: Allows all origins (configured in `main.rs`)

## Troubleshooting

### Backend Issues

**502 Bad Gateway / Application failed to respond:**
- **Issue**: Backend not binding to Railway's dynamic PORT
- **Solution**: The backend must read the `PORT` environment variable
  ```rust
  // In backend/src/main.rs
  let port = std::env::var("PORT").unwrap_or_else(|_| "11451".to_string());
  let addr = format!("0.0.0.0:{}", port);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  ```
- **Verify**: Check Railway logs with `railway logs` or in the dashboard

**Build fails on Railway:**
- Check Railway build logs in the dashboard
- Ensure `partition.json` exists in backend directory
- Verify `Cargo.lock` is committed to git
- Ensure `railway.toml` is in the `backend` directory
- Set **Root Directory** to `backend` in Railway settings

**Missing OpenSSL libraries:**
- Ensure Dockerfile includes: `RUN apt-get update && apt-get install -y ca-certificates libssl3`
- Required for HTTPS requests to external APIs

**CORS errors:**
The backend already allows all origins. If you still see CORS errors, check Railway logs.

### Frontend Issues

**API calls return "Unexpected token" or JSON parse errors:**
- **Issue**: Frontend is calling wrong API endpoint (missing `https://` protocol)
- **Solution**: Ensure `PUBLIC_API_BASE` includes the full URL with protocol
  - ✅ Correct: `https://your-app.up.railway.app`
  - ❌ Wrong: `your-app.up.railway.app`
  - ❌ Wrong: `http://localhost:11451`

**API calls return 404 NOT_FOUND:**
- **Issue**: Frontend is calling Vercel instead of Railway backend
- **Solution**:
  1. Set `PUBLIC_API_BASE` environment variable in Vercel with **full URL including https://**
  2. Redeploy the frontend (environment variables only apply on build)
  3. Verify in browser console or check the bundled JavaScript:
     ```bash
     curl -s https://your-vercel-app.com/ | grep -o 'const l="[^"]*"'
     # Should show: const l="https://your-app.up.railway.app"
     ```

**Environment variable not updating:**
```bash
# Remove old variable
vercel env rm PUBLIC_API_BASE production

# Add new variable with FULL URL including https://
vercel env add PUBLIC_API_BASE production
# Enter: https://your-app.up.railway.app

# Redeploy to apply changes
vercel --prod
```

**Vercel build fails - "cd: frontend: No such file or directory":**
- **Issue**: Root directory not set correctly
- **Solution**:
  1. Go to Vercel Dashboard → Settings → General
  2. Set **Root Directory** to `frontend`
  3. Ensure `vercel.json` is in the `frontend` directory (not project root)
  4. Redeploy

**Build fails:**
- Check Vercel build logs in the dashboard
- Ensure `bun` is available (Vercel auto-detects it from package.json)
- Verify `astro.config.mjs` has `output: 'static'`
- Ensure `frontend/vercel.json` exists with correct configuration

### Common Deployment Checklist

✅ **Backend (Railway)**
- [ ] Root Directory set to `backend`
- [ ] `railway.toml` is in `backend/` directory
- [ ] Backend binds to `PORT` environment variable
- [ ] Health endpoint returns `OK`: `curl https://your-app.up.railway.app/health`
- [ ] Partitions endpoint returns JSON: `curl https://your-app.up.railway.app/api/partitions`

✅ **Frontend (Vercel)**
- [ ] Root Directory set to `frontend`
- [ ] `vercel.json` is in `frontend/` directory
- [ ] Environment variable `PUBLIC_API_BASE` is set with **full URL including https://**
- [ ] Redeployed after setting environment variable
- [ ] Verify API_BASE in deployed code: `curl -s https://your-vercel-app.com/ | grep 'const l='`
- [ ] Browser console shows no CORS or 404 errors

## Updating Deployments

### Update Backend
```bash
cd backend
git add .
git commit -m "Update backend"
git push
# Railway auto-deploys on push (if connected to GitHub)
# Or manually: railway up
```

### Update Frontend
```bash
cd frontend
git add .
git commit -m "Update frontend"
git push
# Vercel auto-deploys on push (if connected to GitHub)
# Or manually: vercel --prod
```

## Alternative Backend Hosting Options

If Railway doesn't work for you, consider:

### Fly.io
```bash
# Install flyctl
curl -L https://fly.io/install.sh | sh

# Deploy
cd backend
fly launch
fly deploy
```

### Render
1. Go to [Render Dashboard](https://dashboard.render.com/)
2. New → Web Service
3. Connect repository
4. Configure:
   - **Build Command**: `cargo build --release`
   - **Start Command**: `./target/release/death-oji-backend`
   - **Root Directory**: `backend`

## Cost Estimates

- **Vercel**: Free tier includes 100GB bandwidth, unlimited deployments
- **Railway**: Free tier includes $5 credit/month, ~500 hours runtime
- **Total**: $0/month for hobby projects (within free tier limits)

## Security Notes

1. **Never commit sensitive data** (cookies, tokens) to git
2. **Use HTTPS** for all production deployments (both platforms provide this)
3. **CORS is wide open** - Consider restricting to your Vercel domain in production:
   ```rust
   // In backend/src/main.rs
   let cors = CorsLayer::new()
       .allow_origin("https://your-app.vercel.app".parse::<HeaderValue>().unwrap())
       .allow_methods(Any)
       .allow_headers(Any);
   ```

## Support

- Railway: https://railway.app/help
- Vercel: https://vercel.com/support
- Project Issues: https://github.com/your-repo/issues
