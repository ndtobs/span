<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { sessionStore } from "$lib/stores/sessions.svelte";
  import { inventoryStore } from "$lib/stores/inventory.svelte";
  import type { InventoryFolder, InventoryDevice } from "$lib/types";

  interface Props {
    onadddevice?: () => void;
  }

  let { onadddevice }: Props = $props();

  async function handleDeviceClick(device: InventoryDevice) {
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
    } catch (error) {
      console.error("Failed to connect:", error);
      sessionStore.updateStatus(sessionId, "error");
    }
  }
</script>

<aside class="sidebar">
  <div class="sidebar-header">
    <span class="section-title">Inventory</span>
    <button class="icon-btn" title="Add device" onclick={() => onadddevice?.()}>+</button>
  </div>

  <div class="search-box">
    <input
      type="text"
      placeholder="Search devices..."
      bind:value={inventoryStore.searchQuery}
    />
  </div>

  <div class="device-tree">
    {#if inventoryStore.searchQuery}
      <!-- Flat search results -->
      {#each inventoryStore.filteredDevices as device}
        <button
          class="device-item"
          onclick={() => handleDeviceClick(device)}
        >
          <span class="device-icon">🖥</span>
          <span class="device-name">{device.name}</span>
          <span class="device-host">{device.connectionConfig.host}</span>
        </button>
      {:else}
        <div class="empty">No devices found</div>
      {/each}
    {:else if inventoryStore.folders.length === 0}
      <div class="empty">
        <p>No devices yet</p>
        <p class="hint">Add devices or import from SecureCRT</p>
      </div>
    {:else}
      <!-- Tree view -->
      {#each inventoryStore.folders as folder}
        {@render folderNode(folder)}
      {/each}
    {/if}
  </div>

  <div class="sidebar-footer">
    <button class="footer-btn" title="Import sessions">📥 Import</button>
    <button class="footer-btn" title="Settings">⚙ Settings</button>
  </div>
</aside>

{#snippet folderNode(folder: InventoryFolder)}
  <div class="folder">
    <button
      class="folder-header"
      onclick={() => inventoryStore.toggleFolder(folder.id)}
    >
      <span class="folder-arrow">{folder.expanded ? "▾" : "▸"}</span>
      <span class="folder-icon">📁</span>
      <span class="folder-name">{folder.name}</span>
      <span class="folder-count">{folder.devices.length}</span>
    </button>

    {#if folder.expanded}
      <div class="folder-children">
        {#each folder.children as child}
          {@render folderNode(child)}
        {/each}
        {#each folder.devices as device}
          <button
            class="device-item"
            onclick={() => handleDeviceClick(device)}
          >
            <span class="device-icon">🖥</span>
            <span class="device-name">{device.name}</span>
          </button>
        {/each}
      </div>
    {/if}
  </div>
{/snippet}

<style>
  .sidebar {
    width: var(--sidebar-width);
    min-width: var(--sidebar-width);
    background: var(--bg-secondary);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm) var(--space-md);
    border-bottom: 1px solid var(--border);
  }

  .section-title {
    font-size: var(--font-size-sm);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-muted);
  }

  .icon-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 16px;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
  }

  .icon-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .search-box {
    padding: var(--space-sm) var(--space-md);
  }

  .search-box input {
    width: 100%;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: var(--space-xs) var(--space-sm);
    color: var(--text-primary);
    font-family: var(--font-ui);
    font-size: var(--font-size-sm);
    outline: none;
  }

  .search-box input:focus {
    border-color: var(--accent);
  }

  .search-box input::placeholder {
    color: var(--text-muted);
  }

  .device-tree {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-xs) 0;
  }

  .folder-header {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    width: 100%;
    padding: var(--space-xs) var(--space-md);
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    text-align: left;
  }

  .folder-header:hover {
    background: var(--bg-hover);
  }

  .folder-arrow {
    font-size: 10px;
    width: 12px;
  }

  .folder-count {
    color: var(--text-muted);
    font-size: 11px;
    margin-left: auto;
  }

  .folder-children {
    padding-left: var(--space-md);
  }

  .device-item {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    width: 100%;
    padding: var(--space-xs) var(--space-md);
    padding-left: calc(var(--space-md) + 12px);
    background: none;
    border: none;
    color: var(--text-primary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    text-align: left;
  }

  .device-item:hover {
    background: var(--bg-hover);
  }

  .device-host {
    color: var(--text-muted);
    font-size: 11px;
    margin-left: auto;
    font-family: var(--font-mono);
  }

  .empty {
    padding: var(--space-xl) var(--space-md);
    text-align: center;
    color: var(--text-muted);
    font-size: var(--font-size-sm);
  }

  .hint {
    font-size: 11px;
    margin-top: var(--space-xs);
  }

  .sidebar-footer {
    display: flex;
    gap: var(--space-xs);
    padding: var(--space-sm) var(--space-md);
    border-top: 1px solid var(--border);
  }

  .footer-btn {
    flex: 1;
    background: none;
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: var(--space-xs) var(--space-sm);
    color: var(--text-muted);
    cursor: pointer;
    font-size: 11px;
  }

  .footer-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }
</style>
