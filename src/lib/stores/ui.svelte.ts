/**
 * UI state store
 */
class UIStore {
  sidebarVisible = $state(true);
  sidebarWidth = $state(260);

  toggleSidebar() {
    this.sidebarVisible = !this.sidebarVisible;
  }
}

export const uiStore = new UIStore();
