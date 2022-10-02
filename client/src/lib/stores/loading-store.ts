import { writable } from 'svelte/store';

function create() {
    const { subscribe, set, update } = writable<boolean>(false);
    return {
        subscribe,
        set,
        update,
        start: () => { 
            set(true);
        },
        complete: () => {
            set(false);
        }
    }
}

export const Loading = create();