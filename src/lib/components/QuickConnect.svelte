<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { sessionStore } from "$lib/stores/sessions.svelte";
  import { inventoryStore } from "$lib/stores/inventory.svelte";

  interface Props {
    onclose: () => void;
  }

  let { onclose }: Props = $props();
  let inputEl: HTMLInputElement;
  let usernameEl: HTMLInputElement;
  let passwordEl: HTMLInputElement;
  let query = $state("");
  let step = $state<"host" | "credentials">("host");
  let username = $state("");
  let password = $state("");
  let connecting = $state(false);
  let parsedConnection = $state<{
    username: string;
    host: string;
    port: number;
  } | null>(null);

  // Parse quick connect input: user@host:port or just host
  function parseTarget(input: string): {
    username: string;
    host: string;
    port: number;
  } {
    let username = "";
    let host = input.trim().replace(/^ssh\s+/i, "");
    let port = 22;

    // user@host
    if (host.includes("@")) {
      const parts = host.split("@");
      username = parts[0];
      host = parts[1];
    }

    // host:port
    if (host.includes(":")) {
      const parts = host.split(":");
      host = parts[0];
      port = parseInt(parts[1], 10) || 22;
    }

    return { username, host, port };
  }

  function handleHostSubmit() {
    if (!query.trim()) return;

    // Check if it matches an inventory device first
    const device = inventoryStore.flatDevices.find(
      (d) =>
        d.name.toLowerCase() === query.toLowerCase() ||
        d.connectionConfig.host === query,
    );

    if (device) {
      // Connect using inventory config - assume it has credentials configured
      connectWithDevice(device);
    } else {
      // Parse input and move to credentials step
      const parsed = parseTarget(query);
      parsedConnection = parsed;
      username = parsed.username || "";
      step = "credentials";
    }
  }

  async function handleCredentialsSubmit() {
    if (!parsedConnection || !username.trim() || !password.trim()) return;

    await connectSSH(
      parsedConnection.host,
      parsedConnection.port,
      username,
      password,
    );
  }

  async function connectWithDevice(device: any) {
    connecting = true;
    try {
      const sessionId = crypto.randomUUID();
      
      sessionStore.add({
        id: sessionId,
        connectionId: device.connectionConfig.id,
        name: device.name,
        status: "connecting",
        startedAt: Date.now(),
      });

      await invoke("connect", {
        sessionId,
        host: device.connectionConfig.host,
        port: device.connectionConfig.port,
        username: device.connectionConfig.username,
      });

      onclose();
    } catch (error) {
      console.error("Failed to connect:", error);
      sessionStore.updateStatus(sessionId, "error");
    } finally {
      connecting = false;
    }
  }

  async function connectSSH(host: string, port: number, user: string, pass: string) {
    connecting = true;
    try {
      const sessionId = crypto.randomUUID();
      
      sessionStore.add({
        id: sessionId,
        connectionId: "",
        name: host,
        status: "connecting",
        startedAt: Date.now(),
      });

      await invoke("connect", {
        sessionId,
        host,
        port,
        username: user,
        password: pass,
      });

      onclose();
    } catch (error) {
      console.error("Failed to connect:", error);
      // Find the session and update its status
      const sessions = sessionStore.sessions;
      const session = sessions.find(s => s.name === host);
      if (session) {
        sessionStore.updateStatus(session.id, "error");
      }
    } finally {
      connecting = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onclose();
    }
  }

  function goBack() {
    step = "host";
    username = "";
    password = "";
  }

  $effect(() => {
    // Auto-focus appropriate input when step changes
    if (step === "host") {
      inputEl?.focus();
    } else if (step === "credentials" && !username) {
      usernameEl?.focus();
    } else if (step === "credentials" && username && !password) {
      passwordEl?.focus();
    }
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" onclick={onclose}>
  <div class="quick-connect" onclick={(e: MouseEvent) => e.stopPropagation()}>
    {#if step === "host"}
      <form onsubmit={(e: SubmitEvent) => { e.preventDefault(); handleHostSubmit(); }}>
        <input
          bind:this={inputEl}
          bind:value={query}
          type="text"
          placeholder="Connect: hostname, user@host:port, or device name..."
          spellcheck="false"
          autocomplete="off"
        />
      </form>

      {#if query && inventoryStore.filteredDevices.length > 0}
        <div class="suggestions">
          {#each inventoryStore.filteredDevices.slice(0, 8) as device}
            <button
              class="suggestion"
              onclick={() => {
                query = device.name;
                handleHostSubmit();
              }}
            >
              <span class="suggestion-name">{device.name}</span>
              <span class="suggestion-host"
                >{device.connectionConfig.host}</span
              >
            </button>
          {/each}
        </div>
      {/if}
    {:else if step === "credentials"}
      <div class="credentials-header">
        <button class="back-btn" onclick={goBack}>←</button>
        <span class="host-info">
          {parsedConnection?.username ? `${parsedConnection.username}@` : ""}{parsedConnection?.host}:{parsedConnection?.port}
        </span>
      </div>
      
      <form onsubmit={(e: SubmitEvent) => { e.preventDefault(); handleCredentialsSubmit(); }}>
        <input
          bind:this={usernameEl}
          bind:value={username}
          type="text"
          placeholder="Username"
          spellcheck="false"
          autocomplete="username"
          disabled={connecting}
        />
        <input
          bind:this={passwordEl}
          bind:value={password}
          type="password"
          placeholder="Password"
          autocomplete="current-password"
          disabled={connecting}
        />
        
        <div class="actions">
          <button type="button" onclick={goBack} disabled={connecting}>
            Cancel
          </button>
          <button type="submit" disabled={connecting || !username.trim() || !password.trim()}>
            {connecting ? "Connecting..." : "Connect"}
          </button>
        </div>
      </form>
    {/if}
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 20vh;
    z-index: 100;
  }

  .quick-connect {
    width: 560px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
    overflow: hidden;
  }

  input {
    width: 100%;
    padding: var(--space-md) var(--space-lg);
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-family: var(--font-mono);
    font-size: var(--font-size-lg);
    outline: none;
  }

  input::placeholder {
    color: var(--text-muted);
  }

  .suggestions {
    border-top: 1px solid var(--border);
    max-height: 300px;
    overflow-y: auto;
  }

  .suggestion {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: var(--space-sm) var(--space-lg);
    background: none;
    border: none;
    color: var(--text-primary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    text-align: left;
  }

  .suggestion:hover {
    background: var(--bg-hover);
  }

  .suggestion-host {
    color: var(--text-muted);
    font-family: var(--font-mono);
    font-size: 11px;
  }

  .credentials-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-md) var(--space-lg);
    border-bottom: 1px solid var(--border);
    background: var(--bg-primary);
  }

  .back-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: var(--font-size-lg);
    padding: var(--space-xs);
    border-radius: 4px;
  }

  .back-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .host-info {
    color: var(--text-secondary);
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
  }

  .credentials-header + form input {
    border-top: none;
    border-radius: 0;
  }

  .credentials-header + form input:first-child {
    border-top: 1px solid var(--border);
  }

  .actions {
    display: flex;
    gap: var(--space-sm);
    padding: var(--space-md) var(--space-lg);
    border-top: 1px solid var(--border);
    background: var(--bg-primary);
  }

  .actions button {
    flex: 1;
    padding: var(--space-sm) var(--space-md);
    border-radius: 4px;
    font-size: var(--font-size-sm);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .actions button[type="button"] {
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    color: var(--text-secondary);
  }

  .actions button[type="button"]:hover:not(:disabled) {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .actions button[type="submit"] {
    background: var(--accent);
    border: 1px solid var(--accent);
    color: white;
  }

  .actions button[type="submit"]:hover:not(:disabled) {
    background: var(--accent-hover);
    border-color: var(--accent-hover);
  }

  .actions button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
