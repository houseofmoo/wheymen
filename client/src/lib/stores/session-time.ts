import { readable, derived } from 'svelte/store';

let sessionStartTime = Date.now() - 10;
function startSessionTimer() {
	const { subscribe } = readable(Date.now(), function start(set) {
		const inverval = setInterval(() => {
			set(Date.now());
		}, 5000);

		return function stop() {
			clearInterval(inverval);
		};
	});

	return {
		subscribe,
		reset: () => {
			sessionStartTime = Date.now() - 10;
		}
	};
}

export const sessionLength = startSessionTimer();
export const sessionElapsed = derived(sessionLength, ($sessionLength) => {
	return $sessionLength - sessionStartTime;
});


let restStartTime = Date.now() - 10;
function startRestTimer() {
	const { subscribe } = readable(Date.now(), function start(set) {
		const inverval = setInterval(() => {
			set(Date.now());
		}, 5000);

		return function stop() {
			clearInterval(inverval);
		};
	});

	return {
		subscribe,
		reset: () => {
			restStartTime = Date.now() - 10;
		}
	};
}

export const restTime = startRestTimer();
export const restElapsed = derived(restTime, ($restTime) => {
	return $restTime - restStartTime;
});