use crate::models;
use crate::schema;
use log::error;
use log::{debug, info};
use uuid::Uuid;

use diesel::prelude::*;

// Add the following import if establish_connection is defined in your crate (commonly in a db module)
use super::models::RequestTabsSessions;
use super::util::establish_connection;
use super::util::ConfigState;
use super::util::FullTabdata;

#[tauri::command]
#[allow(dead_code)]
pub fn init_session(config: tauri::State<ConfigState>) -> Vec<FullTabdata> {
    use schema::requesttabs::dsl as requesttabs_dsl;
    use schema::requesttabs_sessions::dsl as requesttabs_sessions_dsl;
    let session = &config.0.config.lock().expect("Could not lock mutex");
    let conn = &mut establish_connection();

    let mut request_tabs: Vec<FullTabdata> = Vec::new();

    debug!("Init session: {:?}", session);
    let session_requesttabs: Vec<RequestTabsSessions> =
        requesttabs_sessions_dsl::requesttabs_sessions
            .filter(requesttabs_sessions_dsl::sessions_uuid.eq(session.last_session.clone()))
            .get_results(conn)
            .expect("Get request tab sessions");

    for session_requsttab in session_requesttabs {
        let requesttab_entry = requesttabs_dsl::requesttabs
            .filter(requesttabs_dsl::uuid.eq(&session_requsttab.requesttabs_uuid))
            .first::<models::RequestTabs>(conn);

        match requesttab_entry {
            Ok(tab_data) => {
                let request_tab = FullTabdata {
                    uuid: tab_data.uuid,
                    data: serde_json::from_str(tab_data.tabdata.as_str()).unwrap(),
                    saved_data: tab_data
                        .tabdata_saved
                        .and_then(|val| serde_json::from_str(val.as_str()).unwrap()),
                };
                request_tabs.push(request_tab)
            }
            Err(_) => {
                debug!("Tab data doesn't exist: {:?}", session_requsttab.uuid);
            }
        };
    }

    request_tabs
}

