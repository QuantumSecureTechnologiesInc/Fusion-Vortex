// Tauri API integration for desktop app

declare global {
  interface Window {
    __TAURI__?: {
      invoke: (cmd: string, args?: any) => Promise<any>;
    };
  }
}

export const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;

export async function processIntent(intent: string): Promise<string> {
  if (isTauri && window.__TAURI__) {
    return await window.__TAURI__.invoke('process_intent', { intent });
  } else {
    // Fallback to HTTP API for web version
    const res = await fetch('http://localhost:3000/api/intent', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ intent }),
    });
    const data = await res.json();
    return data.session_id;
  }
}

export async function getBuildStatus(sessionId: string): Promise<any> {
  if (isTauri && window.__TAURI__) {
    return await window.__TAURI__.invoke('get_build_status', { sessionId });
  } else {
    const res = await fetch(`http://localhost:3000/api/status/${sessionId}`);
    return await res.json();
  }
}

export async function openOutputFolder(path: string): Promise<void> {
  if (isTauri && window.__TAURI__) {
    await window.__TAURI__.invoke('open_output_folder', { path });
  } else {
    console.log('Open folder:', path);
  }
}
