<script lang="ts">
  import { sessionStore } from "$lib/stores/sessions";

  function statusColor(status: string): string {
    switch (status) {
      case "connected":
        return "var(--success)";
      case "connecting":
        return "var(--warning)";
      case "error":
        return "var(--error)";
      default:
        return "var(--text-muted)";
    }
  }
</script>

<div class="tabbar">
  <div class="tabs">
    {#each sessionStore.sessions as session}
      <button
        class="tab"
        class:active={session.id === sessionStore.activeId}
        onclick={() => sessionStore.setActive(session.id)}
      >
        <span
          class="status-dot"
          style="background: {statusColor(session.status)}"
        ></span>
        <span class="tab-name">{session.name}</span>
        <button
          class="tab-close"
          onclick|stopPropagation={() => sessionStore.remove(session.id)}
          aria-label="Close tab"
        >
          ✕
        </button>
      </button>
    {/each}
  </div>

  <button
    class="new-tab"
    title="New connection (Ctrl+T)"
    aria-label="New tab"
  >
    +
  </button>
</div>

<style>
  .tabbar {
    display: flex;
    align-items: center;
    height: var(--tabbar-height);
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border);
    padding: 0 var(--space-xs);
    overflow-x: auto;
  }

  .tabs {
    display: flex;
    gap: 1px;
    flex: 1;
    min-width: 0;
    overflow-x: auto;
  }

  .tabs::-webkit-scrollbar {
    height: 0;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    padding: 0 var(--space-md);
    height: 28px;
    background: var(--bg-primary);
    border: none;
    border-radius: 6px 6px 0 0;
    color: var(--text-muted);
    cursor: pointer;
    font-size: var(--font-size-sm);
    font-family: var(--font-ui);
    white-space: nowrap;
    min-width: 100px;
    max-width: 200px;
  }

  .tab:hover {
    color: var(--text-secondary);
    background: var(--bg-tertiary);
  }

  .tab.active {
    background: var(--bg-primary);
    color: var(--text-primary);
    border-bottom: 2px solid var(--accent);
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .tab-name {
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    text-align: left;
  }

  .tab-close {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 10px;
    padding: 2px;
    border-radius: 3px;
    opacity: 0;
    flex-shrink: 0;
  }

  .tab:hover .tab-close {
    opacity: 1;
  }

  .tab-close:hover {
    background: var(--error);
    color: white;
    opacity: 1;
  }

  .new-tab {
    width: 28px;
    height: 28px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 16px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .new-tab:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }
</style>
