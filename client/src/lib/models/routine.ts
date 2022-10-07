import type { Workout } from "./workout";

export class Routine {
    id: string;
    user_id: string;
    name: string;
    days: string[];
    last_completed: Date;
    note: string;
    workouts: Workout[];
}

export class RoutineRow {
    id: string;
    user_id: string;
    name: string;
    days: string[];
    last_completed: Date;
    note: string;
    workouts: string[];
}

export class InsertRoutineRow {
    user_id: string;
    name: string;
    days: string[];
    last_completed: Date;
    note: string;
    workouts: string[];
}