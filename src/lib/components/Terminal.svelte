<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { sessionStore } from "$lib/stores/sessions";
  import type { Session } from "$lib/types";

  interface Props {
    session: Session;
  }

  let { session }: Props = $props();
  let terminalEl: HTMLDivElement;
  let term: any = null;
  let fitAddon: any = null;
  let dataListener: UnlistenFn | null = null;
  let statusListener: UnlistenFn | null = null;

  onMount(async () => {
    // Dynamic imports to avoid SSR issues
    const { Terminal } = await import("@xterm/xterm");
    const { FitAddon } = await import("@xterm/addon-fit");
    const { SearchAddon } = await import("@xterm/addon-search");
    const { WebLinksAddon } = await import("@xterm/addon-web-links");

    // Import xterm CSS
    await import("@xterm/xterm/css/xterm.css");

    term = new Terminal({
      fontFamily: "'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace",
      fontSize: 14,
      lineHeight: 1.2,
      cursorStyle: "block",
      cursorBlink: true,
      scrollback: 10000,
      theme: {
        background: "#1a1b26",
        foreground: "#c0caf5",
        cursor: "#c0caf5",
        selectionBackground: "#33467c",
        black: "#15161e",
        red: "#f7768e",
        green: "#9ece6a",
        yellow: "#e0af68",
        blue: "#7aa2f7",
        magenta: "#bb9af7",
        cyan: "#7dcfff",
        white: "#a9b1d6",
        brightBlack: "#414868",
        brightRed: "#f7768e",
        brightGreen: "#9ece6a",
        brightYellow: "#e0af68",
        brightBlue: "#7aa2f7",
        brightMagenta: "#bb9af7",
        brightCyan: "#7dcfff",
        brightWhite: "#c0caf5",
      },
    });

    fitAddon = new FitAddon();
    const searchAddon = new SearchAddon();
    const webLinksAddon = new WebLinksAddon();

    term.loadAddon(fitAddon);
    term.loadAddon(searchAddon);
    term.loadAddon(webLinksAddon);

    term.open(terminalEl);
    fitAddon.fit();

    // Handle user input → send to SSH backend
    term.onData(async (data: string) => {
      try {
        await invoke("write_data", { sessionId: session.id, data });
      } catch (error) {
        console.error("Failed to write data to session:", error);
      }
    });

    // Handle resize
    term.onResize(async ({ cols, rows }: { cols: number; rows: number }) => {
      try {
        await invoke("resize", { sessionId: session.id, cols, rows });
      } catch (error) {
        console.error("Failed to resize session:", error);
      }
    });

    // Listen for data from SSH backend
    dataListener = await listen(`session-data-${session.id}`, (event: any) => {
      if (term && event.payload) {
        term.write(event.payload);
      }
    });

    // Listen for session status updates
    statusListener = await listen(`session-status-${session.id}`, (event: any) => {
      if (event.payload) {
        sessionStore.updateStatus(session.id, event.payload);
        if (event.payload === "connected") {
          // Clear welcome message and indicate ready
          term.clear();
          term.writeln(`\x1b[1;32m✓\x1b[0m Connected to ${session.name}`);
          term.writeln("");
        } else if (event.payload === "error") {
          term.writeln(`\x1b[1;31m✗\x1b[0m Failed to connect to ${session.name}`);
          term.writeln("");
        }
      }
    });

    // Handle window resize
    const resizeObserver = new ResizeObserver(() => {
      fitAddon?.fit();
    });
    resizeObserver.observe(terminalEl);

    // Welcome message for now
    term.writeln("\x1b[1;34m⚡ Span\x1b[0m - SSH Terminal for Network Engineers");
    term.writeln(`\x1b[90mConnecting to ${session.name}...\x1b[0m`);
    term.writeln("");

    return () => {
      resizeObserver.disconnect();
      dataListener?.();
      statusListener?.();
    };
  });

  onDestroy(() => {
    dataListener?.();
    statusListener?.();
    term?.dispose();
  });

  // Re-fit when session changes (tab switch)
  $effect(() => {
    if (session && fitAddon) {
      requestAnimationFrame(() => fitAddon?.fit());
    }
  });
</script>

<div class="terminal-container" bind:this={terminalEl}></div>

<style>
  .terminal-container {
    width: 100%;
    height: 100%;
    padding: var(--space-xs);
  }

  .terminal-container :global(.xterm) {
    height: 100%;
  }

  .terminal-container :global(.xterm-viewport) {
    overflow-y: auto !important;
  }
</style>
