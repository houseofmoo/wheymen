export enum RequestTarget {
    InsertRoutine           = `api/routines/insert`,
    UpdateRoutine           = `api/routines/update`,
    GetAllRoutines          = `api/routines/get-all`,
    GetRoutine              = `api/routines/get`,
    DeleteRoutine           = `api/routines/delete`,

    InsertWorkout           = `api/workouts/insert`,
    UpdateWorkout           = `api/workouts/update`,
    GetAllWorkouts          = `api/workouts/get-all`,
    GetWorkout              = `api/workouts/get`,
    GetUnrelatedWorkouts    = `api/workouts/get-all/unrelated`,
    DeleteWorkout           = `api/workouts/delete`,

    StartSession            = 'api/session/start',
}

export function generateUrl(target: RequestTarget, id: string = null) {
    if (id) {
        return `/${target}/${id}`
    }
    return `/${target}`;
}