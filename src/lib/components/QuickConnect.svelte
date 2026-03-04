<script lang="ts">
  import { sessionStore } from "$lib/stores/sessions";
  import { inventoryStore } from "$lib/stores/inventory";

  interface Props {
    onclose: () => void;
  }

  let { onclose }: Props = $props();
  let inputEl: HTMLInputElement;
  let query = $state("");

  // Parse quick connect input: user@host:port or just host
  function parseTarget(input: string): {
    username: string;
    host: string;
    port: number;
  } {
    let username = "";
    let host = input.trim();
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

  function handleSubmit() {
    if (!query.trim()) return;

    // Check if it matches an inventory device first
    const device = inventoryStore.flatDevices.find(
      (d) =>
        d.name.toLowerCase() === query.toLowerCase() ||
        d.connectionConfig.host === query,
    );

    if (device) {
      // Connect using inventory config
      sessionStore.add({
        id: crypto.randomUUID(),
        connectionId: device.connectionConfig.id,
        name: device.name,
        status: "connecting",
        startedAt: Date.now(),
      });
    } else {
      // Quick connect with parsed input
      const { username, host, port } = parseTarget(query);
      sessionStore.add({
        id: crypto.randomUUID(),
        connectionId: "",
        name: host,
        status: "connecting",
        startedAt: Date.now(),
      });

      // TODO: invoke Tauri command to connect
      // invoke("ssh_connect", { host, port, username });
    }

    onclose();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onclose();
    }
  }

  $effect(() => {
    // Auto-focus when opened
    inputEl?.focus();
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="overlay" onclick={onclose}>
  <div class="quick-connect" onclick={(e: MouseEvent) => e.stopPropagation()}>
    <form onsubmit={(e: SubmitEvent) => { e.preventDefault(); handleSubmit(); }}>
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
              handleSubmit();
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
</style>
