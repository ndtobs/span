<script lang="ts">
  // Custom title bar for frameless window (Tauri)
  // Provides drag region + window controls
</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="titlebar-left">
    <span class="app-name">⚡ Span</span>
  </div>
  <div class="titlebar-center" data-tauri-drag-region>
    <!-- Breadcrumb or session info can go here -->
  </div>
  <div class="titlebar-controls">
    <button class="control minimize" aria-label="Minimize">─</button>
    <button class="control maximize" aria-label="Maximize">□</button>
    <button class="control close" aria-label="Close">✕</button>
  </div>
</div>

<style>
  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: var(--titlebar-height);
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border);
    user-select: none;
    -webkit-user-select: none;
    padding: 0 var(--space-sm);
  }

  .titlebar-left {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .app-name {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-muted);
    letter-spacing: 1px;
    text-transform: uppercase;
  }

  .titlebar-center {
    flex: 1;
  }

  .titlebar-controls {
    display: flex;
    gap: 2px;
  }

  .control {
    width: 28px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 11px;
    border-radius: 4px;
  }

  .control:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .close:hover {
    background: var(--error);
    color: white;
  }
</style>
