<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import type { Session } from "$lib/types";

  interface Props {
    session: Session;
  }

  let { session }: Props = $props();
  let terminalEl: HTMLDivElement;
  let term: any = null;
  let fitAddon: any = null;

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
    term.onData((data: string) => {
      // TODO: invoke Tauri command to send data to SSH session
      // invoke("session_write", { sessionId: session.id, data });
    });

    // Handle resize
    term.onResize(({ cols, rows }: { cols: number; rows: number }) => {
      // TODO: invoke Tauri command to resize PTY
      // invoke("session_resize", { sessionId: session.id, cols, rows });
    });

    // Listen for data from SSH backend
    // TODO: listen for Tauri events
    // listen(`session-data-${session.id}`, (event) => {
    //   term.write(event.payload as string);
    // });

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
    };
  });

  onDestroy(() => {
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
