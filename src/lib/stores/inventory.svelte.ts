import type { InventoryFolder, InventoryDevice } from "$lib/types";

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
      // TODO: invoke Tauri command to load from SQLite
      // const data = await invoke("inventory_list");
      // this.folders = data;
    } finally {
      this.loading = false;
    }
  }
}

export const inventoryStore = new InventoryStore();
