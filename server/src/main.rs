#![allow(unused)]


mod websocket;
// For websocket
use std::{env, io::Error};
use futures_util::{future, StreamExt, TryStreamExt, stream::SplitSink, SinkExt};
use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{WebSocketStream, tungstenite::Result};
use tokio_tungstenite::tungstenite::Message;

mod api;

