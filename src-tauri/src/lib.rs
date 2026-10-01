mod zsftp;

use serde::{Deserialize,Serialize};
use serde_json::Value;
use std::sync::{Mutex,MutexGuard,PoisonError};
use tauri::State;
use zsftp::Job;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)] #[serde(rename_all="snake_case")] pub enum Mode{Passive,Active}
#[derive(Deserialize)] #[serde(rename_all="camelCase")] pub struct TransferConfig{pub archive:String,pub rar_password:Option<String>,pub host:String,pub port:u32,pub mode:Mode,pub user:String,pub password:String,pub directory:String,pub mkdir:bool,pub verbose:bool,pub buffer_mib:u32}
#[derive(Default)] struct AppState{job:Mutex<Option<Job>>}
impl AppState{fn lock(&self)->MutexGuard<'_,Option<Job>>{self.job.lock().unwrap_or_else(PoisonError::into_inner)} fn start(&self,c:&TransferConfig)->Result<(),String>{let old={let mut slot=self.lock();if let Some(j)=slot.as_ref(){if j.poll(u64::MAX)?.get("phase").and_then(Value::as_str)!=Some("finished"){return Err("A transfer is already in progress".into())}}slot.replace(Job::start(c)?)};drop(old);Ok(())} fn poll(&self,c:u64)->Result<Value,String>{self.lock().as_ref().map(|j|j.poll(c)).unwrap_or_else(||Err("no transfer".into()))} fn cancel(&self){if let Some(j)=self.lock().as_ref(){j.cancel()}} fn take(&self)->Option<Job>{self.lock().take()}}
#[derive(Serialize)] #[serde(rename_all="camelCase")] struct EngineStatus{available:bool,version:Option<String>,detail:String}
#[tauri::command] fn zsftp_engine_status()->EngineStatus{let v=zsftp::version();EngineStatus{available:true,version:Some(v),detail:"ZSFTP core integrated with ZeroSpace.".into()}}
#[tauri::command] async fn start_zsftp_transfer(state:State<'_,AppState>,request:TransferConfig)->Result<String,String>{state.start(&request)?;Ok("Transfer started with the integrated ZSFTP engine.".into())}
#[tauri::command] async fn poll_zsftp_transfer(state:State<'_,AppState>,cursor:u64)->Result<Value,String>{state.poll(cursor)}
#[tauri::command] async fn cancel_zsftp_transfer(state:State<'_,AppState>)->Result<(),String>{state.cancel();Ok(())}
#[tauri::command] async fn close_zsftp_transfer(state:State<'_,AppState>)->Result<(),String>{if let Some(j)=state.take(){tauri::async_runtime::spawn_blocking(move||drop(j)).await.map_err(|e|e.to_string())?}Ok(())}
pub fn run(){tauri::Builder::default().manage(AppState::default()).invoke_handler(tauri::generate_handler![zsftp_engine_status,start_zsftp_transfer,poll_zsftp_transfer,cancel_zsftp_transfer,close_zsftp_transfer]).run(tauri::generate_context!()).expect("error while running ZeroSpace");}