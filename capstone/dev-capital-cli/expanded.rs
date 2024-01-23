#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{
    collections::HashMap, fmt::Debug, fs::File, io::Read, path::PathBuf, str::FromStr,
};
use amplify_num::u24;
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::{hash, Hash},
    pubkey::Pubkey, signature::{read_keypair_file, Keypair},
    signer::Signer, system_program,
};
use tracing::{debug, info, Level};
use tracing_subscriber;
mod compress {
    pub struct LookupTable(Vec<(u8, u16)>);
    impl LookupTable {
        pub fn new() -> Self {
            Self {
                0: ::alloc::vec::Vec::new(),
            }
        }
    }
}
mod program_idl {
    use solana_idlgen::idlgen;
    use borsh::{BorshSerialize, to_vec};
    use solana_sdk::{
        signature::{Keypair, Signer},
        message::Message, transaction::Transaction, hash::Hash, pubkey::Pubkey,
        instruction::{Instruction, AccountMeta},
    };
    ///The args struct for our instruction:
    ///initDevFund
    pub struct InitDevFundArgs {
        pub lamports: u64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InitDevFundArgs {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "InitDevFundArgs",
                "lamports",
                &&self.lamports,
            )
        }
    }
    impl borsh::ser::BorshSerialize for InitDevFundArgs {
        fn serialize<W: borsh::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::io::Error> {
            borsh::BorshSerialize::serialize(&self.lamports, writer)?;
            Ok(())
        }
    }
    ///The args struct for our instruction:
    ///initDevDeploy
    pub struct InitDevDeployArgs {
        pub ot_6_len: u32,
        pub ot_5_len: u32,
        pub orig_len: u32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InitDevDeployArgs {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "InitDevDeployArgs",
                "ot_6_len",
                &self.ot_6_len,
                "ot_5_len",
                &self.ot_5_len,
                "orig_len",
                &&self.orig_len,
            )
        }
    }
    impl borsh::ser::BorshSerialize for InitDevDeployArgs {
        fn serialize<W: borsh::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::io::Error> {
            borsh::BorshSerialize::serialize(&self.ot_6_len, writer)?;
            borsh::BorshSerialize::serialize(&self.ot_5_len, writer)?;
            borsh::BorshSerialize::serialize(&self.orig_len, writer)?;
            Ok(())
        }
    }
    ///The args struct for our instruction:
    ///deployOffsets
    pub struct DeployOffsetsArgs {
        pub data: Vec<u8>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DeployOffsetsArgs {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "DeployOffsetsArgs",
                "data",
                &&self.data,
            )
        }
    }
    impl borsh::ser::BorshSerialize for DeployOffsetsArgs {
        fn serialize<W: borsh::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::io::Error> {
            borsh::BorshSerialize::serialize(&self.data, writer)?;
            Ok(())
        }
    }
    pub struct DevCapitalProgram {}
    #[automatically_derived]
    impl ::core::fmt::Debug for DevCapitalProgram {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "DevCapitalProgram")
        }
    }
    impl DevCapitalProgram {
        ///Returns the program ID.
        pub fn id() -> Pubkey {
            Pubkey::new_from_array([
                64,
                160,
                161,
                226,
                172,
                58,
                98,
                62,
                10,
                41,
                57,
                242,
                185,
                234,
                26,
                142,
                104,
                165,
                218,
                172,
                138,
                54,
                44,
                56,
                230,
                220,
                43,
                86,
                66,
                53,
                89,
                1,
            ])
        }
        /**🔑 Required accounts:

1. funder - signer: ✅, mutable: ✅
2. dev - signer: ❌, mutable: ❌
3. devFund - signer: ❌, mutable: ✅
4. systemProgram - signer: ❌, mutable: ❌*/
        pub fn init_dev_fund_ix_from_bytes(
            accounts: &[&Pubkey; 4usize],
            bytes: &[u8],
        ) -> Instruction {
            let account_meta: Vec<AccountMeta> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    AccountMeta::new(accounts[0usize].clone(), true),
                    AccountMeta::new_readonly(accounts[1usize].clone(), false),
                    AccountMeta::new(accounts[2usize].clone(), false),
                    AccountMeta::new_readonly(accounts[3usize].clone(), false),
                ]),
            );
            Instruction::new_with_bytes(Self::id(), &bytes, account_meta)
        }
        /**🔑 Required accounts:

1. funder - signer: ✅, mutable: ✅
2. dev - signer: ❌, mutable: ❌
3. devFund - signer: ❌, mutable: ✅
4. systemProgram - signer: ❌, mutable: ❌*/
        pub fn init_dev_fund_ix(
            accounts: &[&Pubkey; 4usize],
            args: &InitDevFundArgs,
        ) -> Instruction {
            let mut data_bytes: Vec<u8> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([156, 38, 241, 149, 1, 37, 110, 117]),
            );
            data_bytes
                .extend_from_slice(&to_vec(&args).expect("Unable to serialize data"));
            Self::init_dev_fund_ix_from_bytes(accounts, &data_bytes)
        }
        /**🔑 Required accounts:

1. funder - signer: ✅, mutable: ✅
2. dev - signer: ❌, mutable: ❌
3. devFund - signer: ❌, mutable: ✅
4. systemProgram - signer: ❌, mutable: ❌*/
        /**

*/
        /**✍️ Required signers:

1. funder - signer: ✅, mutable: ✅*/
        pub fn init_dev_fund(
            accounts: &[&Pubkey; 4usize],
            args: &InitDevFundArgs,
            payer: Option<&Pubkey>,
            signers: &[&Keypair; 1usize],
            blockhash: Hash,
        ) -> Transaction {
            let ix = Self::init_dev_fund_ix(accounts, args);
            Transaction::new_signed_with_payer(&[ix], payer, signers, blockhash)
        }
        /**🔑 Required accounts:

1. funder - signer: ✅, mutable: ✅
2. dev - signer: ❌, mutable: ❌
3. devFund - signer: ❌, mutable: ✅
4. systemProgram - signer: ❌, mutable: ❌*/
        pub fn init_dev_fund_unsigned(
            accounts: &[&Pubkey; 4usize],
            args: &InitDevFundArgs,
            payer: Option<&Pubkey>,
        ) -> Transaction {
            let ix = Self::init_dev_fund_ix(accounts, args);
            Transaction::new_unsigned(Message::new(&[ix], payer))
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ✅
4. deployOffsets - signer: ❌, mutable: ✅
5. deployData - signer: ❌, mutable: ✅
6. systemProgram - signer: ❌, mutable: ❌*/
        pub fn init_dev_deploy_ix_from_bytes(
            accounts: &[&Pubkey; 6usize],
            bytes: &[u8],
        ) -> Instruction {
            let account_meta: Vec<AccountMeta> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    AccountMeta::new(accounts[0usize].clone(), true),
                    AccountMeta::new(accounts[1usize].clone(), false),
                    AccountMeta::new(accounts[2usize].clone(), false),
                    AccountMeta::new(accounts[3usize].clone(), false),
                    AccountMeta::new(accounts[4usize].clone(), false),
                    AccountMeta::new_readonly(accounts[5usize].clone(), false),
                ]),
            );
            Instruction::new_with_bytes(Self::id(), &bytes, account_meta)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ✅
