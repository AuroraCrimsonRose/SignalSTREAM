# SignalSTREAM

**Modular, multithreaded Rust-based audio streaming engine** built by **CATALYSTS LABS**  
A modern, drop-in alternative to Icecast + Liquidsoap, designed for:

- Web-based radio broadcasting
- Live DJ input routing
- Real-time metadata and listener messaging
- Future SignalFrame integration

> **Build it. Stream it. Own it.**

---

## 🎧 Features

✅ Serve raw MP3 streams at `/station.mp3`  
✅ Real-time metadata and now playing API  
✅ Live DJ input switching (OBS, butt, ffmpeg)  
✅ Listener message push system (WebSocket)  
✅ Built-in shell control + optional admin panel UI  
✅ Multithreaded audio routing + per-station isolation  
✅ Embedded player support (HTML5 audio compatible)  
✅ Full Rust async runtime (Tokio)  
✅ Log system from day one (system, station, error)

---

## 🚀 Getting Started

```bash
git clone https://github.com/catalystslabs/signalstream.git
cd signalstream
cargo build --release
Start with a basic station config:

json
Copy
Edit
// stations/lofi.json
{
  "id": "lofi",
  "mount": "/lofi.mp3",
  "playlist": "audio/lofi/playlist.m3u",
  "fallback": "audio/lofi/fallback.mp3",
  "enable_live": true,
  "crossfade": 3.0
}
Then run:

bash
Copy
Edit
cargo run
Access stream at:
http://localhost:9090/lofi.mp3

🧠 Architecture
Stations: hot-reloadable from stations/*.json

Router: auto-switches between playlist, live input, fallback

Listeners: each gets a ring buffer for smooth playback

WebSocket: real-time metadata & DJ messages to web clients

Shell: local + remote CLI for control and status

📁 Project Structure
bash
Copy
Edit
src/
├── station/     # Station logic, routing, buffer, metadata
├── server/      # HTTP + WebSocket serving
├── shell/       # CLI interface
├── logger.rs    # Core logging
├── config.rs    # Config file loader
├── api.rs       # Shared logic between shell and server

🛡️ License & Redistribution Policy
SignalSTREAM is licensed under the MIT License.
However, redistribution of this code — including public forks or reuploads — is not permitted
without explicit permission from the author.

You may:

Clone and modify it for personal or private use

Use it internally within your organization or project

You may not:

Publicly reupload this repository or modified versions

Rebrand it and distribute it under another name

Claim the original work as your own

For full terms, see TRADEMARK.md

🤝 Contributing
We welcome code patches, feedback, and feature requests.
If you'd like to contribute, please open an issue or send your patch to:

📧 aurora@catalystslabs.com

🧑‍💻 Author
Aurora Rosabella
Founder — CATALYSTS LABS
🌐 catalystslabs.com

💬 Contact
Email: aurora@catalystslabs.com

Discord: coming soon

Mastodon: 

© 2025 CATALYSTS LABS — All rights reserved.
