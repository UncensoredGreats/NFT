use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(icrc7), forward_attrs(allow, doc, cfg))]
struct Opts {
    token_type: String,
    symbol: String,
    name: String,
    description: Option<String>,
    logo: Option<String>,
    assets_origin: Option<String>,
    total_supply: Option<u64>,
    supply_cap: Option<u64>,
    max_query_batch_size: Option<usize>,
    max_update_batch_size: Option<usize>,
    default_take_value: Option<usize>,
    max_take_value: Option<usize>,
    max_memo_size: Option<usize>,
    atomic_batch_transfers: Option<bool>,
    tx_window: Option<i64>,
    permitted_drift: Option<i64>,
    mutable: Option<bool>,
}

#[proc_macro_derive(Icrc7, attributes(icrc7))]
pub fn derive_icrc7(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;
    let tt = opts.token_type.clone();
    let token_type: proc_macro2::TokenStream = tt.parse().unwrap();
    let symbol = opts.symbol.clone();
    let name = opts.name.clone();
    let description = match opts.description {
        Some(d) => d.clone(),
        None => "".to_string(),
    };
    let logo = match opts.logo {
        Some(d) => d.clone(),
        None => "".to_string(),
    };
    let supply_cap = match opts.supply_cap {
        Some(x) => quote! {
            fn supply_cap() -> Option<usize>{
                Some(#x)
            }
        },
        None => quote! {},
    };
    let max_query_batch_size = match opts.max_query_batch_size {
        Some(x) => quote! {
            fn max_query_batch_size() -> usize{
                #x
            }
        },
        None => quote! {},
    };
    let max_update_batch_size = match opts.max_update_batch_size {
        Some(x) => quote! {
            fn max_update_batch_size() -> usize{
                #x
            }
        },
        None => quote! {},
    };
    let default_take_value = match opts.default_take_value {
        Some(x) => quote! {
            fn default_take_value() -> usize{
                #x
            }
        },
        None => quote! {},
    };
    let max_take_value = match opts.max_take_value {
        Some(x) => quote! {
            fn max_take_value() -> usize{
                #x
            }
        },
        None => quote! {},
    };
    let max_memo_size = match opts.max_memo_size {
        Some(x) => quote! {
            fn max_memo_size() -> usize{
                #x
            }
        },
        None => quote! {},
    };
    let atomic_batch_transfers = match opts.atomic_batch_transfers {
        Some(x) => quote! {
            fn atomic_batch_transfers() -> bool{
                #x
            }
        },
        None => quote! {},
    };
    let tx_window = match opts.tx_window {
        Some(x) => quote! {
            fn tx_window() -> usize{
                #x
            }
        },
        None => quote! {},
    };
    let permitted_drift = match opts.permitted_drift {
        Some(x) => quote! {
            fn permitted_drift() -> usize{
                #x
            }
        },
        None => quote! {},
    };
    let output = quote! {
        impl uncensored_greats_dao::Icrc7<#token_type> for #ident {
            fn symbol() -> &'static str{
                #symbol
            }
            fn name() -> &'static str{
                #name
            }
            fn description() -> &'static str{
                #description
            }
            fn logo() -> &'static str{
                #logo
            }
            #supply_cap
            #max_query_batch_size
            #max_update_batch_size
            #default_take_value
            #max_take_value
            #max_memo_size
            #atomic_batch_transfers
            #tx_window
            #permitted_drift
        }

        use uncensored_greats_dao::ic_cdk;
        use uncensored_greats_dao::candid;
        use uncensored_greats_dao::num_traits::cast::ToPrimitive;

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_symbol() -> String {
            #ident::symbol().to_string()
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_name() -> String {
            #ident::name().to_string()
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_description() -> Option<String> {
            Some(#ident::description().to_string())
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_logo() -> Option<String> {
            Some(#ident::logo().to_string())
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_total_supply() -> uncensored_greats_dao::candid::Nat {
            #ident::total_supply().into()
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_supply_cap() -> Option<uncensored_greats_dao::candid::Nat> {
            match #ident::supply_cap(){
                Some(sc) => Some(sc.into()),
                None => None
            }
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_max_query_batch_size() -> Option<uncensored_greats_dao::candid::Nat> {
            Some(#ident::max_query_batch_size().into())
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_max_update_batch_size() -> Option<uncensored_greats_dao::candid::Nat> {
            Some(#ident::max_update_batch_size().into())
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_default_take_value() -> Option<uncensored_greats_dao::candid::Nat> {
            Some(#ident::default_take_value().into())
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_max_take_value() -> Option<uncensored_greats_dao::candid::Nat> {
            Some(#ident::max_take_value().into())
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_max_memo_size() -> Option<uncensored_greats_dao::candid::Nat> {
            Some(#ident::max_memo_size().into())
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_atomic_batch_transfers() -> bool {
            #ident::atomic_batch_transfers()
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_tx_window() -> Option<uncensored_greats_dao::candid::Nat> {
            Some(#ident::tx_window().into())
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_permitted_drift() -> Option<uncensored_greats_dao::candid::Nat> {
            Some(#ident::permitted_drift().into())
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_token_metadata(token_ids: Vec<uncensored_greats_dao::candid::Nat>) -> Vec<String> {
            match #ident::token_metadata(token_ids.into_iter().map(|i| i.0.to_u64().unwrap_or(0)).collect()){
                Ok(map) => {
                    map
                }
                Err(e) => {
                    uncensored_greats_dao::ic_cdk::trap(&e.to_string());
                }
            }
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_owner_of(token_ids: Vec<uncensored_greats_dao::candid::Nat>) -> Vec<Option<uncensored_greats_dao::icrc_ledger_types::icrc1::account::Account>> {
            match #ident::owner_of(token_ids.into_iter().map(|i| i.0.to_u64().unwrap_or(0)).collect()){
                Ok(map) => {
                    map.into_iter().map(|p| match p{
                        Some(pp) => Some(uncensored_greats_dao::icrc_ledger_types::icrc1::account::Account{owner: pp, subaccount: None}),
                        None => None
                    }).collect()
                }
                Err(e) => {
                    uncensored_greats_dao::ic_cdk::trap(&e.to_string());
                }
            }
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_balance_of(accounts: Vec<uncensored_greats_dao::icrc_ledger_types::icrc1::account::Account>) -> Vec<uncensored_greats_dao::candid::Nat> {
            match #ident::balance_of(accounts.into_iter().map(|a| a.owner).collect()){
                Ok(m) => {
                    m.into_iter().map(|i| i.into()).collect()
                }
                Err(e) => {
                    uncensored_greats_dao::ic_cdk::trap(&e.to_string());
                }
            }
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_tokens(prev: Option<uncensored_greats_dao::candid::Nat>, take: Option<uncensored_greats_dao::candid::Nat>) -> Vec<uncensored_greats_dao::candid::Nat> {
            match #ident::tokens(match prev{
                Some(s) => Some(s.0.to_u64().unwrap_or(0) as usize),
                None => None
            }, match take{
                Some(s) => Some(s.0.to_u64().unwrap_or(0) as usize),
                None => None
            }){
                Ok(m) => {
                    m.into_iter().map(|i| i.into()).collect()
                }
                Err(e) => {
                    uncensored_greats_dao::ic_cdk::trap(&e.to_string());
                }

            }
        }

        #[uncensored_greats_dao::ic_cdk::query]
        pub fn icrc7_tokens_of(account: uncensored_greats_dao::icrc_ledger_types::icrc1::account::Account, prev: Option<uncensored_greats_dao::candid::Nat>, take: Option<uncensored_greats_dao::candid::Nat>) -> Vec<uncensored_greats_dao::candid::Nat> {
            match #ident::tokens_of(account.owner, match prev{
                Some(s) => Some(s.0.to_u64().unwrap_or(0) as usize),
                None => None
            }, match take{
                Some(s) => Some(s.0.to_u64().unwrap_or(0) as usize),
                None => None
            }){
                Ok(m) => {
                    m.into_iter().map(|i| i.into()).collect()
                }
                Err(e) => {
                    uncensored_greats_dao::ic_cdk::trap(&e.to_string());
                }
            }
        }

        #[derive(uncensored_greats_dao::candid::CandidType, Deserialize, Clone, Debug)]
        pub struct TransferArg {
            pub to: uncensored_greats_dao::icrc_ledger_types::icrc1::account::Account,
            pub token_id: uncensored_greats_dao::candid::Nat,
            pub memo: Option<uncensored_greats_dao::icrc_ledger_types::icrc1::transfer::Memo>,
            pub created_at_time: Option<u64>,
        }

        #[uncensored_greats_dao::ic_cdk::update]
        pub fn icrc7_transfer(
            args: Vec<TransferArg>,
        ) -> Vec<std::result::Result<uncensored_greats_dao::candid::Nat, String>> {
            match #ident::transfer(args.into_iter().map(|arg| (
                arg.token_id.0.to_u64().unwrap_or(0),
                arg.to.owner,
                arg.memo,
                arg.created_at_time
            )).collect()){
                Ok(m) => {
                    m.into_iter().map(|i| match i{
                        Ok(ii) => Ok(ii.into()),
                        Err(e) => Err(e.to_string())
                    }).collect()
                }
                Err(e) => {
                    uncensored_greats_dao::ic_cdk::trap(&e.to_string());
                }
            }
        }

        #[derive(uncensored_greats_dao::candid::CandidType, Deserialize, Clone)]
        pub struct MintArg {
            pub token_id: uncensored_greats_dao::candid::Nat,
            pub holders: std::collections::HashSet<uncensored_greats_dao::icrc_ledger_types::icrc1::account::Account>,
        }
        #[uncensored_greats_dao::ic_cdk::update]
        pub fn mint(
            args: MintArg,
        ) -> Vec<std::result::Result<uncensored_greats_dao::candid::Nat, String>> {
            match #ident::mint(args.token_id.0.to_u64().unwrap_or(0), args.holders.into_iter().map(|arg| (
                arg.owner
            )).collect()){
                Ok(m) => {
                    m.into_iter().map(|i| match i{
                        Ok(ii) => Ok(ii.into()),
                        Err(e) => Err(e.to_string())
                    }).collect()
                }
                Err(e) => {
                    uncensored_greats_dao::ic_cdk::trap(&e.to_string());
                }
            }
        }

        #[derive(uncensored_greats_dao::candid::CandidType, Deserialize, Clone)]
        pub struct CreateArg {
            pub token: #token_type,
            pub supply_cap: Option<uncensored_greats_dao::candid::Nat>,
        }
        #[uncensored_greats_dao::ic_cdk::update]
        pub fn create_token(
            args: CreateArg
        ) -> uncensored_greats_dao::candid::Nat {
            match #ident::create_token(args.token, match args.supply_cap{
                Some(s) => Some(s.0.to_u64().unwrap_or(0) as usize),
                None => None
            }){
                Ok(m) => m.into(),
                Err(e) => {
                    uncensored_greats_dao::ic_cdk::trap(&e.to_string());
                }
            }
        }

        #[derive(uncensored_greats_dao::candid::CandidType, Deserialize, Clone)]
        pub struct UpdateArg {
            pub token_id: uncensored_greats_dao::candid::Nat,
            pub token: #token_type,
            pub supply_cap: Option<uncensored_greats_dao::candid::Nat>,
        }
        #[uncensored_greats_dao::ic_cdk::update]
        pub fn update_token(
            args: UpdateArg
        ){
            match #ident::update_token(args.token_id.0.to_u64().unwrap_or(0), args.token, match args.supply_cap{
                Some(s) => Some(s.0.to_u64().unwrap_or(0) as usize),
                None => None
            }){
                Ok(m) => m.into(),
                Err(e) => {
                    uncensored_greats_dao::ic_cdk::trap(&e.to_string());
                }
            }
        }

        #[ic_cdk::init]
        pub fn init(){}

        ic_cdk::export_candid!();
    };
    output.into()
}

#[proc_macro_derive(Storage, attributes(icrc7))]
pub fn derive_storage(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;
    let tt = opts.token_type.clone();
    let token_type: proc_macro2::TokenStream = tt.parse().unwrap();
    let output = quote! {
        thread_local! {
            static MEMORY_MANAGER: std::cell::RefCell<uncensored_greats_dao::ic_stable_structures::memory_manager::MemoryManager<uncensored_greats_dao::ic_stable_structures::DefaultMemoryImpl>> =
                std::cell::RefCell::new(uncensored_greats_dao::ic_stable_structures::memory_manager::MemoryManager::init(uncensored_greats_dao::ic_stable_structures::DefaultMemoryImpl::default()));


            static TOKENS: std::cell::RefCell<uncensored_greats_dao::ic_stable_structures::StableBTreeMap<u64, uncensored_greats_dao::TokenInner<#token_type>, uncensored_greats_dao::ic_stable_structures::memory_manager::VirtualMemory<uncensored_greats_dao::ic_stable_structures::DefaultMemoryImpl>>> = std::cell::RefCell::new(
                uncensored_greats_dao::ic_stable_structures::StableBTreeMap::init(
                    MEMORY_MANAGER.with_borrow(|m| m.get(uncensored_greats_dao::ic_stable_structures::memory_manager::MemoryId::new(1)))
                )
            );

            static ASSETS: std::cell::RefCell<uncensored_greats_dao::ic_stable_structures::StableVec<u64, uncensored_greats_dao::ic_stable_structures::memory_manager::VirtualMemory<uncensored_greats_dao::ic_stable_structures::DefaultMemoryImpl>>> = std::cell::RefCell::new(
                uncensored_greats_dao::ic_stable_structures::StableVec::init(
                    MEMORY_MANAGER.with_borrow(|m| m.get(uncensored_greats_dao::ic_stable_structures::memory_manager::MemoryId::new(2))),
                ).expect("failed to init TOKENS store")
            );

            static TRANSACTIONS: std::cell::RefCell<
                    uncensored_greats_dao::ic_stable_structures::StableLog<uncensored_greats_dao::Transaction,
                                                               uncensored_greats_dao::ic_stable_structures::memory_manager::VirtualMemory<uncensored_greats_dao::ic_stable_structures::DefaultMemoryImpl>,
                                                               uncensored_greats_dao::ic_stable_structures::memory_manager::VirtualMemory<uncensored_greats_dao::ic_stable_structures::DefaultMemoryImpl>>> = std::cell::RefCell::new(
                uncensored_greats_dao::ic_stable_structures::StableLog::init(
                    MEMORY_MANAGER.with_borrow(|m| m.get(uncensored_greats_dao::ic_stable_structures::memory_manager::MemoryId::new(3))),
                    MEMORY_MANAGER.with_borrow(|m| m.get(uncensored_greats_dao::ic_stable_structures::memory_manager::MemoryId::new(4))),
                ).expect("failed to init BLOCKS store")
            );
        }

        impl uncensored_greats_dao::ic_stable_structures::Storable for #ident{
            const BOUND: uncensored_greats_dao::ic_stable_structures::storable::Bound = uncensored_greats_dao::ic_stable_structures::storable::Bound::Unbounded;
            fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
                let mut buf = vec![];
                uncensored_greats_dao::ciborium::into_writer(self, &mut buf).expect("failed to encode Collection data");
                std::borrow::Cow::Owned(buf)
            }

            fn from_bytes(bytes: std::borrow::Cow<'_, [u8]>) -> Self {
                uncensored_greats_dao::ciborium::from_reader(&bytes[..]).expect("failed to decode Collection data")
            }
        }

        impl uncensored_greats_dao::Storage<#token_type> for #ident {}

        impl uncensored_greats_dao::Icrc7TokenStorage<#token_type> for #ident {
            fn get_tokens(
            ) -> &'static std::thread::LocalKey<
                    std::cell::RefCell<
                            uncensored_greats_dao::ic_stable_structures::StableBTreeMap<
                                    u64,
                                uncensored_greats_dao::TokenInner<#token_type>,
                                uncensored_greats_dao::ic_stable_structures::memory_manager::VirtualMemory<
                                        uncensored_greats_dao::ic_stable_structures::DefaultMemoryImpl,
                                    >,
                                >,
                        >,
                >{
                &TOKENS
            }
        }

        impl uncensored_greats_dao::Icrc7AssetsStorage for #ident {
            fn check_asset(asset: u64) -> bool{
                ASSETS.with(|r| r.borrow().iter().any(|s| s == asset))
            }
            fn add_asset(asset: u64) -> uncensored_greats_dao::Result<()>{
                Ok(ASSETS.with(|r| r.borrow_mut().push(&asset))?)
            }
        }
        impl uncensored_greats_dao::Icrc7TransactionStorage for #ident {
            fn add_transaction(transaction: uncensored_greats_dao::Transaction) -> uncensored_greats_dao::Result<u64>{
                Ok(TRANSACTIONS.with(|r| r.borrow_mut().append(&transaction)).map_err(|_| uncensored_greats_dao::Error::Custom("failed to wrote log"))? )
            }
       }
    };
    output.into()
}
