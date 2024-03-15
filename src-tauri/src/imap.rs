use std::{
    error::Error,
    str::from_utf8,
    sync::{Arc, Mutex},
};

use async_imap::Session;
use async_native_tls::TlsStream;
use futures::TryStreamExt;
use mail_parser::MessageParser;
use tokio::net::TcpStream;

use crate::{
    app_state::EventDispatcher,
    database::{Account, Email, EmailTable},
    Shareble,
};

pub async fn imap(mut app_state: Arc<Mutex<Shareble>>, account: Account) {
    // sleeep
    match initiliaze_imap(&mut app_state, account.clone()).await {
        Ok(session) => {
            let mut idle = session.idle();
            idle.init().await.unwrap();
            loop {
                let (idle_wait, interrupt) = idle.wait();

                let controll_app_state = app_state.clone();
                let idle_stop = Arc::new(Mutex::new(false));
                let cpy_idle_stop = idle_stop.clone();

                let account_id = account.id;
                tokio::spawn(async move {
                    loop {
                        // if all threads are stopped, stop the idle
                        if controll_app_state.lock().unwrap().logout
                            || *cpy_idle_stop.lock().unwrap()
                        {
                            drop(interrupt);
                            break;
                        }

                        // if this thread is stopped, stop the idle
                        if let Some(thread) = controll_app_state
                            .lock()
                            .unwrap()
                            .imap_threads
                            .iter()
                            .find(|thread| thread.account_id == account_id)
                        {
                            if thread.stop {
                                drop(interrupt);
                                break;
                            }
                        }
                    }
                });

                let idle_result = idle_wait.await;
                if let Ok(result) = idle_result {
                    match result {
                        async_imap::extensions::idle::IdleResponse::ManualInterrupt => {
                            break;
                        }
                        async_imap::extensions::idle::IdleResponse::Timeout => {}
                        async_imap::extensions::idle::IdleResponse::NewData(data) => {
                            let bytes = data.borrow_owner().to_vec();
                            let msg = from_utf8(bytes.as_slice()).unwrap_or("");
                            let splitted_msg: Vec<&str> = msg.split(' ').collect();
                            let email_id = splitted_msg
                                .get(1)
                                .unwrap_or(&"")
                                .parse::<u32>()
                                .unwrap_or(0);
                            println!("New data: {} {}", msg, email_id);
                            if splitted_msg.get(2).unwrap().contains("EXISTS") {
                                let mut session = idle.done().await.unwrap();
                                let mut notification_body = String::new();
                                fetch_emails(
                                    &mut app_state,
                                    &mut session,
                                    account.clone(),
                                    email_id,
                                    -1,
                                )
                                .await
                                .iter()
                                .for_each(|email| {
                                    notification_body.push_str(&email.subject);
                                    notification_body.push('\n');
                                    email
                                        .push(app_state.lock().unwrap().sql.as_ref().unwrap())
                                        .unwrap();
                                });
                                app_state.notify("New email(s)", &notification_body);
                                app_state.action("fetch_emails", "");
                                idle = session.idle();
                                idle.init().await.unwrap();
                            }
                        }
                    }
                    *idle_stop.lock().unwrap() = true;
                } else {
                    app_state.log_error(idle_result.err().unwrap());
                }
            }

            let session = idle.done().await; // TODO: is error sometimes?
            app_state.log_info("Logging out from imap!");
            println!("Logging out from imap!");
            match session {
                Ok(mut session) => {
                    if let Err(e) = session.logout().await {
                        app_state.log_error(e);
                    }
                }
                Err(e) => {
                    app_state.log_error(e);
                }
            }
        }
        Err(e) => {
            app_state.log_error(e);
        }
    }
}

async fn initiliaze_imap(
    app_state: &mut Arc<Mutex<Shareble>>,
    account: Account,
) -> Result<Session<TlsStream<TcpStream>>, Box<dyn Error + Send + Sync>> {
    let imap_addr = (account.imap_host.as_str(), account.imap_port as u16);
    let tcp_stream = TcpStream::connect(imap_addr).await?;
    let tls = async_native_tls::TlsConnector::new();
    let tls_stream = tls.connect(account.imap_host.as_str(), tcp_stream).await?;

    let client = async_imap::Client::new(tls_stream);
    app_state.log_info(format!("-- connected to {}:{}", imap_addr.0, imap_addr.1));

    let mut imap_session = client
        .login(account.username.clone(), account.password.clone())
        .await
        .map_err(|e| e.0)?;
    app_state.log_info(format!("-- logged in a {}", account.username));

    let mailbox = imap_session.select("INBOX").await?;
    app_state.log_info(format!("-- select a mailbox: {:?}", mailbox));

    let last_email = app_state
        .lock()
        .unwrap()
        .sql
        .as_ref()
        .unwrap()
        .get_email_count(account.id);
    match last_email {
        Ok(email) => {
            let last_emails = (email as i32 - 100).max(1) as u32;
            fetch_emails(
                app_state,
                &mut imap_session,
                account.clone(),
                last_emails,
                -1,
            )
            .await
            .iter()
            .for_each(|email| {
                email
                    .push(app_state.lock().unwrap().sql.as_ref().unwrap())
                    .unwrap();
            });
            app_state.action("fetch_emails", "");
            fetch_emails(
                app_state,
                &mut imap_session,
                account,
                1,
                last_emails as i32 - 1,
            )
            .await
            .iter()
            .for_each(|email| {
                email
                    .push(app_state.lock().unwrap().sql.as_ref().unwrap())
                    .unwrap()
            });
        }
        Err(e) => match e {
            rusqlite::Error::QueryReturnedNoRows => {
                fetch_emails(app_state, &mut imap_session, account, 1, -1)
                    .await
                    .iter()
                    .for_each(|email| {
                        email
                            .push(app_state.lock().unwrap().sql.as_ref().unwrap())
                            .unwrap();
                    });
                app_state.log_info("No emails found in the database!");
            }
            _ => {
                app_state.log_info(e.to_string());
            }
        },
    }
    app_state.action("fetch_emails", "");

    Ok(imap_session)
}

async fn fetch_emails(
    app_state: &mut Arc<Mutex<Shareble>>,
    session: &mut Session<TlsStream<TcpStream>>,
    account: Account,
    from: u32,
    to: i32,
) -> Vec<Email> {
    let mut emails = vec![];
    let messages_stream = session
        .fetch(
            format!(
                "{}:{}",
                from,
                if to < 0 {
                    "*".to_string()
                } else {
                    to.to_string()
                }
            ),
            "(BODY.PEEK[] FLAGS)",
        )
        .await
        .unwrap();
    let messages: Vec<_> = messages_stream.try_collect().await.unwrap();
    app_state.log_info(format!("Fetched {} new emails!", messages.len()));
    for raw_message in messages {
        let flags: Vec<_> = raw_message.flags().collect();
        let uid = raw_message.uid;
        if let Some(body) = raw_message.body() {
            let message = MessageParser::default()
                .parse(std::str::from_utf8(body).unwrap())
                .unwrap();
            let mut new_email = Email::from(message);
            new_email.set_flags(flags);
            new_email.account_id = account.id;
            if let Some(uid) = uid {
                new_email.id = uid as i64;
            }
            emails.push(new_email);
            /* new_email
            .push(app_state.lock().unwrap().sql.as_ref().unwrap())
            .unwrap(); */
        }
    }
    emails
}
