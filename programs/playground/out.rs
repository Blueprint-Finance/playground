#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
use std::io::Write;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
    218u8,
    7u8,
    92u8,
    178u8,
    255u8,
    94u8,
    198u8,
    129u8,
    118u8,
    19u8,
    222u8,
    83u8,
    11u8,
    105u8,
    42u8,
    135u8,
    53u8,
    71u8,
    119u8,
    105u8,
    218u8,
    71u8,
    67u8,
    12u8,
    189u8,
    129u8,
    84u8,
    51u8,
    92u8,
    74u8,
    131u8,
    39u8,
]);
/// Const version of `ID`
pub const ID_CONST: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
    218u8,
    7u8,
    92u8,
    178u8,
    255u8,
    94u8,
    198u8,
    129u8,
    118u8,
    19u8,
    222u8,
    83u8,
    11u8,
    105u8,
    42u8,
    135u8,
    53u8,
    71u8,
    119u8,
    105u8,
    218u8,
    71u8,
    67u8,
    12u8,
    189u8,
    129u8,
    84u8,
    51u8,
    92u8,
    74u8,
    131u8,
    39u8,
]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
/// Const version of `ID`
pub const fn id_const() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID_CONST
}
use self::playground::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) = unsafe {
        ::solana_program::entrypoint::deserialize(input)
    };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one category for now.
