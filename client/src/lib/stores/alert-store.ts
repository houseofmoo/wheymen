import { writable } from 'svelte/store';

function create() {
	let interval: NodeJS.Timeout;

	const { subscribe, set, update } = writable<string>(undefined);
	return {
		subscribe,
		set,
		update,
		setMsg: (newValue: string) => {
			set(newValue);
			interval && clearTimeout(interval);
			interval = setTimeout(() => {
                Alert.set(null);
            }, 1000 * 5);
		},
		setMsgWithLength: (newValue: string, length_in_sec: number) => {
			set(newValue);
			interval && clearTimeout(interval);
			interval = setTimeout(() => {
                Alert.set(null);
            }, 1000 * length_in_sec);
		},
	};
}

export const Alert = create();