4. deployOffsets - signer: ❌, mutable: ✅
5. deployData - signer: ❌, mutable: ✅
6. systemProgram - signer: ❌, mutable: ❌*/
        pub fn init_dev_deploy_ix(
            accounts: &[&Pubkey; 6usize],
            args: &InitDevDeployArgs,
        ) -> Instruction {
            let mut data_bytes: Vec<u8> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([53, 171, 67, 243, 141, 241, 97, 228]),
            );
            data_bytes
                .extend_from_slice(&to_vec(&args).expect("Unable to serialize data"));
            Self::init_dev_deploy_ix_from_bytes(accounts, &data_bytes)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ✅
4. deployOffsets - signer: ❌, mutable: ✅
5. deployData - signer: ❌, mutable: ✅
6. systemProgram - signer: ❌, mutable: ❌*/
        /**

*/
        /**✍️ Required signers:

1. dev - signer: ✅, mutable: ✅*/
        pub fn init_dev_deploy(
            accounts: &[&Pubkey; 6usize],
            args: &InitDevDeployArgs,
            payer: Option<&Pubkey>,
            signers: &[&Keypair; 1usize],
            blockhash: Hash,
        ) -> Transaction {
            let ix = Self::init_dev_deploy_ix(accounts, args);
            Transaction::new_signed_with_payer(&[ix], payer, signers, blockhash)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ✅
4. deployOffsets - signer: ❌, mutable: ✅
5. deployData - signer: ❌, mutable: ✅
6. systemProgram - signer: ❌, mutable: ❌*/
        pub fn init_dev_deploy_unsigned(
            accounts: &[&Pubkey; 6usize],
            args: &InitDevDeployArgs,
            payer: Option<&Pubkey>,
        ) -> Transaction {
            let ix = Self::init_dev_deploy_ix(accounts, args);
            Transaction::new_unsigned(Message::new(&[ix], payer))
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ❌
4. deployOffsets - signer: ❌, mutable: ✅*/
        pub fn account_size_offsets_ix_from_bytes(
            accounts: &[&Pubkey; 4usize],
            bytes: &[u8],
        ) -> Instruction {
            let account_meta: Vec<AccountMeta> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    AccountMeta::new(accounts[0usize].clone(), true),
                    AccountMeta::new(accounts[1usize].clone(), false),
                    AccountMeta::new_readonly(accounts[2usize].clone(), false),
                    AccountMeta::new(accounts[3usize].clone(), false),
                ]),
            );
            Instruction::new_with_bytes(Self::id(), &bytes, account_meta)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ❌
