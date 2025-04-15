mod message;
mod user_info;
mod message_serializer;
mod message_deserializer;

use std::io;
use std::io::prelude::*;
use bytes::BytesMut;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;
use crate::message::Message::UserInfoMessage;
use crate::message::MESSAGE_BYTES;
use crate::message_deserializer::deserialize;
use crate::message_serializer::serialize_message;
use crate::user_info::UserInfo;

fn read_line() -> io::Result<String>  {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn process_message(buffer: BytesMut) {
    println!("processing");
    match deserialize(buffer).unwrap() {
        UserInfoMessage(user) => {
            println!("{:?}", user)
        }
        _ => {}
    }
}

async fn listen_to_server(mut reader: ReadHalf<TcpStream>) {
    loop {
        let mut buffer = BytesMut::with_capacity(MESSAGE_BYTES);
        buffer.resize(MESSAGE_BYTES, 0);
        let bytes = reader.read(&mut buffer).await.unwrap();

        match bytes {
            0 => break,
            _ => process_message(buffer)
        }
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

    // let user_info = UserInfo::new("bab".to_string(), stream.local_addr()?);
    // let buffer = serialize_message(UserInfoMessage(user_info));
    // println!("{}", stream.local_addr()?);
    //
    // stream.write_all(&buffer)?;
    //
    // let mut res = [0; 128];
    // stream.read(&mut res)?;
    //
    // println!("{}", String::from_utf8(res.to_vec()).unwrap());
    Ok(())
}