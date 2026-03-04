import type { InventoryFolder, InventoryDevice } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

/**
 * Inventory store - device tree with folders
 */
class InventoryStore {
  folders = $state<InventoryFolder[]>([]);
  searchQuery = $state("");
  loading = $state(false);

  get flatDevices(): InventoryDevice[] {
    const devices: InventoryDevice[] = [];
    const walk = (folders: InventoryFolder[]) => {
      for (const f of folders) {
        devices.push(...f.devices);
        walk(f.children);
      }
    };
    walk(this.folders);
    return devices;
  }

  get filteredDevices(): InventoryDevice[] {
    if (!this.searchQuery) return this.flatDevices;
    const q = this.searchQuery.toLowerCase();
    return this.flatDevices.filter(
      (d) =>
        d.name.toLowerCase().includes(q) ||
        d.connectionConfig.host.toLowerCase().includes(q) ||
        d.tags?.some((t) => t.toLowerCase().includes(q)),
    );
  }

  toggleFolder(folderId: string) {
    const find = (folders: InventoryFolder[]): InventoryFolder | null => {
      for (const f of folders) {
        if (f.id === folderId) return f;
        const found = find(f.children);
        if (found) return found;
      }
      return null;
    };
    const folder = find(this.folders);
    if (folder) {
      folder.expanded = !folder.expanded;
    }
  }

  async load() {
    this.loading = true;
    try {
      const response = await invoke<{
        folders: Array<{
          id: string;
          name: string;
          parent_id: string | null;
          sort_order: number;
        }>;
        devices: Array<{
          id: string;
          name: string;
          folder_id: string | null;
          host: string;
          port: number;
          username: string;
          auth_method: string;
          key_path?: string;
          platform?: string;
          tags: string[];
          jump_hosts: Array<{
            host: string;
            port: number;
            username: string;
            auth_method: string;
            key_path?: string;
          }>;
          post_connect_commands: string[];
          notes?: string;
          last_connected?: number;
          created_at: number;
          updated_at: number;
        }>;
      }>("list_devices");

      // Build folder hierarchy
      const folderMap = new Map<string, InventoryFolder>();
      const rootFolders: InventoryFolder[] = [];

      // Create folder objects
      for (const f of response.folders) {
        const folder: InventoryFolder = {
          id: f.id,
          name: f.name,
          parentId: f.parent_id,
          children: [],
          devices: [],
        };
        folderMap.set(f.id, folder);
      }

      // Build parent-child relationships
      for (const folder of folderMap.values()) {
        if (folder.parentId) {
          const parent = folderMap.get(folder.parentId);
          if (parent) {
            parent.children.push(folder);
          }
        } else {
          rootFolders.push(folder);
        }
      }

      // Assign devices to folders
      for (const d of response.devices) {
        const device: InventoryDevice = {
          id: d.id,
          name: d.name,
          folderId: d.folder_id || "",
          connectionConfig: {
            id: d.id,
            name: d.name,
            host: d.host,
            port: d.port,
            username: d.username,
            authMethod: d.auth_method as "password" | "key" | "agent",
            keyPath: d.key_path,
            jumpHosts: d.jump_hosts,
            postConnectCommands: d.post_connect_commands,
          },
          platform: d.platform,
          tags: d.tags,
          lastConnected: d.last_connected,
          notes: d.notes,
        };

        if (d.folder_id) {
          const folder = folderMap.get(d.folder_id);
          if (folder) {
            folder.devices.push(device);
          }
        }
      }

      this.folders = rootFolders;
    } catch (err) {
      console.error("Failed to load inventory:", err);
    } finally {
      this.loading = false;
    }
  }
}

export const inventoryStore = new InventoryStore();
