mod zsftp;

use serde::{Deserialize,Serialize};
use serde_json::Value;
use std::{fs,io::{Read,Write},net::TcpStream,path::PathBuf,sync::{Mutex,MutexGuard,PoisonError},time::Duration};
use tauri::State;
use suppaftp::{list::File as FtpListFile,types::FileType,FtpError,FtpStream};
use std::convert::TryFrom;
use zsftp::Job;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)] #[serde(rename_all="snake_case")] pub enum Mode{Passive,Active}
#[derive(Deserialize)] #[serde(rename_all="camelCase")] pub struct TransferConfig{pub archive:String,pub rar_password:Option<String>,pub host:String,pub port:u32,pub mode:Mode,pub user:String,pub password:String,pub directory:String,pub mkdir:bool,pub verbose:bool,pub buffer_mib:u32}
#[derive(Default)] struct AppState{job:Mutex<Option<Job>>}
impl AppState{fn lock(&self)->MutexGuard<'_,Option<Job>>{self.job.lock().unwrap_or_else(PoisonError::into_inner)} fn start(&self,c:&TransferConfig)->Result<(),String>{let old={let mut slot=self.lock();if let Some(j)=slot.as_ref(){if j.poll(0)?.get("phase").and_then(Value::as_str)!=Some("finished"){return Err("A transfer is already in progress".into())}}slot.replace(Job::start(c)?)};drop(old);Ok(())} fn poll(&self,c:u64)->Result<Value,String>{self.lock().as_ref().map(|j|j.poll(c)).unwrap_or_else(||Err("no transfer".into()))} fn cancel(&self){if let Some(j)=self.lock().as_ref(){j.cancel()}} fn take(&self)->Option<Job>{self.lock().take()}}
#[derive(Serialize)] #[serde(rename_all="camelCase")] struct EngineStatus{available:bool,version:Option<String>,detail:String}
#[derive(Serialize)] #[serde(rename_all="camelCase")] struct LocalEntry{name:String,path:String,is_dir:bool,size:u64,modified:Option<u64>}
#[tauri::command] fn zsftp_engine_status()->EngineStatus{let v=zsftp::version();EngineStatus{available:true,version:Some(v),detail:"ZSFTP core integrated with ZeroSpace.".into()}}
#[tauri::command] async fn start_zsftp_transfer(state:State<'_,AppState>,request:TransferConfig)->Result<String,String>{state.start(&request)?;Ok("Transfer started with the integrated ZSFTP engine.".into())}
#[tauri::command] async fn poll_zsftp_transfer(state:State<'_,AppState>,cursor:u64)->Result<Value,String>{state.poll(cursor)}
#[tauri::command] async fn cancel_zsftp_transfer(state:State<'_,AppState>)->Result<(),String>{state.cancel();Ok(())}
#[tauri::command] async fn answer_zsftp_password(state:State<'_,AppState>,password:Option<String>)->Result<(),String>{state.lock().as_ref().ok_or_else(||"no transfer".to_string())?.answer_password(password.as_deref())}
#[tauri::command] async fn close_zsftp_transfer(state:State<'_,AppState>)->Result<(),String>{if let Some(j)=state.take(){tauri::async_runtime::spawn_blocking(move||drop(j)).await.map_err(|e|e.to_string())?}Ok(())}
#[tauri::command] fn list_local_directory(path:String)->Result<Vec<LocalEntry>,String>{let root=PathBuf::from(path);let mut out=Vec::new();for e in fs::read_dir(&root).map_err(|e|e.to_string())?{let e=e.map_err(|e|e.to_string())?;let m=e.metadata().map_err(|e|e.to_string())?;out.push(LocalEntry{name:e.file_name().to_string_lossy().into_owned(),path:e.path().to_string_lossy().into_owned(),is_dir:m.is_dir(),size:if m.is_file(){m.len()}else{0},modified:m.modified().ok().and_then(|t|t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d|d.as_secs())});}out.sort_by(|a,b|b.is_dir.cmp(&a.is_dir).then_with(||a.name.to_lowercase().cmp(&b.name.to_lowercase())));Ok(out)}
#[tauri::command] fn create_local_folder(path:String)->Result<(),String>{fs::create_dir_all(path).map_err(|e|e.to_string())}
#[tauri::command] fn rename_local_path(from:String,to:String)->Result<(),String>{fs::rename(from,to).map_err(|e|e.to_string())}
#[tauri::command] fn delete_local_path(path:String)->Result<(),String>{let p=PathBuf::from(path);if p.is_dir(){fs::remove_dir_all(p)}else{fs::remove_file(p)}.map_err(|e|e.to_string())}

#[derive(Deserialize)] #[serde(rename_all="camelCase")] struct FtpRequest{host:String,port:u16,user:Option<String>,password:Option<String>,path:String}
#[derive(Serialize)] #[serde(rename_all="camelCase")] struct RemoteEntry{name:String,path:String,is_dir:bool,size:Option<u64>}
fn ftp_login(r:&FtpRequest)->Result<FtpStream,String>{let mut ftp=FtpStream::connect((r.host.as_str(),r.port)).map_err(|e|format!("Connect to {}:{}: {e}",r.host,r.port))?;ftp.set_passive_nat_workaround(true);let u=r.user.as_deref().filter(|s|!s.is_empty()).unwrap_or("anonymous");let p=r.password.as_deref().filter(|s|!s.is_empty()).unwrap_or("anonymous@");ftp.login(u,p).map_err(|e|format!("FTP login: {e}"))?;ftp.transfer_type(FileType::Binary).map_err(|e|format!("Set binary mode: {e}"))?;Ok(ftp)}
fn remote_join(base:&str,name:&str)->String{format!("{}/{}",base.trim_end_matches('/'),name).replace("//","/")}
#[tauri::command] async fn ftp_list(request:FtpRequest)->Result<Vec<RemoteEntry>,String>{tauri::async_runtime::spawn_blocking(move||{let mut ftp=ftp_login(&request)?;ftp.cwd(&request.path).map_err(|e|format!("Open {}: {e}",request.path))?;let lines=ftp.list(None).map_err(|e|format!("List {}: {e}",request.path))?;let mut out=Vec::new();for line in lines{let parsed=FtpListFile::try_from(line.as_str());let Ok(file)=parsed else{continue};let name=file.name().to_string();if name.is_empty()||name=="."||name==".."{continue}let is_dir=file.is_directory();out.push(RemoteEntry{name:name.clone(),path:remote_join(&request.path,&name),is_dir,size:if is_dir{None}else{Some(file.size() as u64)}});}let _=ftp.quit();out.sort_by(|a,b|b.is_dir.cmp(&a.is_dir).then_with(||a.name.to_lowercase().cmp(&b.name.to_lowercase())));Ok(out)}).await.map_err(|e|e.to_string())?}
const FTX2_MAGIC:u32=u32::from_le_bytes(*b"FTX2");
const FTX2_HEADER_LEN:usize=28;
const HELPER_MGMT_PORT:u16=9114;
fn helper_rpc(host:&str,frame_type:u16,expected:u16,body:&[u8])->Result<Vec<u8>,String>{
 let mut s=TcpStream::connect((host,HELPER_MGMT_PORT)).map_err(|e|format!("Helper connect {host}:{HELPER_MGMT_PORT}: {e}"))?;
 s.set_read_timeout(Some(Duration::from_secs(10))).map_err(|e|e.to_string())?;
 s.set_write_timeout(Some(Duration::from_secs(10))).map_err(|e|e.to_string())?;
 let mut h=[0u8;FTX2_HEADER_LEN];
 h[0..4].copy_from_slice(&FTX2_MAGIC.to_le_bytes());h[4..6].copy_from_slice(&1u16.to_le_bytes());h[6..8].copy_from_slice(&frame_type.to_le_bytes());h[12..20].copy_from_slice(&(body.len() as u64).to_le_bytes());h[20..28].copy_from_slice(&1u64.to_le_bytes());
 s.write_all(&h).map_err(|e|format!("Helper send header: {e}"))?;if !body.is_empty(){s.write_all(body).map_err(|e|format!("Helper send body: {e}"))?}
 let mut rh=[0u8;FTX2_HEADER_LEN];s.read_exact(&mut rh).map_err(|e|format!("Helper read header: {e}"))?;
 let magic=u32::from_le_bytes(rh[0..4].try_into().unwrap());let version=u16::from_le_bytes(rh[4..6].try_into().unwrap());let kind=u16::from_le_bytes(rh[6..8].try_into().unwrap());let len=u64::from_le_bytes(rh[12..20].try_into().unwrap());
 if magic!=FTX2_MAGIC{return Err("Helper returned invalid FTX2 magic".into())}if version!=1{return Err(format!("Unsupported helper protocol version {version}"))}if len>4*1024*1024{return Err(format!("Helper response too large: {len} bytes"))}
 let mut out=vec![0u8;len as usize];if len>0{s.read_exact(&mut out).map_err(|e|format!("Helper read body: {e}"))?}
 if kind==3{return Err(format!("Helper error: {}",String::from_utf8_lossy(&out)))}if kind!=expected{return Err(format!("Unexpected helper frame {kind}, expected {expected}"))}Ok(out)
}
#[tauri::command] async fn helper_status(host:String)->Result<serde_json::Value,String>{tauri::async_runtime::spawn_blocking(move||{let b=helper_rpc(&host,20,21,&[])?;serde_json::from_slice(&b).map_err(|e|format!("Invalid helper status JSON: {e}"))}).await.map_err(|e|e.to_string())?}
#[tauri::command] async fn helper_list_registered_games(host:String)->Result<serde_json::Value,String>{tauri::async_runtime::spawn_blocking(move||{let b=helper_rpc(&host,62,63,&[])?;serde_json::from_slice(&b).map_err(|e|format!("Invalid helper games JSON: {e}"))}).await.map_err(|e|e.to_string())?}
#[tauri::command] async fn helper_list_screenshots(host:String)->Result<serde_json::Value,String>{tauri::async_runtime::spawn_blocking(move||{let b=helper_rpc(&host,94,95,&[])?;serde_json::from_slice(&b).map_err(|e|format!("Invalid helper screenshots JSON: {e}"))}).await.map_err(|e|e.to_string())?}

#[tauri::command] async fn ftp_create_folder(request:FtpRequest,name:String)->Result<(),String>{tauri::async_runtime::spawn_blocking(move||{let mut ftp=ftp_login(&request)?;ftp.cwd(&request.path).map_err(|e|e.to_string())?;ftp.mkdir(&name).map_err(|e|e.to_string())?;let _=ftp.quit();Ok(())}).await.map_err(|e|e.to_string())?}
#[tauri::command] async fn ftp_delete(request:FtpRequest,name:String,is_dir:bool)->Result<(),String>{tauri::async_runtime::spawn_blocking(move||{let mut ftp=ftp_login(&request)?;ftp.cwd(&request.path).map_err(|e|e.to_string())?;if is_dir{ftp.rmdir(&name)}else{ftp.rm(&name)}.map_err(|e|e.to_string())?;let _=ftp.quit();Ok(())}).await.map_err(|e|e.to_string())?}
#[tauri::command] async fn ftp_rename(request:FtpRequest,from:String,to:String)->Result<(),String>{tauri::async_runtime::spawn_blocking(move||{let mut ftp=ftp_login(&request)?;ftp.cwd(&request.path).map_err(|e|e.to_string())?;ftp.rename(&from,&to).map_err(|e|e.to_string())?;let _=ftp.quit();Ok(())}).await.map_err(|e|e.to_string())?}

#[tauri::command] async fn ftp_read_text(request:FtpRequest,remote_name:String,max_bytes:Option<usize>)->Result<String,String>{tauri::async_runtime::spawn_blocking(move||{let limit=max_bytes.unwrap_or(1024*1024).min(4*1024*1024);let mut ftp=ftp_login(&request)?;ftp.cwd(&request.path).map_err(|e|e.to_string())?;let bytes=ftp.retr(&remote_name,|stream|{let mut out=Vec::new();let mut chunk=[0u8;16384];loop{let n=stream.read(&mut chunk).map_err(FtpError::ConnectionError)?;if n==0{break}if out.len()+n>limit{return Err(FtpError::ConnectionError(std::io::Error::new(std::io::ErrorKind::InvalidData,"remote text file exceeds safety limit")))}out.extend_from_slice(&chunk[..n]);}Ok(out)}).map_err(|e|e.to_string())?;let _=ftp.quit();String::from_utf8(bytes).map_err(|e|format!("Remote file is not UTF-8 text: {e}"))}).await.map_err(|e|e.to_string())?}
#[tauri::command] async fn ftp_upload(request:FtpRequest,local_path:String,remote_name:String)->Result<u64,String>{tauri::async_runtime::spawn_blocking(move||{let mut ftp=ftp_login(&request)?;ftp.cwd(&request.path).map_err(|e|e.to_string())?;let mut file=fs::File::open(&local_path).map_err(|e|e.to_string())?;let size=file.metadata().map_err(|e|e.to_string())?.len();ftp.put_file(&remote_name,&mut file).map_err(|e|e.to_string())?;let _=ftp.quit();Ok(size)}).await.map_err(|e|e.to_string())?}
#[tauri::command] async fn ftp_download(request:FtpRequest,remote_name:String,local_path:String)->Result<u64,String>{tauri::async_runtime::spawn_blocking(move||{let mut ftp=ftp_login(&request)?;ftp.cwd(&request.path).map_err(|e|e.to_string())?;let mut file=fs::File::create(&local_path).map_err(|e|e.to_string())?;let total=ftp.retr(&remote_name,|stream|{let mut buf=[0u8;1024*1024];let mut total=0u64;loop{let n=stream.read(&mut buf).map_err(FtpError::ConnectionError)?;if n==0{break}file.write_all(&buf[..n]).map_err(FtpError::ConnectionError)?;total+=n as u64;}Ok(total)}).map_err(|e|e.to_string())?;let _=ftp.quit();Ok(total)}).await.map_err(|e|e.to_string())?}

static BUNDLED_HELPER:&[u8]=include_bytes!("../resources/helper/zerospace-helper.elf");

#[tauri::command] async fn send_bundled_helper(host:String)->Result<u64,String>{
 tauri::async_runtime::spawn_blocking(move||{
  let bytes=BUNDLED_HELPER;
  if bytes.is_empty(){return Err("Bundled ZeroSpace Helper is empty".into())}
  if bytes.len()>128*1024*1024{return Err("Bundled helper is larger than the 128 MiB safety limit".into())}
  let mut stream=TcpStream::connect_timeout(&format!("{host}:9021").parse().map_err(|e|format!("Invalid address: {e}"))?,Duration::from_secs(8)).map_err(|e|format!("Connect to {host}:9021: {e}"))?;
  stream.set_write_timeout(Some(Duration::from_secs(30))).map_err(|e|e.to_string())?;
  stream.write_all(bytes).map_err(|e|format!("Send bundled helper: {e}"))?;
  stream.flush().map_err(|e|e.to_string())?;
  Ok(bytes.len() as u64)
 }).await.map_err(|e|e.to_string())?
}

