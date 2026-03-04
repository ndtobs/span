<script lang="ts">
  import "./app.css";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import Terminal from "$lib/components/Terminal.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import QuickConnect from "$lib/components/QuickConnect.svelte";
  import { sessionStore } from "$lib/stores/sessions";
  import { uiStore } from "$lib/stores/ui";

  let showQuickConnect = $state(false);

  function handleKeydown(e: KeyboardEvent) {
    // Ctrl+K: Quick connect
    if (e.ctrlKey && e.key === "k") {
      e.preventDefault();
      showQuickConnect = !showQuickConnect;
    }
    // Ctrl+T: New tab
    if (e.ctrlKey && e.key === "t") {
      e.preventDefault();
      showQuickConnect = true;
    }
    // Ctrl+W: Close tab
    if (e.ctrlKey && e.key === "w") {
      e.preventDefault();
      sessionStore.closeActive();
    }
    // Ctrl+Tab: Next tab
    if (e.ctrlKey && e.key === "Tab") {
      e.preventDefault();
      if (e.shiftKey) {
        sessionStore.prevTab();
      } else {
        sessionStore.nextTab();
      }
    }
    // Ctrl+B: Toggle sidebar
    if (e.ctrlKey && e.key === "b") {
      e.preventDefault();
      uiStore.toggleSidebar();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="app-container">
  <TitleBar />

  <div class="app-body">
    {#if uiStore.sidebarVisible}
      <Sidebar />
    {/if}

    <div class="main-content">
      <TabBar />

      <div class="terminal-area">
        {#if sessionStore.activeSession}
          <Terminal session={sessionStore.activeSession} />
        {:else}
          <div class="welcome">
            <div class="welcome-content">
              <h1>Span</h1>
              <p class="subtitle">SSH Terminal for Network Engineers</p>
              <div class="shortcuts">
                <div class="shortcut"><kbd>Ctrl+K</kbd> Quick Connect</div>
                <div class="shortcut"><kbd>Ctrl+T</kbd> New Tab</div>
                <div class="shortcut"><kbd>Ctrl+B</kbd> Toggle Sidebar</div>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <StatusBar />
    </div>
  </div>

  {#if showQuickConnect}
    <QuickConnect onclose={() => (showQuickConnect = false)} />
  {/if}
</div>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
  }

  .app-body {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .main-content {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .terminal-area {
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  .welcome {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    opacity: 0.6;
  }

  .welcome-content {
    text-align: center;
  }

  .welcome h1 {
    font-size: 48px;
    font-weight: 300;
    letter-spacing: 8px;
    text-transform: uppercase;
    color: var(--accent);
    margin-bottom: 8px;
  }

  .subtitle {
    color: var(--text-muted);
    margin-bottom: 32px;
  }

  .shortcuts {
    display: flex;
    flex-direction: column;
    gap: 8px;
    align-items: center;
  }

  .shortcut {
    color: var(--text-muted);
    font-size: var(--font-size-sm);
  }

  kbd {
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 2px 6px;
    font-family: var(--font-mono);
    font-size: 11px;
    margin-right: 8px;
  }
</style>
