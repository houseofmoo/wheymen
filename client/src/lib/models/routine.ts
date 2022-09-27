import type { Workout } from "./workout";

export class Routine {
    id: string;
    user_id: string;
    name: string;
    days: string[];
    last_completed: string;
    note: string;
    workouts: Workout[];
}

export class RoutineRow {
    id: string;
    user_id: string;
    name: string;
    days: string[];
    last_completed: string;
    note: string;
}

export class RoutineHistory {
    id: string;
    user_id: string;
    routine_id: string;
    workouts: string[];
    date: string;
    duration_in_sec: number;
}