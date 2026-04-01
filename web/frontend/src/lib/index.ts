const USE_WASM = import.meta.env.VITE_USE_WASM === '1' || import.meta.env.VITE_USE_WASM === 'true';

import { writable } from 'svelte/store';
import { CassidaKernel } from '$lib/cassida';
import type { CassidaResult } from './cassida/types/CassidaResult';

type AppState = {
    history: Array<CassidaResult>,
};

function createGlobalState() {
    const { subscribe, set, update } = writable<{
        data: AppState;
        connected: boolean;
    }>({
        data: { history: [] } as AppState,
        connected: false,
    });

    let socket: WebSocket | undefined;
    let kernel: CassidaKernel | undefined;

    function push_message(state: AppState, msg: CassidaResult) {
        let last = state.history.pop();

        if (last !== undefined && last.type != "err") {
            state.history.push(last);
        }

        state.history.push(msg);
    }

    async function connectWasm() {
        kernel = new CassidaKernel();
        update(s => ({ ...s, connected: true }));

        return {
            send: async (msg: string) => {
                const result = await kernel?.execute(msg);
                const parsed = typeof result === 'string' ? JSON.parse(result) : result;

                update(s => {
                    push_message(s.data, parsed);
                    return { ...s, connected: true };
                });
            }
        };
    }

    function connectWs() {
        const protocol = location.protocol === 'https:' ? 'wss' : 'ws';
        const url = `${protocol}://${location.host}/ws`;
        socket = new WebSocket(url);

        socket.onopen = () => update(s => ({ ...s, connected: true }));

        socket.onmessage = (event) => {
            try {
                const msg = JSON.parse(event.data);

                update(s => {
                    push_message(s.data, msg);
                    return { ...s, connected: true };
                });
            } catch (e) {
                console.log('Error parsing message:', e);
            }
        };

        socket.onclose = () => {
            update(s => ({ ...s, connected: false }));
            setTimeout(connectWs, 3000);
        };

        return {
            send: (msg: string) => socket?.send(msg),
        };
    }

    let transport: { send: (msg: string) => any } = { send: () => { } };

    if (typeof window !== 'undefined') {
        if (USE_WASM) {
            connectWasm().then(t => (transport = t));
        } else {
            transport = connectWs();
        }
    }

    return {
        subscribe,
        send: (msg: string) => transport.send(msg),
    };
}

export const appState = createGlobalState();

export function submitExpression(expr: string) {
    appState.send(expr);
}
