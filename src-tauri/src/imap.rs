use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use async_imap::Session;
use async_native_tls::TlsStream;
use tauri::Manager;
use tokio::net::TcpStream;

use crate::{database::AccountTable, logging_handler, LoggerType, Shareble};

pub async fn imap<R: tauri::Runtime>(app_state: Arc<Mutex<Shareble>>, handle: &impl Manager<R>) {
    // sleeep
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    match initiliaze_imap(&app_state, handle).await {
        Ok(mut session) => {
            loop {
                if app_state.lock().unwrap().logout {
                    break;
                }
            }
            logging_handler("Logging out from imap!", LoggerType::Info, handle);
            println!("Logging out for real!");
            if let Err(e) = session.logout().await {
                logging_handler(e, LoggerType::Error, handle);
            }
        }
        Err(e) => {
            logging_handler(e, LoggerType::Error, handle);
        }
    }
}

async fn initiliaze_imap<R: tauri::Runtime>(
    app_state: &Arc<Mutex<Shareble>>,
    handle: &impl Manager<R>,
) -> Result<Session<TlsStream<TcpStream>>, Box<dyn Error + Send + Sync>> {
    let account = app_state
        .lock()
        .unwrap()
        .sql
        .as_ref()
        .unwrap()
        .get_account(1)?;
    let imap_addr = (account.imap_server.as_str(), account.imap_port as u16);
    let tcp_stream = TcpStream::connect(imap_addr).await?;
    let tls = async_native_tls::TlsConnector::new();
    let tls_stream = tls
        .connect(account.imap_server.as_str(), tcp_stream)
        .await?;

    let client = async_imap::Client::new(tls_stream);
    logging_handler(
        format!("-- connected to {}:{}", imap_addr.0, imap_addr.1),
        LoggerType::Info,
        handle,
    );

    let mut imap_session = client
        .login(account.username.clone(), account.password)
        .await
        .map_err(|e| e.0)?;
    logging_handler(
        format!("-- logged in a {}", account.username),
        LoggerType::Info,
        handle,
    );
    Ok(imap_session)
}
