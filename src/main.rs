mod message;
mod user_info;
mod message_serializer;
mod message_deserializer;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::net::SocketAddr;
use bytes::BytesMut;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;
use crate::message::Message::{FileSendRequest, UserInfoMessage};
use crate::message::MESSAGE_BYTES;
use crate::message_deserializer::deserialize;
use crate::message_serializer::serialize_message;
use crate::user_info::UserInfo;

fn read_line() -> io::Result<String>  {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

async fn accept_file(sender_addr: SocketAddr) -> io::Result<()> {
    let stream = TcpStream::connect(sender_addr).await?;

    let mut buffer = BytesMut::with_capacity(MESSAGE_BYTES);
    buffer.resize(MESSAGE_BYTES, 0);

    // loop {
    //
    // }

    Ok(())
}

fn process_message(buffer: BytesMut, other_users: &mut HashMap<SocketAddr, UserInfo>) {
    println!("processing");
    match deserialize(buffer).unwrap() {
        UserInfoMessage(user) => {
            other_users.insert(user.socket_addr, user);
        }
        FileSendRequest(user) => {

        }
    }
}

async fn listen_to_server(mut reader: ReadHalf<TcpStream>) -> io::Result<()> {
    let mut other_users = HashMap::new();

    loop {
        let mut buffer = BytesMut::with_capacity(MESSAGE_BYTES);
        buffer.resize(MESSAGE_BYTES, 0);
        reader.read_exact(&mut buffer).await?;

        process_message(buffer, &mut other_users);
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6969").await?;
    let addr = stream.local_addr()?;
    let (reader, mut writer) = split(stream);

    let read_task = tokio::spawn(async move {
       listen_to_server(reader).await;
    });

    let user_info = UserInfoMessage(UserInfo::new("DK".to_string(), addr));
    let buffer = serialize_message(user_info);
    println!("{}", buffer.len());
    writer.write_all(&buffer).await?;

    read_task.await?;

    Ok(())
}