use std::cell::RefCell;

use candid::{candid_method, CandidType, Decode, Encode};

use ic_cdk::{init, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};

// A memory for upgrades, where data from the heap can be serialized/deserialized.
const UPGRADES: MemoryId = MemoryId::new(0);

// A memory for the StableBTreeMap we're using. A new memory should be created for
// every additional stable structure.
const STABLE_CHUNK_BTREE: MemoryId = MemoryId::new(1);

pub type StableMemory = VirtualMemory<DefaultMemoryImpl>;

pub fn get_upgrades_memory() -> StableMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES))
}

pub fn get_stable_btree_memory() -> StableMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STABLE_CHUNK_BTREE))
}

pub fn init_chunk_stable_data() -> StableBTreeMap<u128, User, StableMemory> {
    StableBTreeMap::init(get_stable_btree_memory())
}

#[derive(CandidType, candid::Deserialize, Debug)]
pub struct User {
    name: String,
    fav_numbers: Vec<Vec<u8>>,
}

impl Storable for User {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(&self).unwrap())
    }
}

impl BoundedStorable for User {
    const IS_FIXED_SIZE: bool = false;
    const MAX_SIZE: u32 = u32::MAX;
}

pub struct State {
    users: ic_stable_structures::StableBTreeMap<u128, User, StableMemory>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            users: init_chunk_stable_data(),
        }
    }
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

#[init]
#[candid_method(init)]
pub fn init() {}

#[update]
#[candid_method(update)]
pub fn insert() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        for id in 0..10 {
            let mut user = User {
                name: format!("user with {}", id),
                fav_numbers: vec![],
            };
            ic_cdk::println!("{}", id);
            for inner_id in 0..=id {
                let nums = [1; 2000].to_vec();
                user.fav_numbers.push(nums);
                ic_cdk::println!("{}", inner_id);
            }
            state.users.insert(id, user);
        }
    })
}

#[query]
#[candid_method(query)]
pub fn get(id: u128) -> bool {
    STATE.with(|state| {
        let state = state.borrow();
        match state.users.get(&id) {
            None => false,
            Some(user) => {
                ic_cdk::println!("{:?}", user);
                true
            }
        }
    })
}
