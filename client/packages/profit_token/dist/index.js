import { Buffer } from "buffer";
import { Client as ContractClient, Spec as ContractSpec, } from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk';
export * as contract from '@stellar/stellar-sdk/contract';
export * as rpc from '@stellar/stellar-sdk/rpc';
if (typeof window !== 'undefined') {
    //@ts-ignore Buffer exists
    window.Buffer = window.Buffer || Buffer;
}
export const networks = {
    testnet: {
        networkPassphrase: "Test SDF Network ; September 2015",
        contractId: "CCFU4LOQZK2TWIQIN6FLDERPAAJ2U2J6Q6DMTPUADGZTBHBFT5XENNLF",
    }
};
export const Errors = {};
export class Client extends ContractClient {
    options;
    static async deploy(
    /** Options for initalizing a Client as well as for calling a method, with extras specific to deploying. */
    options) {
        return ContractClient.deploy(null, options);
    }
    constructor(options) {
        super(new ContractSpec(["AAAAAAAAAAAAAAAOaW5pdGlhbGl6ZV9wdGMAAAAAAAEAAAAAAAAABWFkbWluAAAAAAAAEwAAAAA=",
            "AAAAAAAAAAAAAAAJZ2V0X2FkbWluAAAAAAAAAAAAAAEAAAAT",
            "AAAAAAAAAAAAAAAEbWludAAAAAIAAAAAAAAAAnRvAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAA",
            "AAAAAAAAAAAAAAAEYnVybgAAAAIAAAAAAAAABGZyb20AAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAA",
            "AAAAAAAAAAAAAAAIdHJhbnNmZXIAAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAACdG8AAAAAABMAAAAAAAAABmFtb3VudAAAAAAACwAAAAA=",
            "AAAAAAAAAAAAAAAHYXBwcm92ZQAAAAADAAAAAAAAAAVvd25lcgAAAAAAABMAAAAAAAAAB3NwZW5kZXIAAAAAEwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAA==",
            "AAAAAAAAAAAAAAAJYWxsb3dhbmNlAAAAAAAAAgAAAAAAAAAFb3duZXIAAAAAAAATAAAAAAAAAAdzcGVuZGVyAAAAABMAAAABAAAACw==",
            "AAAAAAAAAAAAAAANdHJhbnNmZXJfZnJvbQAAAAAAAAQAAAAAAAAAB3NwZW5kZXIAAAAAEwAAAAAAAAAEZnJvbQAAABMAAAAAAAAAAnRvAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAA",
            "AAAAAAAAAAAAAAAHYmFsYW5jZQAAAAABAAAAAAAAAAN3aG8AAAAAEwAAAAEAAAAL",
            "AAAAAAAAAAAAAAAMdG90YWxfc3VwcGx5AAAAAAAAAAEAAAAL"]), options);
        this.options = options;
    }
    fromJSON = {
        initialize_ptc: (this.txFromJSON),
        get_admin: (this.txFromJSON),
        mint: (this.txFromJSON),
        burn: (this.txFromJSON),
        transfer: (this.txFromJSON),
        approve: (this.txFromJSON),
        allowance: (this.txFromJSON),
        transfer_from: (this.txFromJSON),
        balance: (this.txFromJSON),
        total_supply: (this.txFromJSON)
    };
}
