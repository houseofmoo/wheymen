import type { Category } from "./category";

export class Workout {
    id: string;
    user_id: string;
    name: string;
    category: Category;
    note: string;
}

export class WorkoutRow {
    id: string;
    user_id: string;
    name: string;
    category: Category;
    note: string;
}

export class InsertWorkoutRow {
    user_id: string;
    name: string;
    category: Category;
    note: string;
}

export class UpsertWorkoutRow<T> {
    workout_row: T;
    selected_routine_ids: string[];
    unselected_routine_ids: string[];
}

export class WorkoutHistoryRow {
    id: string;
    workout: string;
    user_id: string;
    completed_on: string;
    sets: Set[];
}

export class WorkoutHistory {
    id: string;
    workout: Workout;
    user_id: string;
    completed_on: string;
    sets: Set[];
}

export class Set {
    weight: number;
    reps: number;
}