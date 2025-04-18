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
        contractId: "CCS6L527IZSNIOGLAYPJLGJXNZ7XY3YMUY2EVRDXA7SOFKS2PGPG56RV",
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
        super(new ContractSpec(["AAAAAAAAAAAAAAAOaW5pdGlhbGl6ZV9kYW8AAAAAAAUAAAAAAAAABG5hbWUAAAARAAAAAAAAAAtkZXNjcmlwdGlvbgAAAAARAAAAAAAAAAxmdW5kaW5nX2dvYWwAAAAGAAAAAAAAAAdjcmVhdG9yAAAAABMAAAAAAAAAEXRva2VuX2NvbnRyYWN0X2lkAAAAAAAAEwAAAAA=",
            "AAAAAAAAAAAAAAAGaW52ZXN0AAAAAAACAAAAAAAAAAhpbnZlc3RvcgAAABMAAAAAAAAABmFtb3VudAAAAAAABgAAAAA=",
            "AAAAAAAAAAAAAAAPY3JlYXRlX3Byb3Bvc2FsAAAAAAEAAAAAAAAAB2RldGFpbHMAAAAAEQAAAAEAAAAG",
            "AAAAAAAAAAAAAAAEdm90ZQAAAAMAAAAAAAAABXZvdGVyAAAAAAAAEwAAAAAAAAALcHJvcG9zYWxfaWQAAAAABgAAAAAAAAAHc3VwcG9ydAAAAAABAAAAAA==",
            "AAAAAAAAAAAAAAAQZXhlY3V0ZV9wcm9wb3NhbAAAAAEAAAAAAAAAC3Byb3Bvc2FsX2lkAAAAAAYAAAAA",
            "AAAAAAAAAAAAAAATcmVjb3JkX2Rpc3RyaWJ1dGlvbgAAAAACAAAAAAAAAAl0aW1lc3RhbXAAAAAAAAAGAAAAAAAAAAZhbW91bnQAAAAAAAYAAAAA",
            "AAAAAAAAAAAAAAAYZ2V0X2Rpc3RyaWJ1dGlvbl9oaXN0b3J5AAAAAAAAAAEAAAPqAAAD7QAAAAIAAAAGAAAABg==",
            "AAAAAAAAAAAAAAATZ2V0X3Byb3Bvc2Fsc19jb3VudAAAAAAAAAAAAQAAAAY=",
            "AAAAAAAAAAAAAAAUZ2V0X3Byb3Bvc2FsX2RldGFpbHMAAAABAAAAAAAAAAtwcm9wb3NhbF9pZAAAAAAGAAAAAQAAABE=",
            "AAAAAAAAAAAAAAASZ2V0X3Byb3Bvc2FsX3ZvdGVzAAAAAAABAAAAAAAAAAtwcm9wb3NhbF9pZAAAAAAGAAAAAQAAAAs=",
            "AAAAAAAAAAAAAAAVZ2V0X3Byb3Bvc2FsX2V4ZWN1dGVkAAAAAAAAAQAAAAAAAAALcHJvcG9zYWxfaWQAAAAABgAAAAEAAAAB",
            "AAAAAAAAAAAAAAAQZ2V0X3ZvdGluZ19wb3dlcgAAAAEAAAAAAAAABXZvdGVyAAAAAAAAEwAAAAEAAAAL",
            "AAAAAAAAAAAAAAAQZ2V0X3RvdGFsX3JhaXNlZAAAAAAAAAABAAAABg==",
            "AAAAAAAAAAAAAAAXaXNfZnVuZGluZ19nb2FsX3JlYWNoZWQAAAAAAAAAAAEAAAAB",
            "AAAAAAAAAAAAAAANZ2V0X2ludmVzdG9ycwAAAAAAAAAAAAABAAAD6gAAABM=",
            "AAAAAAAAAAAAAAASZ2V0X3Rva2VuX2NvbnRyYWN0AAAAAAAAAAAAAQAAABM=",
            "AAAAAAAAAAAAAAAPZ2V0X2ludmVzdG1lbnRzAAAAAAAAAAABAAAD7AAAABMAAAAG",
            "AAAAAAAAAAAAAAAIZ2V0X25hbWUAAAAAAAAAAQAAABE=",
            "AAAAAAAAAAAAAAAPZ2V0X2Rlc2NyaXB0aW9uAAAAAAAAAAABAAAAEQ==",
            "AAAAAAAAAAAAAAAQZ2V0X2Z1bmRpbmdfZ29hbAAAAAAAAAABAAAABg==",
            "AAAAAAAAAAAAAAALZ2V0X2NyZWF0b3IAAAAAAAAAAAEAAAAT",
            "AAAAAAAAAAAAAAAOaW5pdGlhbGl6ZV9wdGMAAAAAAAEAAAAAAAAABWFkbWluAAAAAAAAEwAAAAA=",
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
        initialize_dao: (this.txFromJSON),
        invest: (this.txFromJSON),
        create_proposal: (this.txFromJSON),
        vote: (this.txFromJSON),
        execute_proposal: (this.txFromJSON),
        record_distribution: (this.txFromJSON),
        get_distribution_history: (this.txFromJSON),
        get_proposals_count: (this.txFromJSON),
        get_proposal_details: (this.txFromJSON),
        get_proposal_votes: (this.txFromJSON),
        get_proposal_executed: (this.txFromJSON),
        get_voting_power: (this.txFromJSON),
        get_total_raised: (this.txFromJSON),
        is_funding_goal_reached: (this.txFromJSON),
        get_investors: (this.txFromJSON),
        get_token_contract: (this.txFromJSON),
        get_investments: (this.txFromJSON),
        get_name: (this.txFromJSON),
        get_description: (this.txFromJSON),
        get_funding_goal: (this.txFromJSON),
        get_creator: (this.txFromJSON),
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
