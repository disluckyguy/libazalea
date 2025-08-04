use std::{
    error::Error,
    fs::remove_file,
    path::Path,
    sync::{Arc, Mutex},
};

use bytes::{Buf, BytesMut};
use iced::{
    futures::{SinkExt, Stream, channel::mpsc},
    stream,
};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
    sync::{broadcast, mpsc::unbounded_channel},
};

use crate::{
    applet::{
        Applet, AppletMessage,
        runtime::{RuntimeMessage, RuntimeRequest},
    },
    runtime,
};

pub fn subscribe() -> impl Stream<Item = RuntimeMessage> {
    stream::channel(100, |mut tx: mpsc::Sender<RuntimeMessage>| async move {
        let (stream_sender, mut stream_receiver) = unbounded_channel();
        let mut output = tx.clone();

        runtime().spawn(async move {
            let socket = Path::new("/tmp/sock");

            if socket.exists() {
                remove_file(socket).unwrap();
            }
            let listener = UnixListener::bind(socket).unwrap();

            let (runtime_sender, mut runtime_receiver) = unbounded_channel();

            output
                .send(RuntimeMessage::Ready(runtime_sender.clone()))
                .await
                .unwrap();

            let applets = Arc::new(tokio::sync::Mutex::new(Vec::new()));

            {
                let applets = applets.clone();
                runtime().spawn(async move {
                    loop {
                        if let Some(request) = runtime_receiver.recv().await {
                            match request {
                                RuntimeRequest::Launch {
                                    info,
                                    order,
                                    position,
                                } => {
                                    let (applet_sender, applet_receiver) = broadcast::channel(100);
                                    let Ok(child) = info.launch() else {
                                        continue;
                                    };
                                    let applet = Applet {
                                        id: child.id(),
                                        order,
                                        info,
                                        sender: applet_sender,
                                        limits: None,
                                        intrinsic: None,
                                        size: None,
                                        margin: Default::default(),
                                        process: Arc::new(Mutex::new(child)),
                                        position,
                                    };
                                    output
                                        .send(RuntimeMessage::New(applet.clone()))
                                        .await
                                        .unwrap();
                                    applets.lock().await.push((applet, applet_receiver));
                                }
                            }
                        }
                    }
                });
            }
            loop {
                if let Ok((mut stream, _)) = listener.accept().await {
                    let pid = stream.peer_cred().unwrap().pid().unwrap().clone();

                    let Some((applet, mut receiver)) = applets
                        .lock()
                        .await
                        .iter()
                        .find(|(a, _)| a.id == pid as u32)
                        .map(|(a, r)| (a.clone(), r.resubscribe()))
                    else {
                        _ = stream.shutdown().await;
                        continue;
                    };

                    let (mut read, mut write) = stream.into_split();
                    let mut connection = BufConnection::new();
                    let stream_sender = stream_sender.clone();
                    runtime().spawn(async move {
                        loop {
                            if let Ok(Some(request)) = connection.read_frame(&mut read).await {
                                stream_sender
                                    .send(RuntimeMessage::Request(request, applet.id))
                                    .unwrap();
                            }
                        }
                    });
                    runtime().spawn(async move {
                        loop {
                            if let Ok(event) = receiver.recv().await {
                                write_frame(event, &mut write).await.unwrap();
                            }
                        }
                    });
                }
            }
        });
        runtime().spawn(async move {
            loop {
                if let Some(message) = stream_receiver.recv().await {
                    tx.send(message).await.unwrap()
                }
            }
        });
    })
}

pub fn applet_sub() -> impl Stream<Item = AppletMessage> {
    stream::channel(100, |mut tx: mpsc::Sender<AppletMessage>| async move {
        let (sender, mut receiver) = unbounded_channel();
        let (stream_sender, mut stream_receiver) = unbounded_channel();

        tx.send(AppletMessage::Ready(sender)).await.unwrap();
        runtime().spawn(async move {
            let socket = Path::new("/tmp/sock");
            if let Ok(stream) = UnixStream::connect(socket).await {
                let (mut read, mut write) = stream.into_split();
                let mut connection = BufConnection::new();
                runtime().spawn(async move {
                    loop {
                        if let Ok(Some(event)) = connection.read_frame(&mut read).await {
                            stream_sender.send(AppletMessage::Event(event)).unwrap();
                        };
                    }
                });
                runtime().spawn(async move {
                    loop {
                        if let Some(request) = receiver.recv().await {
                            write_frame(request, &mut write).await.unwrap();
                        }
                    }
                });
            }
        });

        runtime().spawn(async move {
            loop {
                if let Some(message) = stream_receiver.recv().await {
                    tx.send(message).await.unwrap();
                }
            }
        });
    })
}

