use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use async_imap::Session;
use async_native_tls::TlsStream;
use futures::TryStreamExt;
use mail_parser::MessageParser;
use tauri::Manager;
use tokio::net::TcpStream;

use crate::{
    database::{AccountTable, Email, EmailTable},
    Loggable, LoggerType, Shareble,
};

pub async fn imap(mut app_state: Arc<Mutex<Shareble>>) {
    // sleeep
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    match initiliaze_imap(&mut app_state).await {
        Ok(mut session) => {
            loop {
                if app_state.lock().unwrap().logout {
                    break;
                }
            }
            app_state
                .lock()
                .unwrap()
                .push_log("Logging out from imap!", LoggerType::Info);
            println!("Logging out for real!");
            if let Err(e) = session.logout().await {
                app_state.lock().unwrap().push_log(e, LoggerType::Error);
            }
        }
        Err(e) => {
            app_state.lock().unwrap().push_log(e, LoggerType::Error);
        }
    }
}

async fn initiliaze_imap(
    app_state: &mut Arc<Mutex<Shareble>>,
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
    app_state.log(
        format!("-- connected to {}:{}", imap_addr.0, imap_addr.1),
        LoggerType::Info,
    );

    let mut imap_session = client
        .login(account.username.clone(), account.password)
        .await
        .map_err(|e| e.0)?;
    app_state.log(
        format!("-- logged in a {}", account.username),
        LoggerType::Info,
    );

    let mailbox = imap_session.select("INBOX").await?;
    app_state.log(
        format!("-- select a mailbox: {:?}", mailbox),
        LoggerType::Info,
    );

    let last_email = app_state
        .lock()
        .unwrap()
        .sql
        .as_ref()
        .unwrap()
        .get_email_count(1);
    match last_email {
        Ok(email) => {
            let messages_stream = imap_session
                .fetch(format!("{}:*", (email + 1)), "RFC822")
                .await?;
            let messages: Vec<_> = messages_stream.try_collect().await?;
            app_state.log(
                format!("Fetched {} new emails!", messages.len()),
                LoggerType::Info,
            );
            for (i, raw_message) in messages.iter().enumerate() {
                let flags: Vec<_> = raw_message.flags().collect();

                if let Some(body) = raw_message.body() {
                    let message = MessageParser::default()
                        .parse(std::str::from_utf8(body)?)
                        .unwrap();
                    let mut new_email = Email::from(message);
                    new_email.set_flags(flags);
                    new_email.account_id = account.id;
                    new_email.push(app_state.lock().unwrap().sql.as_ref().unwrap())?;
                }
            }
        }
        Err(e) => match e {
            rusqlite::Error::QueryReturnedNoRows => {
                let messages_stream = imap_session.fetch("1:*", "RFC822").await?;
                let messages: Vec<_> = messages_stream.try_collect().await?;
                for raw_message in messages {
                    let flags: Vec<_> = raw_message.flags().collect();
                    if let Some(body) = raw_message.body() {
                        let message = MessageParser::default()
                            .parse(std::str::from_utf8(body)?)
                            .unwrap();
                        let mut new_email = Email::from(message);
                        new_email.set_flags(flags);
                        new_email.account_id = account.id;
                        new_email.push(app_state.lock().unwrap().sql.as_ref().unwrap())?;
                    }
                }
                app_state.log("No emails found in the database!", LoggerType::Info);
            }
            _ => {
                app_state.log(e.to_string(), LoggerType::Error);
            }
        },
    }

    println!("Returning imap_session!");
    Ok(imap_session)
}
