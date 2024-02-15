use futures::TryStreamExt;
use tokio::net::TcpStream;

const USERNAME: &str = "or376135@rwth-aachen.de";
const PASSWORD: &str = "wza.AKY4avj@kxp4hca";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = fetch_inbox_top("mail.rwth-aachen.de", USERNAME, PASSWORD).await?;
    print!("-- top of inbox:\n{}", res.unwrap());
    Ok(())
}

async fn fetch_inbox_top(
    imap_server: &str,
    login: &str,
    password: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let imap_addr = (imap_server, 993);
    let tcp_stream = TcpStream::connect(imap_addr).await?;
    let tls = async_native_tls::TlsConnector::new();
    let tls_stream = tls.connect(imap_server, tcp_stream).await?;

    let client = async_imap::Client::new(tls_stream);
    println!("-- connected to {}:{}", imap_addr.0, imap_addr.1);

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client.login(login, password).await.map_err(|e| e.0)?;
    println!("-- logged in a {}", login);

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX").await?;
    println!("-- INBOX selected");

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages_stream = imap_session.fetch("100", "RFC822").await?;
    let messages: Vec<_> = messages_stream.try_collect().await?;
    let message = if let Some(m) = messages.first() {
        m
    } else {
        return Ok(None);
    };

    // extract the message's body
    /* print!(
        "-- message: {}",
        std::str::from_utf8(message.header().unwrap()).unwrap()
    ); */
    let body = message.body();
    let body = match body {
        Some(body) => {
            let body = std::str::from_utf8(body)
                .expect("message was not valid utf-8")
                .to_string();
            println!("-- 1 message received, logging out");
            body
        }
        None => {
            println!("-- message did not have a body");
            String::new()
        }
    };

    // be nice to the server and log out
    imap_session.logout().await?;

    Ok(Some(body))
}