pub struct BufConnection {
    buffer: BytesMut,
}

impl BufConnection {
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_frame<T, R>(&mut self, reader: &mut R) -> Result<Option<T>, Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
        R: AsyncReadExt + Unpin,
    {
        loop {
            if let Ok(frame) = self.parse_frame(reader).await {
                return Ok(frame);
            }

            if 0 == reader.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    async fn parse_frame<T, R>(&mut self, reader: &mut R) -> Result<Option<T>, Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
        R: AsyncReadExt + Unpin,
    {
        if self.buffer.has_remaining() {
            let size = (&self.buffer[0..4]).get_u32_le() as usize;
            if self.buffer.remaining() - 4 < size {
                return Err("Size doesn't match actual buffer size".into());
            }

            println!("{size}");
            let element: T = bincode::serde::decode_from_slice(
                &self.buffer[4..size + 4],
                bincode::config::standard(),
            )
            .unwrap()
            .0;
            self.buffer.advance(size + 4);
            return Ok(Some(element));
        }

        let mut size_buf = vec![0; 4];
        let _ = reader.read_exact(&mut size_buf).await?;

        let size = size_buf.as_slice().get_u32_le() as usize;

        if size >= self.buffer.remaining() {
            self.buffer.reserve(4096);
        }
        let num_bytes = reader.read_buf(&mut self.buffer).await?;
        if size > num_bytes {
            return Err("Buffer size smaller than frame size".into());
        }

        println!("{size} {num_bytes}");

        let element: T =
            bincode::serde::decode_from_slice(&self.buffer[0..size], bincode::config::standard())?
                .0;
        self.buffer.advance(size);

        Ok(Some(element))
    }
}

pub async fn write_frame<T, W>(message: T, writer: &mut W) -> Result<(), Box<dyn Error>>
where
    T: Serialize + DeserializeOwned,
    W: AsyncWriteExt + Unpin,
{
    let mut src = bincode::serde::encode_to_vec(message, bincode::config::standard())?;
    let len = (src.len() as u32).to_le_bytes();
    src.splice(0..0, len);
    writer.write_all(&mut src).await?;
    writer.flush().await?;
    Ok(())
}

pub struct Connection {
    stream: UnixStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: UnixStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_frame<T>(&mut self) -> Result<Option<T>, Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
    {
        loop {
            if let Ok(frame) = self.parse_frame().await {
                return Ok(frame);
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    async fn parse_frame<T>(&mut self) -> Result<Option<T>, Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
    {
        if !self.buffer.is_empty() {
            let size = (&self.buffer[0..4]).get_u32_le() as usize;
            if self.buffer[5..].len() < size {
                return Err("Size doesn't match actual buffer size".into());
            }

            let element: T = bincode::serde::decode_from_slice(
                &self.buffer[5..size],
                bincode::config::standard(),
            )?
            .0;
            self.buffer.advance(4 + size);
            return Ok(Some(element));
        }
        let mut size_buf = vec![0; 4];
        let _ = self.stream.read_exact(&mut size_buf).await?;

        let size = size_buf.as_slice().get_u32_le();

        if size as usize >= self.buffer.remaining() {
            self.buffer.reserve(size as usize + 4);
        }
        let num_bytes = self.stream.read_buf(&mut self.buffer).await?;

        if size != num_bytes as u32 {
            return Err("Size doesn't match actual buffer size".into());
        }

        let element: T =
            bincode::serde::decode_from_slice(&self.buffer, bincode::config::standard())?.0;
        self.buffer.advance(num_bytes);

        Ok(Some(element))
    }

    pub async fn write_frame<T>(&mut self, message: T) -> Result<(), Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut src = bincode::serde::encode_to_vec(message, bincode::config::standard())?;
        let len = (src.len() as u32).to_le_bytes();
        src.splice(0..0, len);
        self.stream.write_all(&mut src).await?;
        self.stream.flush().await?;
        Ok(())
    }
}
