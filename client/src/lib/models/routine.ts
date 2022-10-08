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
    workouts: string[];
}

export class InsertRoutineRow {
    user_id: string;
    name: string;
    days: string[];
    last_completed: string;
    note: string;
    workouts: string[];
}

export class RoutineHistoryRow {
    id: string;
    routine: string;
    user_id: string;
    completed_on: string;
    duration_in_sec: number;
    workouts: RoutineHistoryWorkout[];
}

export class RoutineHistory {
    id: string;
    routine: Routine;
    user_id: string;
    completed_on: Date;
    duration_in_sec: number;
    workouts: RoutineHistoryWorkout[];
}

export class RoutineHistoryWorkout {
    name: string;
    weight: number;
    reps: number;
}