const SERVER_URL = `http://server:8000`;

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
}

// export function generateUrl(target: RequestTarget, id: string = null) {
//     if (id) {
//         return `${SERVER_URL}/${target}/${id}`
//     }
//     return `${SERVER_URL}/${target}`;
// }

// our request SHOULD be sent back on the same url as we're located
export function generateUrl(target: RequestTarget, id: string = null) {
    if (id) {
        return `/${target}/${id}`
    }
    return `/${target}`;
}