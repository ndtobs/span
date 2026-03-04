<script lang="ts">
  import { sessionStore } from "$lib/stores/sessions";

  let now = $state(Date.now());

  // Update clock every second
  $effect(() => {
    const interval = setInterval(() => {
      now = Date.now();
    }, 1000);
    return () => clearInterval(interval);
  });

  function elapsed(startedAt: number): string {
    const seconds = Math.floor((now - startedAt) / 1000);
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    if (h > 0) return `${h}h ${m}m`;
    if (m > 0) return `${m}m ${s}s`;
    return `${s}s`;
  }
</script>

<div class="statusbar">
  <div class="status-left">
    {#if sessionStore.activeSession}
      <span class="status-item">
        {sessionStore.activeSession.status === "connected" ? "🟢" : "⚪"}
        {sessionStore.activeSession.name}
      </span>
      <span class="status-item muted">
        {elapsed(sessionStore.activeSession.startedAt)}
      </span>
    {:else}
      <span class="status-item muted">No active session</span>
    {/if}
  </div>

  <div class="status-right">
    <span class="status-item muted">
      {sessionStore.sessions.length} session{sessionStore.sessions.length !== 1 ? "s" : ""}
    </span>
    <span class="status-item muted">UTF-8</span>
  </div>
</div>

<style>
  .statusbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: var(--statusbar-height);
    background: var(--bg-secondary);
    border-top: 1px solid var(--border);
    padding: 0 var(--space-md);
    font-size: 11px;
    user-select: none;
  }

  .status-left,
  .status-right {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .status-item {
    color: var(--text-secondary);
  }

  .muted {
    color: var(--text-muted);
  }
</style>