#[tauri::command] async fn send_payload(host:String,port:u16,path:String)->Result<u64,String>{
 tauri::async_runtime::spawn_blocking(move||{
  let mut file=fs::File::open(&path).map_err(|e|format!("Open payload: {e}"))?;
  let size=file.metadata().map_err(|e|e.to_string())?.len();
  if size>128*1024*1024{return Err("Payload is larger than the 128 MiB safety limit".into())}
  let mut stream=TcpStream::connect_timeout(&format!("{host}:{port}").parse().map_err(|e|format!("Invalid address: {e}"))?,Duration::from_secs(8)).map_err(|e|format!("Connect to {host}:{port}: {e}"))?;
  stream.set_write_timeout(Some(Duration::from_secs(30))).map_err(|e|e.to_string())?;
  std::io::copy(&mut file,&mut stream).map_err(|e|format!("Send payload: {e}"))?;
  stream.flush().map_err(|e|e.to_string())?;
  Ok(size)
 }).await.map_err(|e|e.to_string())?
}

pub fn run(){tauri::Builder::default().plugin(tauri_plugin_dialog::init()).manage(AppState::default()).invoke_handler(tauri::generate_handler![zsftp_engine_status,start_zsftp_transfer,poll_zsftp_transfer,cancel_zsftp_transfer,answer_zsftp_password,close_zsftp_transfer,list_local_directory,create_local_folder,rename_local_path,delete_local_path,ftp_list,helper_status,helper_list_registered_games,helper_list_screenshots,ftp_create_folder,ftp_delete,ftp_rename,ftp_read_text,ftp_upload,ftp_download,send_payload,send_bundled_helper]).run(tauri::generate_context!()).expect("error while running ZeroSpace");}