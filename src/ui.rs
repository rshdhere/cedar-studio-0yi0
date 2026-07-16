pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Cedar Chat</title>
  <style>
    :root {
      color-scheme: light dark;
      --bg: #0f1419;
      --panel: #1a2332;
      --panel-2: #243044;
      --text: #e7edf5;
      --muted: #8b9bb4;
      --accent: #3dd6a8;
      --accent-2: #5b8def;
      --danger: #ff6b6b;
      --border: rgba(255, 255, 255, 0.08);
      --shadow: 0 18px 50px rgba(0, 0, 0, 0.35);
      font-family: "Segoe UI", system-ui, sans-serif;
    }

    * { box-sizing: border-box; }

    body {
      margin: 0;
      min-height: 100vh;
      background:
        radial-gradient(circle at top left, rgba(61, 214, 168, 0.18), transparent 28%),
        radial-gradient(circle at top right, rgba(91, 141, 239, 0.18), transparent 30%),
        var(--bg);
      color: var(--text);
      display: grid;
      place-items: center;
      padding: 24px;
    }

    .app {
      width: min(960px, 100%);
      background: rgba(26, 35, 50, 0.92);
      border: 1px solid var(--border);
      border-radius: 20px;
      box-shadow: var(--shadow);
      overflow: hidden;
      display: grid;
      grid-template-rows: auto 1fr auto;
      min-height: min(82vh, 760px);
      backdrop-filter: blur(12px);
    }

    header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 16px;
      padding: 20px 24px;
      border-bottom: 1px solid var(--border);
      background: linear-gradient(180deg, rgba(255,255,255,0.04), transparent);
    }

    header h1 {
      margin: 0;
      font-size: 1.25rem;
      letter-spacing: 0.02em;
    }

    header p {
      margin: 4px 0 0;
      color: var(--muted);
      font-size: 0.92rem;
    }

    .status {
      display: inline-flex;
      align-items: center;
      gap: 8px;
      padding: 8px 12px;
      border-radius: 999px;
      background: var(--panel-2);
      font-size: 0.85rem;
      color: var(--muted);
    }

    .status-dot {
      width: 10px;
      height: 10px;
      border-radius: 50%;
      background: var(--danger);
      box-shadow: 0 0 0 4px rgba(255, 107, 107, 0.15);
    }

    .status.connected .status-dot {
      background: var(--accent);
      box-shadow: 0 0 0 4px rgba(61, 214, 168, 0.15);
    }

    .join-panel,
    .chat-panel {
      padding: 24px;
    }

    .join-panel {
      display: grid;
      gap: 16px;
      max-width: 420px;
      margin: auto;
      text-align: center;
    }

    .join-panel h2 {
      margin: 0;
      font-size: 1.5rem;
    }

    .join-panel p {
      margin: 0;
      color: var(--muted);
      line-height: 1.5;
    }

    .field-row {
      display: flex;
      gap: 12px;
    }

    input, button {
      font: inherit;
    }

    input {
      flex: 1;
      border: 1px solid var(--border);
      background: var(--panel-2);
      color: var(--text);
      border-radius: 12px;
      padding: 14px 16px;
      outline: none;
    }

    input:focus {
      border-color: rgba(61, 214, 168, 0.55);
      box-shadow: 0 0 0 3px rgba(61, 214, 168, 0.12);
    }

    button {
      border: none;
      border-radius: 12px;
      padding: 14px 18px;
      background: linear-gradient(135deg, var(--accent), #24b88f);
      color: #062018;
      font-weight: 700;
      cursor: pointer;
    }

    button:disabled {
      opacity: 0.55;
      cursor: not-allowed;
    }

    .chat-panel {
      display: none;
      grid-template-rows: 1fr auto;
      gap: 16px;
      min-height: 0;
    }

    .chat-panel.active {
      display: grid;
    }

    .messages {
      overflow: auto;
      display: flex;
      flex-direction: column;
      gap: 12px;
      padding-right: 4px;
      min-height: 0;
    }

    .message,
    .system {
      max-width: 78%;
      padding: 12px 14px;
      border-radius: 16px;
      line-height: 1.45;
      word-break: break-word;
    }

    .message {
      background: var(--panel-2);
      border: 1px solid var(--border);
    }

    .message.self {
      margin-left: auto;
      background: rgba(61, 214, 168, 0.14);
      border-color: rgba(61, 214, 168, 0.25);
    }

    .message .meta {
      display: flex;
      justify-content: space-between;
      gap: 12px;
      margin-bottom: 6px;
      font-size: 0.78rem;
      color: var(--muted);
    }

    .message .author {
      color: var(--accent-2);
      font-weight: 700;
    }

    .system {
      align-self: center;
      background: rgba(255, 255, 255, 0.04);
      color: var(--muted);
      font-size: 0.88rem;
      border: 1px dashed var(--border);
    }

    .composer {
      display: flex;
      gap: 12px;
      border-top: 1px solid var(--border);
      padding-top: 16px;
    }

    .error {
      color: #ffb4b4;
      min-height: 1.2em;
      font-size: 0.9rem;
    }

    @media (max-width: 640px) {
      .field-row, .composer { flex-direction: column; }
      .message, .system { max-width: 100%; }
    }
  </style>
</head>
<body>
  <main class="app">
    <header>
      <div>
        <h1>Cedar Chat</h1>
        <p>Real-time room powered by Rust, Tokio, and WebSockets.</p>
      </div>
      <div id="status" class="status">
        <span class="status-dot"></span>
        <span id="status-text">Connecting…</span>
      </div>
    </header>

    <section id="join-panel" class="join-panel">
      <h2>Join the room</h2>
      <p>Pick a display name to start chatting with everyone connected to this server.</p>
      <div class="field-row">
        <input id="username-input" maxlength="24" placeholder="Your name" autocomplete="username">
        <button id="join-btn" type="button">Enter chat</button>
      </div>
      <div id="join-error" class="error"></div>
    </section>

    <section id="chat-panel" class="chat-panel">
      <div id="messages" class="messages" aria-live="polite"></div>
      <form id="composer" class="composer">
        <input id="message-input" maxlength="2000" placeholder="Write a message…" autocomplete="off">
        <button id="send-btn" type="submit">Send</button>
      </form>
    </section>
  </main>

  <script>
    const statusEl = document.getElementById('status');
    const statusText = document.getElementById('status-text');
    const joinPanel = document.getElementById('join-panel');
    const chatPanel = document.getElementById('chat-panel');
    const usernameInput = document.getElementById('username-input');
    const joinBtn = document.getElementById('join-btn');
    const joinError = document.getElementById('join-error');
    const messagesEl = document.getElementById('messages');
    const composer = document.getElementById('composer');
    const messageInput = document.getElementById('message-input');
    const sendBtn = document.getElementById('send-btn');

    let socket = null;
    let currentUser = null;
    let joined = false;

    function setStatus(connected, label) {
      statusEl.classList.toggle('connected', connected);
      statusText.textContent = label;
    }

    function formatTime(ts) {
      return new Date(ts).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }

    function appendSystem(text) {
      const el = document.createElement('div');
      el.className = 'system';
      el.textContent = text;
      messagesEl.appendChild(el);
      messagesEl.scrollTop = messagesEl.scrollHeight;
    }

    function appendMessage({ user, text, ts }, self = false) {
      const el = document.createElement('article');
      el.className = 'message' + (self ? ' self' : '');
      el.innerHTML = `
        <div class="meta">
          <span class="author">${escapeHtml(user)}</span>
          <time>${formatTime(ts)}</time>
        </div>
        <div class="body">${escapeHtml(text)}</div>
      `;
      messagesEl.appendChild(el);
      messagesEl.scrollTop = messagesEl.scrollHeight;
    }

    function escapeHtml(value) {
      return value
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;')
        .replaceAll('"', '&quot;')
        .replaceAll("'", '&#39;');
    }

    function send(payload) {
      if (!socket || socket.readyState !== WebSocket.OPEN) return;
      socket.send(JSON.stringify(payload));
    }

    function connect() {
      const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
      socket = new WebSocket(`${protocol}//${location.host}/ws`);

      socket.addEventListener('open', () => {
        setStatus(true, 'Connected');
        const saved = localStorage.getItem('cedar-chat-user');
        if (saved && !joined) {
          usernameInput.value = saved;
        }
      });
      socket.addEventListener('close', () => {
        joined = false;
        setStatus(false, 'Disconnected');
        joinPanel.style.display = 'grid';
        chatPanel.classList.remove('active');
        joinError.textContent = 'Connection lost. Refresh to reconnect.';
      });

      socket.addEventListener('message', (event) => {
        let data;
        try {
          data = JSON.parse(event.data);
        } catch {
          return;
        }

        switch (data.type) {
          case 'history':
            messagesEl.innerHTML = '';
            for (const item of data.messages) {
              if (item.type === 'message') {
                appendMessage(item, item.user === currentUser);
              }
            }
            break;
          case 'message':
            appendMessage(data, data.user === currentUser);
            break;
          case 'join':
            appendSystem(`${data.user} joined the room`);
            break;
          case 'leave':
            appendSystem(`${data.user} left the room`);
            break;
          case 'error':
            if (!joined) {
              joinError.textContent = data.message;
            } else {
              appendSystem(data.message);
            }
            break;
        }
      });
    }

    joinBtn.addEventListener('click', () => {
      joinError.textContent = '';
      const user = usernameInput.value.trim();
      if (!user) {
        joinError.textContent = 'Enter a username first.';
        return;
      }
      currentUser = user;
      localStorage.setItem('cedar-chat-user', user);
      send({ type: 'join', user });
      joined = true;
      joinPanel.style.display = 'none';
      chatPanel.classList.add('active');
      messageInput.focus();
    });

    composer.addEventListener('submit', (event) => {
      event.preventDefault();
      const text = messageInput.value.trim();
      if (!text || !joined) return;
      send({ type: 'chat', text });
      messageInput.value = '';
    });

    usernameInput.addEventListener('keydown', (event) => {
      if (event.key === 'Enter') joinBtn.click();
    });

    connect();
  </script>
</body>
</html>
"#;
