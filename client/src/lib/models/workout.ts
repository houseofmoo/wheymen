import type { Category } from "./category";

export class Workout {
    id: string;
    user_id: string;
    name: string;
    category: Category;
    note: string;
}

export class WorkoutHistory {
    id: string;
    user_id: string;
    workout_id: string;
    routine_id: string;
    name: string;
    category: Category;
    date: string;
    lift: Lift[];
    cardio: Cardio[];
}

export class Lift {
    weight: number;
    reps: number;
}

export class Cardio {
    duration: number;
    distance: number;
}