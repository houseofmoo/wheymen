import { readable, derived } from 'svelte/store';

let sessionStartTime = Date.now() - 10;
function startSessionTimer() {
	const { subscribe } = readable(Date.now(), function start(set) {
		const inverval = setInterval(() => {
			set(Date.now());
		}, 1000);

		return function stop() {
			clearInterval(inverval);
		};
	});

	return {
		subscribe,
		reset: () => {
			sessionStartTime = Date.now() - 10;
		},
		setTimeSinceStart: (ms_since_start) => {
			sessionStartTime = Date.now() - ms_since_start;
		}
	};
}

export const SessionLength = startSessionTimer();
export const SessionElapsed = derived(SessionLength, ($sessionLength) => {
	return $sessionLength - sessionStartTime;
});


let restStartTime = Date.now() - 10;
function startRestTimer() {
	const { subscribe } = readable(Date.now(), function start(set) {
		const inverval = setInterval(() => {
			set(Date.now());
		}, 1000);

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

export const RestTime = startRestTimer();
export const RestElapsed = derived(RestTime, ($restTime) => {
	return $restTime - restStartTime;
});