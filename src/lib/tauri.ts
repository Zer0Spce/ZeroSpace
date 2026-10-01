import { invoke } from "@tauri-apps/api/core";
export type TransferRequest={archive:string;host:string;port:number;remotePath:string;username?:string;password?:string};
export type EngineStatus={available:boolean;version:string|null;detail:string};
export const getEngineStatus=()=>invoke<EngineStatus>("zsftp_engine_status");
export const startTransfer=(request:TransferRequest)=>invoke<string>("start_zsftp_transfer",{request});