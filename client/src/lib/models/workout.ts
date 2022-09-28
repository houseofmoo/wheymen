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