import { invoke } from "@tauri-apps/api/core";
export type TransferRequest={archive:string;rarPassword?:string;host:string;port:number;mode:"passive"|"active";user:string;password:string;directory:string;mkdir:boolean;verbose:boolean;bufferMib:number};
export type EngineStatus={available:boolean;version:string|null;detail:string};
export type TransferState=Record<string,unknown>;
export const getEngineStatus=()=>invoke<EngineStatus>("zsftp_engine_status");
export const startTransfer=(request:TransferRequest)=>invoke<string>("start_zsftp_transfer",{request});
export const pollTransfer=(cursor=0)=>invoke<TransferState>("poll_zsftp_transfer",{cursor});
export const cancelTransfer=()=>invoke<void>("cancel_zsftp_transfer");
export const closeTransfer=()=>invoke<void>("close_zsftp_transfer");
export type LocalEntry={name:string;path:string;isDir:boolean;size:number;modified:number|null};
export const listLocalDirectory=(path:string)=>invoke<LocalEntry[]>("list_local_directory",{path});
export const createLocalFolder=(path:string)=>invoke<void>("create_local_folder",{path});
export const renameLocalPath=(from:string,to:string)=>invoke<void>("rename_local_path",{from,to});
export const deleteLocalPath=(path:string)=>invoke<void>("delete_local_path",{path});

export type FtpRequest={host:string;port:number;user?:string;password?:string;path:string};
export type RemoteEntry={name:string;path:string;isDir:boolean;size:number|null};
export const ftpList=(request:FtpRequest)=>invoke<RemoteEntry[]>("ftp_list",{request});
export const ftpCreateFolder=(request:FtpRequest,name:string)=>invoke<void>("ftp_create_folder",{request,name});
export const ftpDelete=(request:FtpRequest,name:string,isDir:boolean)=>invoke<void>("ftp_delete",{request,name,isDir});
export const ftpRename=(request:FtpRequest,from:string,to:string)=>invoke<void>("ftp_rename",{request,from,to});

export const ftpUpload=(request:FtpRequest,localPath:string,remoteName:string)=>invoke<number>("ftp_upload",{request,localPath,remoteName});
export const ftpDownload=(request:FtpRequest,remoteName:string,localPath:string)=>invoke<number>("ftp_download",{request,remoteName,localPath});