4. deployOffsets - signer: ❌, mutable: ✅*/
        pub fn account_size_offsets_ix(accounts: &[&Pubkey; 4usize]) -> Instruction {
            let data_bytes: Vec<u8> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([245, 128, 212, 233, 244, 211, 154, 107]),
            );
            Self::account_size_offsets_ix_from_bytes(accounts, &data_bytes)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ❌
4. deployOffsets - signer: ❌, mutable: ✅*/
        /**

*/
        /**✍️ Required signers:

1. dev - signer: ✅, mutable: ✅*/
        pub fn account_size_offsets(
            accounts: &[&Pubkey; 4usize],
            payer: Option<&Pubkey>,
            signers: &[&Keypair; 1usize],
            blockhash: Hash,
        ) -> Transaction {
            let ix = Self::account_size_offsets_ix(accounts);
            Transaction::new_signed_with_payer(&[ix], payer, signers, blockhash)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ❌
4. deployOffsets - signer: ❌, mutable: ✅*/
        pub fn account_size_offsets_unsigned(
            accounts: &[&Pubkey; 4usize],
            payer: Option<&Pubkey>,
        ) -> Transaction {
            let ix = Self::account_size_offsets_ix(accounts);
            Transaction::new_unsigned(Message::new(&[ix], payer))
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ❌
4. deployData - signer: ❌, mutable: ✅*/
        pub fn account_size_data_ix_from_bytes(
            accounts: &[&Pubkey; 4usize],
            bytes: &[u8],
        ) -> Instruction {
            let account_meta: Vec<AccountMeta> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    AccountMeta::new(accounts[0usize].clone(), true),
                    AccountMeta::new(accounts[1usize].clone(), false),
                    AccountMeta::new_readonly(accounts[2usize].clone(), false),
                    AccountMeta::new(accounts[3usize].clone(), false),
                ]),
            );
            Instruction::new_with_bytes(Self::id(), &bytes, account_meta)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ❌
4. deployData - signer: ❌, mutable: ✅*/
        pub fn account_size_data_ix(accounts: &[&Pubkey; 4usize]) -> Instruction {
            let data_bytes: Vec<u8> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([222, 51, 92, 126, 37, 5, 203, 241]),
            );
            Self::account_size_data_ix_from_bytes(accounts, &data_bytes)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ❌
4. deployData - signer: ❌, mutable: ✅*/
        /**

*/
        /**✍️ Required signers:

1. dev - signer: ✅, mutable: ✅*/
        pub fn account_size_data(
            accounts: &[&Pubkey; 4usize],
            payer: Option<&Pubkey>,
            signers: &[&Keypair; 1usize],
            blockhash: Hash,
        ) -> Transaction {
            let ix = Self::account_size_data_ix(accounts);
            Transaction::new_signed_with_payer(&[ix], payer, signers, blockhash)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ❌
4. deployData - signer: ❌, mutable: ✅*/
        pub fn account_size_data_unsigned(
            accounts: &[&Pubkey; 4usize],
            payer: Option<&Pubkey>,
        ) -> Transaction {
            let ix = Self::account_size_data_ix(accounts);
            Transaction::new_unsigned(Message::new(&[ix], payer))
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ✅
4. deployOffsets - signer: ❌, mutable: ✅
5. deployData - signer: ❌, mutable: ✅
6. systemProgram - signer: ❌, mutable: ❌*/
        pub fn deploy_offsets_ix_from_bytes(
            accounts: &[&Pubkey; 6usize],
            bytes: &[u8],
        ) -> Instruction {
            let account_meta: Vec<AccountMeta> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    AccountMeta::new(accounts[0usize].clone(), true),
                    AccountMeta::new(accounts[1usize].clone(), false),
                    AccountMeta::new(accounts[2usize].clone(), false),
                    AccountMeta::new(accounts[3usize].clone(), false),
                    AccountMeta::new(accounts[4usize].clone(), false),
                    AccountMeta::new_readonly(accounts[5usize].clone(), false),
                ]),
            );
            Instruction::new_with_bytes(Self::id(), &bytes, account_meta)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ✅
4. deployOffsets - signer: ❌, mutable: ✅
5. deployData - signer: ❌, mutable: ✅
6. systemProgram - signer: ❌, mutable: ❌*/
        pub fn deploy_offsets_ix(
            accounts: &[&Pubkey; 6usize],
            args: &DeployOffsetsArgs,
        ) -> Instruction {
            let mut data_bytes: Vec<u8> = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([148, 225, 156, 67, 226, 219, 243, 174]),
            );
            data_bytes
                .extend_from_slice(&to_vec(&args).expect("Unable to serialize data"));
            Self::deploy_offsets_ix_from_bytes(accounts, &data_bytes)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ✅
