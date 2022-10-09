export enum RequestTarget {
    GetAllRoutines          = `api/routines/get-all`,
    GetRoutine              = `api/routines/get`,
    InsertRoutine           = `api/routines/insert`,
    UpdateRoutine           = `api/routines/update`,
    DeleteRoutine           = `api/routines/delete`,

    GetWorkout              = `api/workouts/get`,
    GetAllWorkouts          = `api/workouts/get-all`,
    InsertWorkout           = `api/workouts/insert`,
    UpdateWorkout           = `api/workouts/update`,
    DeleteWorkout           = `api/workouts/delete`,
    GetUnrelatedWorkouts    = `api/workouts/get-all/unrelated`,

    GetAllSessions          = 'api/sessions/get-all',
    GetSession              = 'api/sessions/get',
    StartSession            = 'api/sessions/start',
    RestartSession          = 'api/sessions/restart',
    UpdateSession           = 'api/sessions/update',
    DeleteSession           = 'api/sessions/delete',
    DoesSessionExist        = 'api/sessions/exists'
}

export function generateUrl(target: RequestTarget, sub_domains: string[] = []) {
    let url = `/${target}`;
    for (let i = 0; i < sub_domains.length; i++) {
        url += `/${sub_domains[i]}`;
    }
    return url;
}