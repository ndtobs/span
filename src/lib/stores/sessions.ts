import type { Session } from "$lib/types";

/**
 * Session store - manages active terminal sessions (tabs)
 */
class SessionStore {
  sessions = $state<Session[]>([]);
  activeId = $state<string | null>(null);

  get activeSession(): Session | null {
    return this.sessions.find((s) => s.id === this.activeId) ?? null;
  }

  add(session: Session) {
    this.sessions.push(session);
    this.activeId = session.id;
  }

  remove(id: string) {
    const idx = this.sessions.findIndex((s) => s.id === id);
    if (idx === -1) return;

    this.sessions.splice(idx, 1);

    // If we closed the active tab, activate the nearest one
    if (this.activeId === id) {
      if (this.sessions.length === 0) {
        this.activeId = null;
      } else {
        const newIdx = Math.min(idx, this.sessions.length - 1);
        this.activeId = this.sessions[newIdx].id;
      }
    }
  }

  setActive(id: string) {
    this.activeId = id;
  }

  closeActive() {
    if (this.activeId) {
      this.remove(this.activeId);
    }
  }

  nextTab() {
    if (this.sessions.length < 2) return;
    const idx = this.sessions.findIndex((s) => s.id === this.activeId);
    const next = (idx + 1) % this.sessions.length;
    this.activeId = this.sessions[next].id;
  }

  prevTab() {
    if (this.sessions.length < 2) return;
    const idx = this.sessions.findIndex((s) => s.id === this.activeId);
    const prev = (idx - 1 + this.sessions.length) % this.sessions.length;
    this.activeId = this.sessions[prev].id;
  }

  updateStatus(id: string, status: Session["status"]) {
    const session = this.sessions.find((s) => s.id === id);
    if (session) {
      session.status = status;
    }
  }

  reorder(fromIdx: number, toIdx: number) {
    const [moved] = this.sessions.splice(fromIdx, 1);
    this.sessions.splice(toIdx, 0, moved);
  }
}

export const sessionStore = new SessionStore();
