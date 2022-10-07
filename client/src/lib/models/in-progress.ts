export class InProgress {
    user_id: string;
    routine_id: string;
    name: string;
    note: string;               
    start_time: Date;           // date/time this routine was started
    rduration_in_sec: number;    // seconds since start time
    workouts: WorkoutInProgress[];      // the workouts this session contains
}

export class WorkoutInProgress {
    id: string;
    name: string;
    note: string;
    complete: boolean;
    sets: WorkoutSets[];            // the sets being performed now   
    previousSets: WorkoutSets[];    // the list of sets completed last time this workout was performed
}

export class WorkoutSets {
    weight: number;
    reps: number;
    complete: boolean;
}