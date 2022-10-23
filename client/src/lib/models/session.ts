import type { Category } from "./category";

export class Session {
    id: string;
    user_id: string;
    routine_id: string;
    routine_name: string;
    routine_note: string;               
    duration_in_sec: number;
    workouts: SessionWorkout[];
}

export class SessionWorkout {
    workout_id: string;
    workout_name: string;
    workout_note: string;
    workout_category: Category;
    sets: SessionSet[];  
}

export class SessionSet {
    weight: number;
    reps: number;
    complete: boolean;
}