4. deployOffsets - signer: ❌, mutable: ✅
5. deployData - signer: ❌, mutable: ✅
6. systemProgram - signer: ❌, mutable: ❌*/
        /**

*/
        /**✍️ Required signers:

1. dev - signer: ✅, mutable: ✅*/
        pub fn deploy_offsets(
            accounts: &[&Pubkey; 6usize],
            args: &DeployOffsetsArgs,
            payer: Option<&Pubkey>,
            signers: &[&Keypair; 1usize],
            blockhash: Hash,
        ) -> Transaction {
            let ix = Self::deploy_offsets_ix(accounts, args);
            Transaction::new_signed_with_payer(&[ix], payer, signers, blockhash)
        }
        /**🔑 Required accounts:

1. dev - signer: ✅, mutable: ✅
2. devFund - signer: ❌, mutable: ✅
3. devConfig - signer: ❌, mutable: ✅
4. deployOffsets - signer: ❌, mutable: ✅
5. deployData - signer: ❌, mutable: ✅
6. systemProgram - signer: ❌, mutable: ❌*/
        pub fn deploy_offsets_unsigned(
            accounts: &[&Pubkey; 6usize],
            args: &DeployOffsetsArgs,
            payer: Option<&Pubkey>,
        ) -> Transaction {
            let ix = Self::deploy_offsets_ix(accounts, args);
            Transaction::new_unsigned(Message::new(&[ix], payer))
        }
        pub fn derive_program_address(seeds: &[&[u8]]) -> Pubkey {
            Self::derive_program_address_and_bump(seeds).0
        }
        pub fn derive_program_address_and_bump(seeds: &[&[u8]]) -> (Pubkey, u8) {
            Pubkey::find_program_address(seeds, &Self::id())
        }
    }
}
use crate::program_idl::{DevCapitalProgram, InitDevDeployArgs};
const DEVNET_URL: &str = "https://api.devnet.solana.com";
#[command(author, version, about, long_about = None)]
struct DevCapitalCli {
    /// Operation we want to do (deploy)
    op: String,
    /// Optional PATH to program
    #[arg(short, long, value_name = "program_path")]
    program_path: Option<PathBuf>,
    /// PATH to keypair (json)
    #[arg(short, long, value_name = "keypair_path")]
    keypair_path: Option<PathBuf>,
    #[arg(short, long, value_name = "funder_pubkey")]
    funder_pubkey: Option<String>,
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}
#[automatically_derived]
#[allow(unused_qualifications, clippy::redundant_locals)]
impl clap::Parser for DevCapitalCli {}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::CommandFactory for DevCapitalCli {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("dev-capital-cli");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("dev-capital-cli");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for DevCapitalCli {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = DevCapitalCli {
            op: __clap_arg_matches
                .remove_one::<String>("op")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: op",
                ))?,
            program_path: __clap_arg_matches.remove_one::<PathBuf>("program_path"),
            keypair_path: __clap_arg_matches.remove_one::<PathBuf>("keypair_path"),
            funder_pubkey: __clap_arg_matches.remove_one::<String>("funder_pubkey"),
            debug: __clap_arg_matches
                .remove_one::<u8>("debug")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: debug",
                ))?,
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("op") {
            #[allow(non_snake_case)]
            let op = &mut self.op;
            *op = __clap_arg_matches
                .remove_one::<String>("op")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: op",
                ))?;
        }
        if __clap_arg_matches.contains_id("program_path") {
            #[allow(non_snake_case)]
            let program_path = &mut self.program_path;
            *program_path = __clap_arg_matches.remove_one::<PathBuf>("program_path");
        }
        if __clap_arg_matches.contains_id("keypair_path") {
            #[allow(non_snake_case)]
            let keypair_path = &mut self.keypair_path;
            *keypair_path = __clap_arg_matches.remove_one::<PathBuf>("keypair_path");
        }
        if __clap_arg_matches.contains_id("funder_pubkey") {
            #[allow(non_snake_case)]
            let funder_pubkey = &mut self.funder_pubkey;
            *funder_pubkey = __clap_arg_matches.remove_one::<String>("funder_pubkey");
        }
        if __clap_arg_matches.contains_id("debug") {
            #[allow(non_snake_case)]
            let debug = &mut self.debug;
            *debug = __clap_arg_matches
                .remove_one::<u8>("debug")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: debug",
                ))?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Args for DevCapitalCli {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("DevCapitalCli"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("DevCapitalCli")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 5usize] = [
                                clap::Id::from("op"),
                                clap::Id::from("program_path"),
                                clap::Id::from("keypair_path"),
                                clap::Id::from("funder_pubkey"),
                                clap::Id::from("debug"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("op")
                        .value_name("OP")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("Operation we want to do (deploy)")
                        .long_help(None);
                    let arg = arg;
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("program_path")
                        .value_name("PROGRAM_PATH")
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                PathBuf,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("Optional PATH to program")
                        .long_help(None)
                        .short('p')
                        .long("program-path")
                        .value_name("program_path");
                    let arg = arg;
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("keypair_path")
                        .value_name("KEYPAIR_PATH")
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                PathBuf,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("PATH to keypair (json)")
                        .long_help(None)
                        .short('k')
                        .long("keypair-path")
                        .value_name("keypair_path");
                    let arg = arg;
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("funder_pubkey")
                        .value_name("FUNDER_PUBKEY")
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .short('f')
                        .long("funder-pubkey")
                        .value_name("funder_pubkey");
                    let arg = arg;
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("debug")
                        .value_name("DEBUG")
                        .required(true && clap::ArgAction::Count.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                u8,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Count);
                    let arg = arg
                        .help("Turn debugging information on")
                        .long_help(None)
                        .short('d')
                        .long("debug");
                    let arg = arg;
                    arg
                });
            __clap_app.version("0.1.0").long_about(None)
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("DevCapitalCli")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 5usize] = [
                                clap::Id::from("op"),
                                clap::Id::from("program_path"),
                                clap::Id::from("keypair_path"),
                                clap::Id::from("funder_pubkey"),
                                clap::Id::from("debug"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("op")
                        .value_name("OP")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("Operation we want to do (deploy)")
                        .long_help(None);
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("program_path")
                        .value_name("PROGRAM_PATH")
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                PathBuf,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("Optional PATH to program")
                        .long_help(None)
                        .short('p')
                        .long("program-path")
                        .value_name("program_path");
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("keypair_path")
                        .value_name("KEYPAIR_PATH")
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                PathBuf,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("PATH to keypair (json)")
                        .long_help(None)
                        .short('k')
                        .long("keypair-path")
                        .value_name("keypair_path");
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("funder_pubkey")
                        .value_name("FUNDER_PUBKEY")
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                String,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .short('f')
                        .long("funder-pubkey")
                        .value_name("funder_pubkey");
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("debug")
                        .value_name("DEBUG")
                        .required(true && clap::ArgAction::Count.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                u8,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Count);
                    let arg = arg
                        .help("Turn debugging information on")
                        .long_help(None)
                        .short('d')
                        .long("debug");
                    let arg = arg.required(false);
                    arg
                });
            __clap_app.version("0.1.0").long_about(None)
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for DevCapitalCli {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "DevCapitalCli",
            "op",
            &self.op,
            "program_path",
            &self.program_path,
            "keypair_path",
            &self.keypair_path,
            "funder_pubkey",
            &self.funder_pubkey,
            "debug",
            &&self.debug,
        )
    }
}
fn main() -> Result<()> {
    let body = async {
        let dev_cli = DevCapitalCli::parse();
        tracing_subscriber::fmt()
            .with_max_level(
                match dev_cli.debug > 1 {
                    true => Level::DEBUG,
                    _ => Level::DEBUG,
                },
            )
            .init();
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:55",
                        "dev_capital_cli",
                        ::tracing::Level::DEBUG,
                        ::core::option::Option::Some("src/main.rs"),
                        ::core::option::Option::Some(55u32),
                        ::core::option::Option::Some("dev_capital_cli"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::DEBUG
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                    if (match ::tracing::Level::DEBUG {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    }) <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::DEBUG {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &value_set,
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &format_args!("debug mode enabled") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
                if (match ::tracing::Level::DEBUG {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                }) <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::DEBUG {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!("debug mode enabled") as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            }
        };
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:56",
                        "dev_capital_cli",
                        ::tracing::Level::DEBUG,
                        ::core::option::Option::Some("src/main.rs"),
                        ::core::option::Option::Some(56u32),
                        ::core::option::Option::Some("dev_capital_cli"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::DEBUG
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                    if (match ::tracing::Level::DEBUG {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    }) <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::DEBUG {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &value_set,
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &format_args!("{0:?}", dev_cli) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
                if (match ::tracing::Level::DEBUG {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                }) <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::DEBUG {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!("{0:?}", dev_cli) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            }
        };
        let dev_keypair = read_keypair_file(dev_cli.keypair_path.as_ref().unwrap())
            .expect(
                &{
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Can\'t open keypair file {0:?}",
                            &dev_cli.keypair_path,
                        ),
                    );
                    res
                },
            );
        let funder_pubkey = Pubkey::from_str(&dev_cli.funder_pubkey.unwrap()).unwrap();
        let original_program_bytes = open_file(&dev_cli.program_path.unwrap())?;
        let (offsets_6, compressed_6) = compress_data(&original_program_bytes, 6)?;
        let (offsets_5, compressed_6and5) = compress_data(&compressed_6, 5)?;
        let mut test_decomp = compressed_6and5.clone();
        decompress_data(&offsets_5, 5, &mut test_decomp)?;
        decompress_data(&offsets_6, 6, &mut test_decomp)?;
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:71",
                        "dev_capital_cli",
                        ::tracing::Level::DEBUG,
                        ::core::option::Option::Some("src/main.rs"),
                        ::core::option::Option::Some(71u32),
                        ::core::option::Option::Some("dev_capital_cli"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::DEBUG
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                    if (match ::tracing::Level::DEBUG {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    }) <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::DEBUG {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &value_set,
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &format_args!(
                                            "orig sha256 hash  {0}",
                                            hash(&original_program_bytes),
                                        ) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
                if (match ::tracing::Level::DEBUG {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                }) <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::DEBUG {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!(
                                                                    "orig sha256 hash  {0}",
                                                                    hash(&original_program_bytes),
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            }
        };
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:72",
                        "dev_capital_cli",
                        ::tracing::Level::DEBUG,
                        ::core::option::Option::Some("src/main.rs"),
                        ::core::option::Option::Some(72u32),
                        ::core::option::Option::Some("dev_capital_cli"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::DEBUG
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                    if (match ::tracing::Level::DEBUG {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    }) <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::DEBUG {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &value_set,
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &format_args!("decomp sha256 hash {0}", hash(&test_decomp))
                                            as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
                if (match ::tracing::Level::DEBUG {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                }) <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::DEBUG {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!("decomp sha256 hash {0}", hash(&test_decomp))
                                                                    as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            }
        };
        let rpc_client = RpcClient::new("http://localhost:8899".into());
        let recent_blockhash = rpc_client.get_latest_blockhash().await?;
        init_dev_deploy(
                (offsets_6.len() * 3) as u32,
                (offsets_5.len() * 3) as u32,
                original_program_bytes.len() as u32,
                &rpc_client,
                recent_blockhash,
                &dev_keypair,
                &funder_pubkey,
            )
            .await?;
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:96",
                        "dev_capital_cli",
                        ::tracing::Level::DEBUG,
                        ::core::option::Option::Some("src/main.rs"),
                        ::core::option::Option::Some(96u32),
                        ::core::option::Option::Some("dev_capital_cli"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::DEBUG
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::DEBUG
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                    if (match ::tracing::Level::DEBUG {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    }) <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::DEBUG {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &value_set,
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &format_args!("program finished") as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
                if (match ::tracing::Level::DEBUG {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                }) <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::DEBUG {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!("program finished") as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            }
        };
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
async fn init_dev_deploy(
    offsets_6_len: impl Into<u32>,
    offsets_5_len: impl Into<u32>,
    original_len: impl Into<u32>,
    rpc_client: &RpcClient,
    recent_hash: Hash,
    signer: &Keypair,
    funder: &Pubkey,
) -> Result<()> {
    let dev_fund = DevCapitalProgram::derive_program_address(
        &[b"dev_fund", funder.as_ref(), signer.pubkey().to_bytes().as_ref()],
    );
    let dev_config = DevCapitalProgram::derive_program_address(
        &[b"dev_config", dev_fund.as_ref(), signer.pubkey().to_bytes().as_ref()],
    );
    let deploy_offsets = DevCapitalProgram::derive_program_address(
        &[b"deploy_offsets", dev_fund.as_ref(), signer.pubkey().to_bytes().as_ref()],
    );
    let deploy_data = DevCapitalProgram::derive_program_address(
        &[b"deploy_data", dev_fund.as_ref(), signer.pubkey().to_bytes().as_ref()],
    );
    let args = InitDevDeployArgs {
        ot_6_len: offsets_6_len.into(),
        ot_5_len: offsets_5_len.into(),
        orig_len: original_len.into(),
    };
    let tx = DevCapitalProgram::init_dev_deploy(
        &[
            &signer.pubkey(),
            &dev_fund,
            &dev_config,
            &deploy_offsets,
            &deploy_data,
            &system_program::id(),
        ],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        recent_hash,
    );
    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)
        .await
        .expect("Failed to send transaction");
    {
        use ::tracing::__macro_support::Callsite as _;
        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "event src/main.rs:159",
                    "dev_capital_cli",
                    ::tracing::Level::INFO,
                    ::core::option::Option::Some("src/main.rs"),
                    ::core::option::Option::Some(159u32),
                    ::core::option::Option::Some("dev_capital_cli"),
                    ::tracing_core::field::FieldSet::new(
                        &["message"],
                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                    ),
                    ::tracing::metadata::Kind::EVENT,
                )
            };
            ::tracing::callsite::DefaultCallsite::new(&META)
        };
        let enabled = ::tracing::Level::INFO
            <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            && {
                let interest = __CALLSITE.interest();
                !interest.is_never()
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
            };
        if enabled {
            (|value_set: ::tracing::field::ValueSet| {
                let meta = __CALLSITE.metadata();
                ::tracing::Event::dispatch(meta, &value_set);
                if (match ::tracing::Level::INFO {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                }) <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &value_set,
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            })({
                #[allow(unused_imports)]
                use ::tracing::field::{debug, display, Value};
                let mut iter = __CALLSITE.metadata().fields().iter();
                __CALLSITE
                    .metadata()
                    .fields()
                    .value_set(
                        &[
                            (
                                &::core::iter::Iterator::next(&mut iter)
                                    .expect("FieldSet corrupted (this is a bug)"),
                                ::core::option::Option::Some(
                                    &format_args!(
                                        "InitDevDeploy tx https://explorer.solana.com/transaction/{0}?cluster=custom&customUrl=http://localhost:8899",
                                        signature,
                                    ) as &dyn Value,
                                ),
                            ),
                        ],
                    )
            });
        } else {
            if (match ::tracing::Level::INFO {
                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                _ => ::tracing::log::Level::Trace,
            }) <= ::tracing::log::STATIC_MAX_LEVEL
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        use ::tracing::log;
                        let level = match ::tracing::Level::INFO {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        };
                        if level <= log::max_level() {
                            let meta = __CALLSITE.metadata();
                            let log_meta = log::Metadata::builder()
                                .level(level)
                                .target(meta.target())
                                .build();
                            let logger = log::logger();
                            if logger.enabled(&log_meta) {
                                ::tracing::__macro_support::__tracing_log(
                                    meta,
                                    logger,
                                    log_meta,
                                    &{
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "InitDevDeploy tx https://explorer.solana.com/transaction/{0}?cluster=custom&customUrl=http://localhost:8899",
                                                                signature,
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    },
                                )
                            }
                        }
                    }
                } else {
                    {}
                }
            } else {
                {}
            };
        }
    };
    Ok(())
}
async fn send_data(
    offsets_6: Vec<u8>,
    offsets_5: Vec<u8>,
    compressed_data: Vec<u8>,
    orig_len: usize,
) -> Result<()> {
    let mut blob: Vec<u8> = ::alloc::vec::Vec::new();
    let offsets_6_len: u24 = u24::try_from(offsets_6.len() as u32 / 3 as u32).unwrap();
    let offsets_5_len: u24 = u24::try_from(offsets_5.len() as u32 / 3 as u32).unwrap();
    blob.extend_from_slice(&offsets_6_len.to_le_bytes());
    blob.extend_from_slice(&offsets_6);
    blob.extend_from_slice(&offsets_5_len.to_le_bytes());
    blob.extend_from_slice(&offsets_5);
    let offsets_chunks = split_to_chunks(&blob, 1000);
    let data_chunks = split_to_chunks(&compressed_data, 1000);
    {
        use ::tracing::__macro_support::Callsite as _;
        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "event src/main.rs:180",
                    "dev_capital_cli",
                    ::tracing::Level::DEBUG,
                    ::core::option::Option::Some("src/main.rs"),
                    ::core::option::Option::Some(180u32),
                    ::core::option::Option::Some("dev_capital_cli"),
                    ::tracing_core::field::FieldSet::new(
                        &["message"],
                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                    ),
                    ::tracing::metadata::Kind::EVENT,
                )
            };
            ::tracing::callsite::DefaultCallsite::new(&META)
        };
        let enabled = ::tracing::Level::DEBUG
            <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::DEBUG
                <= ::tracing::level_filters::LevelFilter::current()
            && {
                let interest = __CALLSITE.interest();
                !interest.is_never()
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
            };
        if enabled {
            (|value_set: ::tracing::field::ValueSet| {
                let meta = __CALLSITE.metadata();
                ::tracing::Event::dispatch(meta, &value_set);
                if (match ::tracing::Level::DEBUG {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                }) <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::DEBUG {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &value_set,
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            })({
                #[allow(unused_imports)]
                use ::tracing::field::{debug, display, Value};
                let mut iter = __CALLSITE.metadata().fields().iter();
                __CALLSITE
                    .metadata()
                    .fields()
                    .value_set(
                        &[
                            (
                                &::core::iter::Iterator::next(&mut iter)
                                    .expect("FieldSet corrupted (this is a bug)"),
                                ::core::option::Option::Some(
                                    &format_args!(
                                        "Offsets chunks len {0}",
                                        offsets_chunks.len(),
                                    ) as &dyn Value,
                                ),
                            ),
                        ],
                    )
            });
        } else {
            if (match ::tracing::Level::DEBUG {
                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                _ => ::tracing::log::Level::Trace,
            }) <= ::tracing::log::STATIC_MAX_LEVEL
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        use ::tracing::log;
                        let level = match ::tracing::Level::DEBUG {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        };
                        if level <= log::max_level() {
                            let meta = __CALLSITE.metadata();
                            let log_meta = log::Metadata::builder()
                                .level(level)
                                .target(meta.target())
                                .build();
                            let logger = log::logger();
                            if logger.enabled(&log_meta) {
                                ::tracing::__macro_support::__tracing_log(
                                    meta,
                                    logger,
                                    log_meta,
                                    &{
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Offsets chunks len {0}",
                                                                offsets_chunks.len(),
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    },
                                )
                            }
                        }
                    }
                } else {
                    {}
                }
            } else {
                {}
            };
        }
    };
    {
        use ::tracing::__macro_support::Callsite as _;
        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "event src/main.rs:181",
                    "dev_capital_cli",
                    ::tracing::Level::DEBUG,
                    ::core::option::Option::Some("src/main.rs"),
                    ::core::option::Option::Some(181u32),
                    ::core::option::Option::Some("dev_capital_cli"),
                    ::tracing_core::field::FieldSet::new(
                        &["message"],
                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                    ),
                    ::tracing::metadata::Kind::EVENT,
                )
            };
            ::tracing::callsite::DefaultCallsite::new(&META)
        };
        let enabled = ::tracing::Level::DEBUG
            <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && ::tracing::Level::DEBUG
                <= ::tracing::level_filters::LevelFilter::current()
            && {
                let interest = __CALLSITE.interest();
                !interest.is_never()
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
            };
        if enabled {
            (|value_set: ::tracing::field::ValueSet| {
                let meta = __CALLSITE.metadata();
                ::tracing::Event::dispatch(meta, &value_set);
                if (match ::tracing::Level::DEBUG {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                }) <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::DEBUG {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &value_set,
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            })({
                #[allow(unused_imports)]
                use ::tracing::field::{debug, display, Value};
                let mut iter = __CALLSITE.metadata().fields().iter();
                __CALLSITE
                    .metadata()
                    .fields()
                    .value_set(
                        &[
                            (
                                &::core::iter::Iterator::next(&mut iter)
                                    .expect("FieldSet corrupted (this is a bug)"),
                                ::core::option::Option::Some(
                                    &format_args!("Data chunks len {0}", data_chunks.len())
                                        as &dyn Value,
                                ),
                            ),
                        ],
                    )
            });
        } else {
            if (match ::tracing::Level::DEBUG {
                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                _ => ::tracing::log::Level::Trace,
            }) <= ::tracing::log::STATIC_MAX_LEVEL
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        use ::tracing::log;
                        let level = match ::tracing::Level::DEBUG {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        };
                        if level <= log::max_level() {
                            let meta = __CALLSITE.metadata();
                            let log_meta = log::Metadata::builder()
                                .level(level)
                                .target(meta.target())
                                .build();
                            let logger = log::logger();
                            if logger.enabled(&log_meta) {
                                ::tracing::__macro_support::__tracing_log(
                                    meta,
                                    logger,
                                    log_meta,
                                    &{
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!("Data chunks len {0}", data_chunks.len())
                                                                as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    },
                                )
                            }
                        }
                    }
                } else {
                    {}
                }
            } else {
                {}
            };
        }
    };
    Ok(())
}
fn split_to_chunks(data: &Vec<u8>, chunk_size: usize) -> Vec<(u16, Vec<u8>)> {
    let mut data_chunks: Vec<(u16, Vec<u8>)> = ::alloc::vec::Vec::new();
    for (index, chunk) in data.chunks(chunk_size).enumerate() {
        data_chunks.push((index as u16, chunk.to_vec()));
    }
    data_chunks
}
fn open_file(path: &PathBuf) -> Result<Vec<u8>> {
    let mut file_buf = ::alloc::vec::Vec::new();
    File::open(path).and_then(|mut file| file.read_to_end(&mut file_buf))?;
    Ok(file_buf)
}
fn compress_data(data: &Vec<u8>, needle_len: usize) -> Result<(Vec<u8>, Vec<u8>)> {
    let needle = ::alloc::vec::from_elem(0u8, needle_len);
    let mut found_offsets: Vec<u8> = ::alloc::vec::Vec::new();
    let mut compressed_data: Vec<u8> = ::alloc::vec::Vec::new();
    let mut skip_counter = 0;
    for index in 0..data.len() {
        if skip_counter > 0 {
            skip_counter -= 1;
            continue;
        }
        if index + needle.len() <= data.len() {
            if data[index..index + needle.len()] == needle {
                found_offsets
                    .append(
                        &mut u24::try_from(index as u32).unwrap().to_le_bytes().to_vec(),
                    );
                skip_counter += needle.len() - 1;
                continue;
            }
        }
        compressed_data.push(data[index]);
    }
    Ok((found_offsets, compressed_data))
}
fn decompress_data(
    offsets: &Vec<u8>,
    length: u8,
    compressed_data: &mut Vec<u8>,
) -> Result<()> {
    for offset_ in offsets.chunks_exact(3) {
        let mut offset_bytes: [u8; 3] = [0u8; 3];
        offset_bytes.copy_from_slice(offset_);
        let offset = u24::from_le_bytes(offset_bytes);
        for _ in 0..length {
            compressed_data.insert(offset.into(), 0u8);
        }
    }
    Ok(())
}
