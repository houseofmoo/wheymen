use crate::model::db::DbResponse;

pub fn get_first_result<T>(response: Vec<DbResponse<T>>) -> Option<T> {
    for results in response {
        for row in results.result {
            return Some(row);
        }
    }
    None
}

pub fn get_all_results<T>(response: Vec<DbResponse<T>>) -> Option<Vec<T>> {
    let mut rows = vec![];
    for results in response {
        for row in results.result {
            rows.push(row);
        }
    }

    if rows.len() > 0 {
        Some(rows)
    } else {
        None
    }
}