///
/// Global methods - regular methods inside of the `#[program]`.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data)
        .map_err(|e| {
            e.log();
            e.into()
        })
}
fn try_entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct Playground;
    #[automatically_derived]
    impl ::core::clone::Clone for Playground {
        #[inline]
        fn clone(&self) -> Playground {
            Playground
        }
    }
    impl anchor_lang::Id for Playground {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to create a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>:<rust-identifier>")[..8],
///
/// where the namespace can be one type. "global" for a
/// regular instruction.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    use anchor_lang::Discriminator;
    match sighash {
        instruction::Initialize::DISCRIMINATOR => {
            __private::__global::initialize(program_id, accounts, ix_data)
        }
        instruction::Increment::DISCRIMINATOR => {
            __private::__global::increment(program_id, accounts, ix_data)
        }
        instruction::CreateStorage::DISCRIMINATOR => {
            __private::__global::create_storage(program_id, accounts, ix_data)
        }
        instruction::AssignStorage::DISCRIMINATOR => {
            __private::__global::assign_storage(program_id, accounts, ix_data)
        }
        anchor_lang::idl::IDL_IX_TAG_LE => {
            __private::__idl::__idl_dispatch(program_id, accounts, &ix_data)
        }
        anchor_lang::event::EVENT_IX_TAG_LE => {
            Err(anchor_lang::error::ErrorCode::EventInstructionStub.into())
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch<'info>(
            program_id: &Pubkey,
            accounts: &'info [AccountInfo<'info>],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = <IdlCreateAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Resize { data_len } => {
                    let mut bumps = <IdlResizeAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlResizeAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_resize_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Close => {
                    let mut bumps = <IdlCloseAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCloseAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_close_account(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = <IdlCreateBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = <IdlSetBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        use anchor_lang::idl::ERASED_AUTHORITY;
        pub struct IdlAccount {
            pub authority: Pubkey,
            pub data_len: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccount {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "IdlAccount",
                    "authority",
                    &self.authority,
                    "data_len",
                    &&self.data_len,
                )
            }
        }
        impl borsh::ser::BorshSerialize for IdlAccount
        where
            Pubkey: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.data_len, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for IdlAccount
        where
            Pubkey: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    data_len: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IdlAccount {
            #[inline]
            fn clone(&self) -> IdlAccount {
                IdlAccount {
                    authority: ::core::clone::Clone::clone(&self.authority),
                    data_len: ::core::clone::Clone::clone(&self.data_len),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for IdlAccount {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[24, 70, 98, 191, 58, 144, 123, 158]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for IdlAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [24, 70, 98, 191, 58, 144, 123, 158].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[24, 70, 98, 191, 58, 144, 123, 158] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/playground/src/lib.rs",
                                        line: 7u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("IdlAccount"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for IdlAccount {
            const DISCRIMINATOR: [u8; 8] = [24, 70, 98, 191, 58, 144, 123, 158];
        }
        impl IdlAccount {
            pub fn address(program_id: &Pubkey) -> Pubkey {
                let program_signer = Pubkey::find_program_address(&[], program_id).0;
                Pubkey::create_with_seed(&program_signer, IdlAccount::seed(), program_id)
                    .expect("Seed is always valid")
            }
            pub fn seed() -> &'static str {
                "anchor:idl"
            }
        }
        impl anchor_lang::Owner for IdlAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        pub struct IdlCreateAccounts<'info> {
            #[account(signer)]
            pub from: AccountInfo<'info>,
            #[account(mut)]
            pub to: AccountInfo<'info>,
            #[account(seeds = [], bump)]
            pub base: AccountInfo<'info>,
            pub system_program: Program<'info, System>,
            #[account(executable)]
            pub program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateAccountsBumps>
        for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCreateAccountsBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let from: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("from"))?;
                let to: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("to"))?;
                let base: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("base"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("program"))?;
                if !&from.is_signer {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSigner,
                            )
                            .with_account_name("from"),
                    );
                }
                if !&to.is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[],
                    &__program_id,
                );
                __bumps.base = __bump;
                if base.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("base")
                            .with_pubkeys((base.key(), __pda_address)),
                    );
                }
                if !&program.executable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintExecutable,
                            )
                            .with_account_name("program"),
                    );
                }
                Ok(IdlCreateAccounts {
                    from,
                    to,
                    base,
                    system_program,
                    program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.from.to_account_infos());
                account_infos.extend(self.to.to_account_infos());
                account_infos.extend(self.base.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.from.to_account_metas(Some(true)));
                account_metas.extend(self.to.to_account_metas(None));
                account_metas.extend(self.base.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.to, program_id)
                    .map_err(|e| e.with_account_name("to"))?;
                Ok(())
            }
        }
        pub struct IdlCreateAccountsBumps {
            pub base: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "IdlCreateAccountsBumps",
                    "base",
                    &&self.base,
                )
            }
        }
        impl Default for IdlCreateAccountsBumps {
            fn default() -> Self {
                IdlCreateAccountsBumps {
                    base: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCreateAccountsBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_create_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts {
                pub from: Pubkey,
                pub to: Pubkey,
                pub base: Pubkey,
                pub system_program: Pubkey,
                pub program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateAccounts
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.from, writer)?;
                    borsh::BorshSerialize::serialize(&self.to, writer)?;
                    borsh::BorshSerialize::serialize(&self.base, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.from,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.base,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_create_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts<'info> {
                pub from: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub to: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub base: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.from),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.base),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.from),
                        );
                    account_infos
                        .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.to));
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.base),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlAccounts<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlAccountsBumps> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlAccountsBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlAccounts { idl, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlAccountsBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlAccountsBumps")
            }
        }
        impl Default for IdlAccountsBumps {
            fn default() -> Self {
                IdlAccountsBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlAccounts<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlAccountsBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlAccounts`].
            pub struct IdlAccounts {
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlAccounts
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlAccounts`].
            pub struct IdlAccounts<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlResizeAccount<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(mut, constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlResizeAccountBumps>
        for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlResizeAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&authority).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlResizeAccount {
                    idl,
                    authority,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                anchor_lang::AccountsExit::exit(&self.authority, program_id)
                    .map_err(|e| e.with_account_name("authority"))?;
                Ok(())
            }
        }
        pub struct IdlResizeAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlResizeAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlResizeAccountBumps")
            }
        }
        impl Default for IdlResizeAccountBumps {
            fn default() -> Self {
                IdlResizeAccountBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlResizeAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_resize_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount {
                pub idl: Pubkey,
                pub authority: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlResizeAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlResizeAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_resize_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCreateBuffer<'info> {
            #[account(zero)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateBufferBumps>
        for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCreateBufferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let buffer = &__accounts[0];
                *__accounts = &__accounts[1..];
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let __anchor_rent = Rent::get()?;
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = {
                    let mut __data: &[u8] = &buffer.try_borrow_data()?;
                    let mut __disc_bytes = [0u8; 8];
                    __disc_bytes.copy_from_slice(&__data[..8]);
                    let __discriminator = u64::from_le_bytes(__disc_bytes);
                    if __discriminator != 0 {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintZero,
                                )
                                .with_account_name("buffer"),
                        );
                    }
                    match anchor_lang::accounts::account::Account::try_from_unchecked(
                        &buffer,
                    ) {
                        Ok(val) => val,
                        Err(e) => return Err(e.with_account_name("buffer")),
                    }
                };
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        buffer.to_account_info().lamports(),
                        buffer.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlCreateBuffer {
                    buffer,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                Ok(())
            }
        }
        pub struct IdlCreateBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCreateBufferBumps")
            }
        }
        impl Default for IdlCreateBufferBumps {
            fn default() -> Self {
                IdlCreateBufferBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCreateBufferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_create_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer {
                pub buffer: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateBuffer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_create_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlSetBuffer<'info> {
            #[account(mut, constraint = buffer.authority = = idl.authority)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlSetBufferBumps>
        for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlSetBufferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("buffer"))?;
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(buffer.authority == idl.authority) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlSetBuffer {
                    buffer,
                    idl,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlSetBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlSetBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlSetBufferBumps")
            }
        }
        impl Default for IdlSetBufferBumps {
            fn default() -> Self {
                IdlSetBufferBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlSetBufferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_set_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer {
                pub buffer: Pubkey,
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlSetBuffer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlSetBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_set_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCloseAccount<'info> {
            #[account(mut, has_one = authority, close = sol_destination)]
            pub account: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            #[account(mut)]
            pub sol_destination: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCloseAccountBumps>
        for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCloseAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let account: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("account"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let sol_destination: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                if !AsRef::<AccountInfo>::as_ref(&account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("account"),
                    );
                }
                {
                    let my_key = account.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("account")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                {
                    if account.key() == sol_destination.key() {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintClose,
                                )
                                .with_account_name("account"),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !&sol_destination.is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("sol_destination"),
                    );
                }
                Ok(IdlCloseAccount {
                    account,
                    authority,
                    sol_destination,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.account.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.sol_destination.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.account.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.sol_destination.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                {
                    let sol_destination = &self.sol_destination;
                    anchor_lang::AccountsClose::close(
                            &self.account,
                            sol_destination.to_account_info(),
                        )
                        .map_err(|e| e.with_account_name("account"))?;
                }
                anchor_lang::AccountsExit::exit(&self.sol_destination, program_id)
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                Ok(())
            }
        }
        pub struct IdlCloseAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCloseAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCloseAccountBumps")
            }
        }
        impl Default for IdlCloseAccountBumps {
            fn default() -> Self {
                IdlCloseAccountBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCloseAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_close_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount {
                pub account: Pubkey,
                pub authority: Pubkey,
                pub sol_destination: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCloseAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.account, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.sol_destination, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCloseAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.sol_destination,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_close_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount<'info> {
                pub account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub sol_destination: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.sol_destination),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.account),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.sol_destination,
                            ),
                        );
                    account_infos
                }
            }
        }
        use std::cell::{Ref, RefMut};
        pub trait IdlTrailingData<'info> {
            fn trailing_data(self) -> Ref<'info, [u8]>;
            fn trailing_data_mut(self) -> RefMut<'info, [u8]>;
        }
        impl<'a, 'info: 'a> IdlTrailingData<'a> for &'a Account<'info, IdlAccount> {
            fn trailing_data(self) -> Ref<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                Ref::map(info.try_borrow_data().unwrap(), |d| &d[44..])
            }
            fn trailing_data_mut(self) -> RefMut<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                RefMut::map(info.try_borrow_mut_data().unwrap(), |d| &mut d[44..])
            }
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(
                    anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into(),
                );
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = std::cmp::min(8 + 32 + 4 + data_len as usize, 10_000);
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_resize_account(
            program_id: &Pubkey,
            accounts: &mut IdlResizeAccount,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlResizeAccount");
            let data_len: usize = data_len as usize;
            if accounts.idl.data_len != 0 {
                return Err(anchor_lang::error::ErrorCode::IdlAccountNotEmpty.into());
            }
            let idl_ref = AsRef::<AccountInfo>::as_ref(&accounts.idl);
            let new_account_space = idl_ref
                .data_len()
                .checked_add(
                    std::cmp::min(
                        data_len
                            .checked_sub(idl_ref.data_len())
                            .expect(
                                "data_len should always be >= the current account space",
                            ),
                        10_000,
                    ),
                )
                .unwrap();
            if new_account_space > idl_ref.data_len() {
                let sysvar_rent = Rent::get()?;
                let new_rent_minimum = sysvar_rent.minimum_balance(new_account_space);
                anchor_lang::system_program::transfer(
                    anchor_lang::context::CpiContext::new(
                        accounts.system_program.to_account_info(),
                        anchor_lang::system_program::Transfer {
                            from: accounts.authority.to_account_info(),
                            to: accounts.idl.to_account_info(),
                        },
                    ),
                    new_rent_minimum.checked_sub(idl_ref.lamports()).unwrap(),
                )?;
                idl_ref.realloc(new_account_space, false)?;
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_close_account(
            program_id: &Pubkey,
            accounts: &mut IdlCloseAccount,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCloseAccount");
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let prev_len: usize = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.idl.data_len)
                .unwrap();
            let new_len: usize = prev_len.checked_add(idl_data.len()).unwrap() as usize;
            accounts.idl.data_len = accounts
                .idl
                .data_len
                .checked_add(
                    ::std::convert::TryInto::<u32>::try_into(idl_data.len()).unwrap(),
                )
                .unwrap();
            use IdlTrailingData;
            let mut idl_bytes = accounts.idl.trailing_data_mut();
            let idl_expansion = &mut idl_bytes[prev_len..new_len];
            if idl_expansion.len() != idl_data.len() {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireEqViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireEqViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireEqViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/playground/src/lib.rs",
                                    line: 7u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((idl_expansion.len(), idl_data.len())),
                );
            }
            idl_expansion.copy_from_slice(&idl_data[..]);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data_len = accounts.buffer.data_len;
            use IdlTrailingData;
            let buffer_len = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.buffer.data_len)
                .unwrap();
            let mut target = accounts.idl.trailing_data_mut();
            let source = &accounts.buffer.trailing_data()[..buffer_len];
            if target.len() < buffer_len {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireGteViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireGteViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireGteViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/playground/src/lib.rs",
                                    line: 7u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((target.len(), buffer_len)),
                );
            }
            target[..buffer_len].copy_from_slice(source);
            Ok(())
        }
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn initialize<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Initialize");
            let ix = instruction::Initialize::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Initialize = ix;
            let mut __bumps = <Initialize as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Initialize::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = playground::initialize(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn increment<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Increment");
            let ix = instruction::Increment::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Increment = ix;
            let mut __bumps = <Increment as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Increment::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = playground::increment(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn create_storage<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CreateStorage");
            let ix = instruction::CreateStorage::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::CreateStorage = ix;
            let mut __bumps = <CreateStorage as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = CreateStorage::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = playground::create_storage(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn assign_storage<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: AssignStorage");
            let ix = instruction::AssignStorage::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::AssignStorage { entries } = ix;
            let mut __bumps = <AssignStorage as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = AssignStorage::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = playground::assign_storage(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                entries,
            )?;
            __accounts.exit(__program_id)
        }
    }
}
pub mod playground {
    use anchor_lang::Discriminator;
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.user = ctx.accounts.user.key();
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
    pub fn create_storage(ctx: Context<CreateStorage>) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.authority = ctx.accounts.authority.key();
        storage.space = 8;
        storage.total_records = 0;
        storage.positions = Vec::new();
        Ok(())
    }
    pub fn assign_storage(
        ctx: Context<AssignStorage>,
        entries: Vec<Position>,
    ) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        ::solana_program::log::sol_log("Assigning storage");
        let num_entries = entries.len();
        let storage_space = (storage.space - storage.total_records) as usize;
        if num_entries > storage_space {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: PlaygroundError::NotEnoughSpace.name(),
                    error_code_number: PlaygroundError::NotEnoughSpace.into(),
                    error_msg: PlaygroundError::NotEnoughSpace.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/playground/src/lib.rs",
                            line: 45u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        for (i, entry) in entries.into_iter().enumerate() {
            storage.positions.push(entry);
            storage.total_records += 1;
        }
        ::solana_program::log::sol_log(
            &{
                let res = ::alloc::fmt::format(
                    format_args!(
                        "Indices capacity {0}, length {1}",
                        storage.positions.capacity(),
                        storage.positions.len(),
                    ),
                );
                res
            },
        );
        let mut buffer = Vec::<u8>::with_capacity(256);
        buffer.write_all(Storage::discriminator().as_slice()).unwrap();
        storage.serialize(&mut buffer)?;
        ::solana_program::log::sol_log(
            &{
                let res = ::alloc::fmt::format(
                    format_args!("Buffer length: {0}", buffer.len()),
                );
                res
            },
        );
        Ok(())
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction.
    pub struct Initialize;
    impl borsh::ser::BorshSerialize for Initialize {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Initialize {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for Initialize {
        const DISCRIMINATOR: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
    }
    impl anchor_lang::InstructionData for Initialize {}
    impl anchor_lang::Owner for Initialize {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct Increment;
    impl borsh::ser::BorshSerialize for Increment {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Increment {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for Increment {
        const DISCRIMINATOR: [u8; 8] = [11, 18, 104, 9, 104, 174, 59, 33];
    }
    impl anchor_lang::InstructionData for Increment {}
    impl anchor_lang::Owner for Increment {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct CreateStorage;
    impl borsh::ser::BorshSerialize for CreateStorage {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateStorage {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for CreateStorage {
        const DISCRIMINATOR: [u8; 8] = [67, 155, 154, 159, 24, 93, 99, 240];
    }
    impl anchor_lang::InstructionData for CreateStorage {}
    impl anchor_lang::Owner for CreateStorage {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct AssignStorage {
        pub entries: Vec<Position>,
    }
    impl borsh::ser::BorshSerialize for AssignStorage
    where
        Vec<Position>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.entries, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AssignStorage
    where
        Vec<Position>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                entries: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for AssignStorage {
        const DISCRIMINATOR: [u8; 8] = [83, 109, 217, 146, 172, 38, 40, 189];
    }
    impl anchor_lang::InstructionData for AssignStorage {}
    impl anchor_lang::Owner for AssignStorage {
        fn owner() -> Pubkey {
            ID
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_initialize::*;
    pub use crate::__client_accounts_create_storage::*;
    pub use crate::__client_accounts_increment::*;
    pub use crate::__client_accounts_assign_storage::*;
}
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8+8+64)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, InitializeBumps> for Initialize<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut InitializeBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        if __accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let counter = &__accounts[0];
        *__accounts = &__accounts[1..];
        let user: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("user"))?;
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;
        let __anchor_rent = Rent::get()?;
        let counter = {
            let actual_field = AsRef::<AccountInfo>::as_ref(&counter);
            let actual_owner = actual_field.owner;
            let space = 8 + 8 + 64;
            let pa: anchor_lang::accounts::account::Account<Counter> = if !false
                || actual_owner == &anchor_lang::solana_program::system_program::ID
            {
                let __current_lamports = counter.lamports();
                if __current_lamports == 0 {
                    let space = space;
                    let lamports = __anchor_rent.minimum_balance(space);
                    let cpi_accounts = anchor_lang::system_program::CreateAccount {
                        from: user.to_account_info(),
                        to: counter.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::create_account(
                        cpi_context.with_signer(&[]),
                        lamports,
                        space as u64,
                        __program_id,
                    )?;
                } else {
                    if user.key() == counter.key() {
                        return Err(
                            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                    error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .name(),
                                    error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .into(),
                                    error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .to_string(),
                                    error_origin: Some(
                                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                            filename: "programs/playground/src/lib.rs",
                                            line: 64u32,
                                        }),
                                    ),
                                    compared_values: None,
                                })
                                .with_pubkeys((user.key(), counter.key())),
                        );
                    }
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        let cpi_accounts = anchor_lang::system_program::Transfer {
                            from: user.to_account_info(),
                            to: counter.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::transfer(
                            cpi_context,
                            required_lamports,
                        )?;
                    }
                    let cpi_accounts = anchor_lang::system_program::Allocate {
                        account_to_allocate: counter.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::allocate(
                        cpi_context.with_signer(&[]),
                        space as u64,
                    )?;
                    let cpi_accounts = anchor_lang::system_program::Assign {
                        account_to_assign: counter.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::assign(
                        cpi_context.with_signer(&[]),
                        __program_id,
                    )?;
                }
                match anchor_lang::accounts::account::Account::try_from_unchecked(
                    &counter,
                ) {
                    Ok(val) => val,
                    Err(e) => return Err(e.with_account_name("counter")),
                }
            } else {
                match anchor_lang::accounts::account::Account::try_from(&counter) {
                    Ok(val) => val,
                    Err(e) => return Err(e.with_account_name("counter")),
                }
            };
            if false {
                if space != actual_field.data_len() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("counter")
                            .with_values((space, actual_field.data_len())),
                    );
                }
                if actual_owner != __program_id {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("counter")
                            .with_pubkeys((*actual_owner, *__program_id)),
                    );
                }
                {
                    let required_lamports = __anchor_rent.minimum_balance(space);
                    if pa.to_account_info().lamports() < required_lamports {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("counter"),
                        );
                    }
                }
            }
            pa
        };
        if !AsRef::<AccountInfo>::as_ref(&counter).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("counter"),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&counter).is_signer {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSigner,
                    )
                    .with_account_name("counter"),
            );
        }
        if !__anchor_rent
            .is_exempt(
                counter.to_account_info().lamports(),
                counter.to_account_info().try_data_len()?,
            )
        {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("counter"),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&user).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("user"),
            );
        }
        Ok(Initialize {
            counter,
            user,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.counter.to_account_infos());
        account_infos.extend(self.user.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Initialize<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.counter.to_account_metas(Some(true)));
        account_metas.extend(self.user.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Initialize<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.counter, program_id)
            .map_err(|e| e.with_account_name("counter"))?;
        anchor_lang::AccountsExit::exit(&self.user, program_id)
            .map_err(|e| e.with_account_name("user"))?;
        Ok(())
    }
}
pub struct InitializeBumps {}
#[automatically_derived]
impl ::core::fmt::Debug for InitializeBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "InitializeBumps")
    }
}
impl Default for InitializeBumps {
    fn default() -> Self {
        InitializeBumps {}
    }
}
impl<'info> anchor_lang::Bumps for Initialize<'info>
where
    'info: 'info,
{
    type Bumps = InitializeBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_initialize {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`Initialize`].
    pub struct Initialize {
        pub counter: Pubkey,
        pub user: Pubkey,
        pub system_program: Pubkey,
    }
    impl borsh::ser::BorshSerialize for Initialize
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.counter, writer)?;
            borsh::BorshSerialize::serialize(&self.user, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Initialize {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.counter,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.user,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_initialize {
    use super::*;
    /// Generated CPI struct of the accounts for [`Initialize`].
    pub struct Initialize<'info> {
        pub counter: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Initialize<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.counter),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.user),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.counter));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.user));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.system_program),
                );
            account_infos
        }
    }
}
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, IncrementBumps> for Increment<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut IncrementBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let counter: anchor_lang::accounts::account::Account<Counter> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("counter"))?;
        if !AsRef::<AccountInfo>::as_ref(&counter).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("counter"),
            );
        }
        Ok(Increment { counter })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Increment<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.counter.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Increment<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.counter.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Increment<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.counter, program_id)
            .map_err(|e| e.with_account_name("counter"))?;
        Ok(())
    }
}
pub struct IncrementBumps {}
#[automatically_derived]
impl ::core::fmt::Debug for IncrementBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "IncrementBumps")
    }
}
impl Default for IncrementBumps {
    fn default() -> Self {
        IncrementBumps {}
    }
}
impl<'info> anchor_lang::Bumps for Increment<'info>
where
    'info: 'info,
{
    type Bumps = IncrementBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_increment {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`Increment`].
    pub struct Increment {
        pub counter: Pubkey,
    }
    impl borsh::ser::BorshSerialize for Increment
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.counter, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Increment {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.counter,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_increment {
    use super::*;
    /// Generated CPI struct of the accounts for [`Increment`].
    pub struct Increment<'info> {
        pub counter: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Increment<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.counter),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Increment<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.counter));
            account_infos
        }
    }
}
pub struct Counter {
    pub user: Pubkey,
    pub count: u64,
}
impl borsh::ser::BorshSerialize for Counter
where
    Pubkey: borsh::ser::BorshSerialize,
    u64: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.user, writer)?;
        borsh::BorshSerialize::serialize(&self.count, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Counter
where
    Pubkey: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            user: borsh::BorshDeserialize::deserialize_reader(reader)?,
            count: borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Counter {
    #[inline]
    fn clone(&self) -> Counter {
        Counter {
            user: ::core::clone::Clone::clone(&self.user),
            count: ::core::clone::Clone::clone(&self.count),
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for Counter {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> anchor_lang::Result<()> {
        if writer.write_all(&[255, 176, 4, 245, 188, 253, 124, 25]).is_err() {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        if AnchorSerialize::serialize(self, writer).is_err() {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for Counter {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        if buf.len() < [255, 176, 4, 245, 188, 253, 124, 25].len() {
            return Err(
                anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
            );
        }
        let given_disc = &buf[..8];
        if &[255, 176, 4, 245, 188, 253, 124, 25] != given_disc {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                            .name(),
                        error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                            .into(),
                        error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                            .to_string(),
                        error_origin: Some(
                            anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                filename: "programs/playground/src/lib.rs",
                                line: 79u32,
                            }),
                        ),
                        compared_values: None,
                    })
                    .with_account_name("Counter"),
            );
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for Counter {
    const DISCRIMINATOR: [u8; 8] = [255, 176, 4, 245, 188, 253, 124, 25];
}
#[automatically_derived]
impl anchor_lang::Owner for Counter {
    fn owner() -> Pubkey {
        crate::ID
    }
}
pub struct CreateStorage<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [authority.key.as_ref()],
        bump,
        payer = authority,
        space = 8+std::mem::size_of::<Storage>()
    )]
    pub storage: Account<'info, Storage>,
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, CreateStorageBumps> for CreateStorage<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut CreateStorageBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let authority: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("authority"))?;
        if __accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let storage = &__accounts[0];
        *__accounts = &__accounts[1..];
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;
        let __anchor_rent = Rent::get()?;
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[authority.key.as_ref()],
            __program_id,
        );
        __bumps.storage = __bump;
        if storage.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("storage")
                    .with_pubkeys((storage.key(), __pda_address)),
            );
        }
        let storage = {
            let actual_field = AsRef::<AccountInfo>::as_ref(&storage);
            let actual_owner = actual_field.owner;
            let space = 8 + std::mem::size_of::<Storage>();
            let pa: anchor_lang::accounts::account::Account<Storage> = if !false
                || actual_owner == &anchor_lang::solana_program::system_program::ID
            {
                let __current_lamports = storage.lamports();
                if __current_lamports == 0 {
                    let space = space;
                    let lamports = __anchor_rent.minimum_balance(space);
                    let cpi_accounts = anchor_lang::system_program::CreateAccount {
                        from: authority.to_account_info(),
                        to: storage.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::create_account(
                        cpi_context
                            .with_signer(
                                &[&[authority.key.as_ref(), &[__bump][..]][..]],
                            ),
                        lamports,
                        space as u64,
                        __program_id,
                    )?;
                } else {
                    if authority.key() == storage.key() {
                        return Err(
                            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                    error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .name(),
                                    error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .into(),
                                    error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .to_string(),
                                    error_origin: Some(
                                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                            filename: "programs/playground/src/lib.rs",
                                            line: 85u32,
                                        }),
                                    ),
                                    compared_values: None,
                                })
                                .with_pubkeys((authority.key(), storage.key())),
                        );
                    }
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        let cpi_accounts = anchor_lang::system_program::Transfer {
                            from: authority.to_account_info(),
                            to: storage.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::transfer(
                            cpi_context,
                            required_lamports,
                        )?;
                    }
                    let cpi_accounts = anchor_lang::system_program::Allocate {
                        account_to_allocate: storage.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::allocate(
                        cpi_context
                            .with_signer(
                                &[&[authority.key.as_ref(), &[__bump][..]][..]],
                            ),
                        space as u64,
                    )?;
                    let cpi_accounts = anchor_lang::system_program::Assign {
                        account_to_assign: storage.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::assign(
                        cpi_context
                            .with_signer(
                                &[&[authority.key.as_ref(), &[__bump][..]][..]],
                            ),
                        __program_id,
                    )?;
                }
                match anchor_lang::accounts::account::Account::try_from_unchecked(
                    &storage,
                ) {
                    Ok(val) => val,
                    Err(e) => return Err(e.with_account_name("storage")),
                }
            } else {
                match anchor_lang::accounts::account::Account::try_from(&storage) {
                    Ok(val) => val,
                    Err(e) => return Err(e.with_account_name("storage")),
                }
            };
            if false {
                if space != actual_field.data_len() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("storage")
                            .with_values((space, actual_field.data_len())),
                    );
                }
                if actual_owner != __program_id {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("storage")
                            .with_pubkeys((*actual_owner, *__program_id)),
                    );
                }
                {
                    let required_lamports = __anchor_rent.minimum_balance(space);
                    if pa.to_account_info().lamports() < required_lamports {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("storage"),
                        );
                    }
                }
            }
            pa
        };
        if !AsRef::<AccountInfo>::as_ref(&storage).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("storage"),
            );
        }
        if !__anchor_rent
            .is_exempt(
                storage.to_account_info().lamports(),
                storage.to_account_info().try_data_len()?,
            )
        {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("storage"),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&authority).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("authority"),
            );
        }
        Ok(CreateStorage {
            authority,
            storage,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for CreateStorage<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.authority.to_account_infos());
        account_infos.extend(self.storage.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for CreateStorage<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.authority.to_account_metas(None));
        account_metas.extend(self.storage.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for CreateStorage<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.authority, program_id)
            .map_err(|e| e.with_account_name("authority"))?;
        anchor_lang::AccountsExit::exit(&self.storage, program_id)
            .map_err(|e| e.with_account_name("storage"))?;
        Ok(())
    }
}
pub struct CreateStorageBumps {
    pub storage: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for CreateStorageBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "CreateStorageBumps",
            "storage",
            &&self.storage,
        )
    }
}
impl Default for CreateStorageBumps {
    fn default() -> Self {
        CreateStorageBumps {
            storage: u8::MAX,
        }
    }
}
impl<'info> anchor_lang::Bumps for CreateStorage<'info>
where
    'info: 'info,
{
    type Bumps = CreateStorageBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_create_storage {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`CreateStorage`].
    pub struct CreateStorage {
        pub authority: Pubkey,
        pub storage: Pubkey,
        pub system_program: Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateStorage
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            borsh::BorshSerialize::serialize(&self.storage, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for CreateStorage {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.authority,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.storage,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_create_storage {
    use super::*;
    /// Generated CPI struct of the accounts for [`CreateStorage`].
    pub struct CreateStorage<'info> {
        pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub storage: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CreateStorage<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.authority),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.storage),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CreateStorage<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.authority));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.storage));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.system_program),
                );
            account_infos
        }
    }
}
pub struct AssignStorage<'info> {
    #[account(mut)]
    pub storage: Account<'info, Storage>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, AssignStorageBumps> for AssignStorage<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut AssignStorageBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let storage: anchor_lang::accounts::account::Account<Storage> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("storage"))?;
        if !AsRef::<AccountInfo>::as_ref(&storage).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("storage"),
            );
        }
        Ok(AssignStorage { storage })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for AssignStorage<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.storage.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for AssignStorage<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.storage.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for AssignStorage<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.storage, program_id)
            .map_err(|e| e.with_account_name("storage"))?;
        Ok(())
    }
}
pub struct AssignStorageBumps {}
#[automatically_derived]
impl ::core::fmt::Debug for AssignStorageBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "AssignStorageBumps")
    }
}
impl Default for AssignStorageBumps {
    fn default() -> Self {
        AssignStorageBumps {}
    }
}
impl<'info> anchor_lang::Bumps for AssignStorage<'info>
where
    'info: 'info,
{
    type Bumps = AssignStorageBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_assign_storage {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`AssignStorage`].
    pub struct AssignStorage {
        pub storage: Pubkey,
    }
    impl borsh::ser::BorshSerialize for AssignStorage
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.storage, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for AssignStorage {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.storage,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_assign_storage {
    use super::*;
    /// Generated CPI struct of the accounts for [`AssignStorage`].
    pub struct AssignStorage<'info> {
        pub storage: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for AssignStorage<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.storage),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for AssignStorage<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.storage));
            account_infos
        }
    }
}
pub struct IncreaseStorage<'info> {
    #[account(mut)]
    pub storage: Account<'info, Storage>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, IncreaseStorageBumps> for IncreaseStorage<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut IncreaseStorageBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let storage: anchor_lang::accounts::account::Account<Storage> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("storage"))?;
        if !AsRef::<AccountInfo>::as_ref(&storage).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("storage"),
            );
        }
        Ok(IncreaseStorage { storage })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for IncreaseStorage<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.storage.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for IncreaseStorage<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.storage.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for IncreaseStorage<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.storage, program_id)
            .map_err(|e| e.with_account_name("storage"))?;
        Ok(())
    }
}
pub struct IncreaseStorageBumps {}
#[automatically_derived]
impl ::core::fmt::Debug for IncreaseStorageBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "IncreaseStorageBumps")
    }
}
impl Default for IncreaseStorageBumps {
    fn default() -> Self {
        IncreaseStorageBumps {}
    }
}
impl<'info> anchor_lang::Bumps for IncreaseStorage<'info>
where
    'info: 'info,
{
    type Bumps = IncreaseStorageBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_increase_storage {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`IncreaseStorage`].
    pub struct IncreaseStorage {
        pub storage: Pubkey,
    }
    impl borsh::ser::BorshSerialize for IncreaseStorage
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.storage, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for IncreaseStorage {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.storage,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_increase_storage {
    use super::*;
    /// Generated CPI struct of the accounts for [`IncreaseStorage`].
    pub struct IncreaseStorage<'info> {
        pub storage: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for IncreaseStorage<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.storage),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for IncreaseStorage<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.storage));
            account_infos
        }
    }
}
pub struct DecreaseStorage<'info> {
    #[account(mut)]
    pub storage: Account<'info, Storage>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, DecreaseStorageBumps> for DecreaseStorage<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut DecreaseStorageBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let storage: anchor_lang::accounts::account::Account<Storage> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("storage"))?;
        if !AsRef::<AccountInfo>::as_ref(&storage).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("storage"),
            );
        }
        Ok(DecreaseStorage { storage })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for DecreaseStorage<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.storage.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for DecreaseStorage<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.storage.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for DecreaseStorage<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.storage, program_id)
            .map_err(|e| e.with_account_name("storage"))?;
        Ok(())
    }
}
pub struct DecreaseStorageBumps {}
#[automatically_derived]
impl ::core::fmt::Debug for DecreaseStorageBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "DecreaseStorageBumps")
    }
}
impl Default for DecreaseStorageBumps {
    fn default() -> Self {
        DecreaseStorageBumps {}
    }
}
impl<'info> anchor_lang::Bumps for DecreaseStorage<'info>
where
    'info: 'info,
{
    type Bumps = DecreaseStorageBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_decrease_storage {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`DecreaseStorage`].
    pub struct DecreaseStorage {
        pub storage: Pubkey,
    }
    impl borsh::ser::BorshSerialize for DecreaseStorage
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.storage, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for DecreaseStorage {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.storage,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_decrease_storage {
    use super::*;
    /// Generated CPI struct of the accounts for [`DecreaseStorage`].
    pub struct DecreaseStorage<'info> {
        pub storage: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for DecreaseStorage<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.storage),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for DecreaseStorage<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.storage));
            account_infos
        }
    }
}
#[repr(C)]
pub struct Storage {
    pub authority: Pubkey,
    pub space: u8,
    pub total_records: u8,
    pub padding: [u8; 6],
    pub positions: Vec<Position>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Storage {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "Storage",
            "authority",
            &self.authority,
            "space",
            &self.space,
            "total_records",
            &self.total_records,
            "padding",
            &self.padding,
            "positions",
            &&self.positions,
        )
    }
}
impl borsh::ser::BorshSerialize for Storage
where
    Pubkey: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
    u8: borsh::ser::BorshSerialize,
    [u8; 6]: borsh::ser::BorshSerialize,
    Vec<Position>: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.authority, writer)?;
        borsh::BorshSerialize::serialize(&self.space, writer)?;
        borsh::BorshSerialize::serialize(&self.total_records, writer)?;
        borsh::BorshSerialize::serialize(&self.padding, writer)?;
        borsh::BorshSerialize::serialize(&self.positions, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Storage
where
    Pubkey: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    [u8; 6]: borsh::BorshDeserialize,
    Vec<Position>: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
            space: borsh::BorshDeserialize::deserialize_reader(reader)?,
            total_records: borsh::BorshDeserialize::deserialize_reader(reader)?,
            padding: borsh::BorshDeserialize::deserialize_reader(reader)?,
            positions: borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Storage {
    #[inline]
    fn clone(&self) -> Storage {
        Storage {
            authority: ::core::clone::Clone::clone(&self.authority),
            space: ::core::clone::Clone::clone(&self.space),
            total_records: ::core::clone::Clone::clone(&self.total_records),
            padding: ::core::clone::Clone::clone(&self.padding),
            positions: ::core::clone::Clone::clone(&self.positions),
        }
    }
}
#[automatically_derived]
impl anchor_lang::AccountSerialize for Storage {
    fn try_serialize<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> anchor_lang::Result<()> {
        if writer.write_all(&[209, 117, 255, 185, 196, 175, 68, 9]).is_err() {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        if AnchorSerialize::serialize(self, writer).is_err() {
            return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
        }
        Ok(())
    }
}
#[automatically_derived]
impl anchor_lang::AccountDeserialize for Storage {
    fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        if buf.len() < [209, 117, 255, 185, 196, 175, 68, 9].len() {
            return Err(
                anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
            );
        }
        let given_disc = &buf[..8];
        if &[209, 117, 255, 185, 196, 175, 68, 9] != given_disc {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                            .name(),
                        error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                            .into(),
                        error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                            .to_string(),
                        error_origin: Some(
                            anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                filename: "programs/playground/src/lib.rs",
                                line: 118u32,
                            }),
                        ),
                        compared_values: None,
                    })
                    .with_account_name("Storage"),
            );
        }
        Self::try_deserialize_unchecked(buf)
    }
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        let mut data: &[u8] = &buf[8..];
        AnchorDeserialize::deserialize(&mut data)
            .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
    }
}
#[automatically_derived]
impl anchor_lang::Discriminator for Storage {
    const DISCRIMINATOR: [u8; 8] = [209, 117, 255, 185, 196, 175, 68, 9];
}
#[automatically_derived]
impl anchor_lang::Owner for Storage {
    fn owner() -> Pubkey {
        crate::ID
    }
}
pub struct Position {
    pub value: u64,
    pub address: Pubkey,
}
impl borsh::ser::BorshSerialize for Position
where
    u64: borsh::ser::BorshSerialize,
    Pubkey: borsh::ser::BorshSerialize,
{
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
        borsh::BorshSerialize::serialize(&self.value, writer)?;
        borsh::BorshSerialize::serialize(&self.address, writer)?;
        Ok(())
    }
}
impl borsh::de::BorshDeserialize for Position
where
    u64: borsh::BorshDeserialize,
    Pubkey: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            value: borsh::BorshDeserialize::deserialize_reader(reader)?,
            address: borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Position {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Position",
            "value",
            &self.value,
            "address",
            &&self.address,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Position {
    #[inline]
    fn clone(&self) -> Position {
        Position {
            value: ::core::clone::Clone::clone(&self.value),
            address: ::core::clone::Clone::clone(&self.address),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Position {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Position {
    #[inline]
    fn eq(&self, other: &Position) -> bool {
        self.value == other.value && self.address == other.address
    }
}
#[repr(u32)]
pub enum PlaygroundError {
    InvalidOwner,
    NotEnoughSpace,
}
#[automatically_derived]
impl ::core::fmt::Debug for PlaygroundError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                PlaygroundError::InvalidOwner => "InvalidOwner",
                PlaygroundError::NotEnoughSpace => "NotEnoughSpace",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for PlaygroundError {
    #[inline]
    fn clone(&self) -> PlaygroundError {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for PlaygroundError {}
impl PlaygroundError {
    /// Gets the name of this [#enum_name].
    pub fn name(&self) -> String {
        match self {
            PlaygroundError::InvalidOwner => "InvalidOwner".to_string(),
            PlaygroundError::NotEnoughSpace => "NotEnoughSpace".to_string(),
        }
    }
}
impl From<PlaygroundError> for u32 {
    fn from(e: PlaygroundError) -> u32 {
        e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
    }
}
impl From<PlaygroundError> for anchor_lang::error::Error {
    fn from(error_code: PlaygroundError) -> anchor_lang::error::Error {
        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
            error_name: error_code.name(),
            error_code_number: error_code.into(),
            error_msg: error_code.to_string(),
            error_origin: None,
            compared_values: None,
        })
    }
}
impl std::fmt::Display for PlaygroundError {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            PlaygroundError::InvalidOwner => fmt.write_fmt(format_args!("Invalid owner")),
            PlaygroundError::NotEnoughSpace => {
                fmt.write_fmt(format_args!("Not enough storage space"))
            }
        }
    }
}
