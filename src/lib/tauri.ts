import { invoke } from "@tauri-apps/api/core";
export type TransferRequest={archive:string;rarPassword?:string;host:string;port:number;mode:"passive"|"active";user:string;password:string;directory:string;mkdir:boolean;verbose:boolean;bufferMib:number};
export type EngineStatus={available:boolean;version:string|null;detail:string};
export type TransferState=Record<string,unknown>;
export const getEngineStatus=()=>invoke<EngineStatus>("zsftp_engine_status");
export const startTransfer=(request:TransferRequest)=>invoke<string>("start_zsftp_transfer",{request});
export const pollTransfer=(cursor=0)=>invoke<TransferState>("poll_zsftp_transfer",{cursor});
export const cancelTransfer=()=>invoke<void>("cancel_zsftp_transfer");
export const closeTransfer=()=>invoke<void>("close_zsftp_transfer");