#[tauri::command]
#[allow(dead_code)]
pub fn save_session(session_data: String, config: tauri::State<ConfigState>) {
    use schema::requesttabs::dsl::*;
    use schema::requesttabs_sessions::dsl as requesttabs_sessions_dsl;
    use schema::sessions::dsl as sessions_dsl;
    let session = &config.0.config.lock().expect("Could not lock mutex");
    let conn = &mut establish_connection();

    debug!("Save session: {:?}, data: {}", session, session_data);
    let datas: Vec<FullTabdata> = match serde_json::from_str(session_data.as_str()) {
        Ok(val) => val,
        Err(err) => {
            error!("ERROR: {}", err);
            Vec::new()
        }
    };
    debug!("Parsed data: {:?}", datas);

    // Save session uuid if it doesn't exist yet
    let old_session_entry_query = sessions_dsl::sessions
        .filter(sessions_dsl::uuid.eq(session.last_session.clone()))
        .first::<models::Sessions>(conn);

    let old_session_entry = match old_session_entry_query {
        Ok(entry) => {
            info!("Session found: {:?}", entry);
            entry
        }
        Err(_) => {
            debug!("Add new session: {:?}", session.last_session);

            let new_session_entry = models::Sessions {
                uuid: session.last_session.clone(),
            };

            diesel::insert_into(sessions_dsl::sessions)
                .values(&new_session_entry)
                .execute(conn)
                .expect("Expect to add a new session entry");

            new_session_entry
        }
    };

    // Saving tab data to requesttabs
    for full_tab_data in &datas {
        debug!("{:?}", full_tab_data);

        // Find old tabdata entry if it exists
        let old_entry_query = requesttabs
            .filter(uuid.eq(&full_tab_data.uuid))
            .first::<models::RequestTabs>(conn);

        let old_entry = match old_entry_query {
            Ok(entry) => {
                debug!("Update data {:?}", full_tab_data.data.clone());

                let _update_requesttab =
                    diesel::update(requesttabs.filter(uuid.eq(&full_tab_data.uuid)))
                        .set((
                            tabdata.eq(serde_json::to_string(&full_tab_data.data.clone()).unwrap()),
                            tabdata_saved
                                .eq(serde_json::to_string(&full_tab_data.saved_data.clone())
                                    .unwrap()),
                        ))
                        .execute(conn);

                entry
            }
            Err(_) => {
                debug!("Add entry");

                let saved_data: Option<String> = match &full_tab_data.saved_data {
                    Some(val) => Some(serde_json::to_string(val).unwrap()),
                    None => None,
                };

                let new_entry = models::RequestTabs {
                    uuid: full_tab_data.uuid.clone(),
                    tabdata: serde_json::to_string(&full_tab_data.data.clone()).unwrap(),
                    tabdata_saved: saved_data,
                    saved_timestamp: None,
                };

                diesel::insert_into(requesttabs)
                    .values(&new_entry)
                    .execute(conn)
                    .expect("Expect entry to be added");

                info!("Added new entry: {:?}", new_entry);

                new_entry
            }
        };

        // Add to requesttabs_sessions
        let request_tabs_sessions = models::RequestTabsSessions {
            uuid: Uuid::new_v4().to_string(),
            requesttabs_uuid: old_entry.uuid,
            sessions_uuid: old_session_entry.uuid.clone(),
        };

        let old_requesttabs_essions_entry = requesttabs_sessions_dsl::requesttabs_sessions
            .filter(
                requesttabs_sessions_dsl::requesttabs_uuid
                    .eq(&request_tabs_sessions.requesttabs_uuid),
            )
            .filter(
                requesttabs_sessions_dsl::sessions_uuid.eq(&request_tabs_sessions.sessions_uuid),
            )
            .first::<models::RequestTabsSessions>(conn);

        match old_requesttabs_essions_entry {
            Ok(_) => {
                debug!("Requesttabs_sessions found");
            }
            Err(_) => {
                diesel::insert_into(requesttabs_sessions_dsl::requesttabs_sessions)
                    .values(&request_tabs_sessions)
                    .execute(conn)
                    .expect("Expect to add a new requesttabs_sessions entry");
            }
        };

        // Clean deleted tabs from session db
        let session_requesttabs: Vec<RequestTabsSessions> =
            requesttabs_sessions_dsl::requesttabs_sessions
                .filter(requesttabs_sessions_dsl::sessions_uuid.eq(session.last_session.clone()))
                .get_results(conn)
                .expect("Get request tab sessions");

        for requesttab_session in session_requesttabs {
            let mut tab_in_session = false;
            for full_tab_data in &datas {
                if full_tab_data.uuid == requesttab_session.requesttabs_uuid {
                    tab_in_session = true;
                }
            }

            if !tab_in_session {
                // Delete the requesttabs_session object
                debug!("Deleting tab {}", requesttab_session.uuid);
                let _ = diesel::delete(
                    requesttabs_sessions_dsl::requesttabs_sessions
                        .filter(requesttabs_sessions_dsl::uuid.eq(requesttab_session.uuid)),
                )
                .execute(conn);

                // Delete the requesttabs data if it's not in any session anymore
                let found_session = requesttabs_sessions_dsl::requesttabs_sessions
                    .filter(
                        requesttabs_sessions_dsl::requesttabs_uuid
                            .eq(requesttab_session.requesttabs_uuid.clone()),
                    )
                    .first::<models::RequestTabsSessions>(conn);

                match found_session {
                    Ok(_) => {
                        debug!("Session still found. Not deleting tab data.");
                    }
                    Err(_) => {
                        debug!("Deleting tab data");
                        let _ = diesel::delete(
                            requesttabs.filter(uuid.eq(requesttab_session.requesttabs_uuid)),
                        )
                        .execute(conn);
                    }
                };
            }
        }
    }
}
