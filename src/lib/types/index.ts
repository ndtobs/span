/** SSH connection configuration */
export interface ConnectionConfig {
  id: string;
  name: string;
  host: string;
  port: number;
  username: string;
  authMethod: "password" | "key" | "agent";
  keyPath?: string;
  jumpHosts?: JumpHost[];
  /** Auto-execute commands on connect */
  postConnectCommands?: string[];
  /** Color label for visual grouping */
  colorTag?: string;
}

/** Jump host in a proxy chain */
export interface JumpHost {
  host: string;
  port: number;
  username: string;
  authMethod: "password" | "key" | "agent";
  keyPath?: string;
}

/** A live terminal session */
export interface Session {
  id: string;
  connectionId: string;
  name: string;
  status: SessionStatus;
  startedAt: number;
  /** xterm.js instance reference (frontend only) */
  terminalId?: string;
}

export type SessionStatus =
  | "connecting"
  | "connected"
  | "disconnected"
  | "error";

/** Inventory folder / device tree */
export interface InventoryFolder {
  id: string;
  name: string;
  parentId: string | null;
  children: InventoryFolder[];
  devices: InventoryDevice[];
  expanded?: boolean;
}

/** Device in inventory */
export interface InventoryDevice {
  id: string;
  name: string;
  folderId: string;
  connectionConfig: ConnectionConfig;
  /** Platform/vendor for icon display */
  platform?: string;
  /** Tags for filtering */
  tags?: string[];
  /** Last successful connection time */
  lastConnected?: number;
  notes?: string;
}

/** Credential entry */
export interface Credential {
  id: string;
  label: string;
  username: string;
  /** Stored in OS keyring, never in plaintext */
  passwordRef?: string;
  keyPath?: string;
}

/** Session log entry */
export interface LogConfig {
  enabled: boolean;
  directory: string;
  /** Filename template, e.g. "{name}_{date}.log" */
  filenameTemplate: string;
  /** Include timestamps in log */
  timestamps: boolean;
}

/** App settings */
export interface Settings {
  terminal: {
    fontFamily: string;
    fontSize: number;
    lineHeight: number;
    cursorStyle: "block" | "underline" | "bar";
    cursorBlink: boolean;
    scrollbackLines: number;
    colorScheme: string;
  };
  logging: LogConfig;
  ui: {
    sidebarVisible: boolean;
    sidebarWidth: number;
    showStatusBar: boolean;
  };
  ssh: {
    keepAliveInterval: number;
    keepAliveCountMax: number;
    defaultPort: number;
    defaultUsername: string;
  };
}
