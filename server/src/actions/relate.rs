use crate::model::db::Relationship;
use crate::model::{data::Routine, error::LocalError, shared_types::DbResult};
use crate::resource::client::DbClient;

pub async fn create_relationship(
    routine_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<Routine> {
    match does_relationship_exist(routine_id, workout_id, client).await {
        Ok(exists) => {
            if exists {
                return Ok(None);
            }
        }
        Err(e) => return Err(e),
    };

    let query = format!("RELATE {}->workout->{};", routine_id, workout_id);
    let result = client.send_query::<Relationship>(query).await?;
    if result.len() > 0 && result[0].result.len() > 0 {
        Ok(None)
    } else {
        Err(LocalError::RelationshipFailed)
    }
}

pub async fn create_many_relationships(
    routine_id: &String,
    workout_ids: &Vec<String>,
    client: &DbClient,
) -> DbResult<Routine> {
    for workout_id in workout_ids {
        match create_relationship(&routine_id, &workout_id, &client).await {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(None)
}

pub async fn delete_all_routine_relationships(
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Routine> {
    let query = format!("DELETE workout WHERE in=\"{}\";", routine_id);
    client.send_query::<Relationship>(query).await?;
    Ok(None)
}

async fn does_relationship_exist(
    routine_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> Result<bool, LocalError> {
    let query = format!(
        "SELECT * FROM workout WHERE in=\"{}\" AND out=\"{}\";",
        routine_id, workout_id
    );
    let result = client.send_query::<Relationship>(query).await?;
    return Ok(result.len() > 0 && result[0].result.len() > 0);
}
