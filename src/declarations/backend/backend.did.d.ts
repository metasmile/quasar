import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface HttpHeader { 'value' : string, 'name' : string }
export interface HttpResponse {
  'status' : bigint,
  'body' : Uint8Array | number[],
  'headers' : Array<HttpHeader>,
}
export type Result = { 'Ok' : SupplyDistribution } |
  { 'Err' : string };
export interface SupplyDistribution {
  'date' : string,
  'supply' : number,
  'percentage' : number,
}
export interface TransformArgs {
  'context' : Uint8Array | number[],
  'response' : HttpResponse,
}
export interface TransformContext {
  'function' : [Principal, string],
  'context' : Uint8Array | number[],
}
export interface _SERVICE {
  'fetch_supply_data' : ActorMethod<[string], Result>,
  'get_api_key' : ActorMethod<[], string>,
  'get_last_response' : ActorMethod<[], string>,
  'set_api_key' : ActorMethod<[string], undefined>,
  'transform_response' : ActorMethod<
    [TransformContext, HttpResponse],
    HttpResponse
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
