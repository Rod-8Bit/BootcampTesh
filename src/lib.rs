#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, BytesN, Env,
    String,
};

const TTL_THRESHOLD: u32 = 100_000;
const TTL_EXTEND: u32 = 3_000_000;

#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Status {
    Pending = 0,
    Accepted = 1,
    Cancelled = 2,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agreement {
    pub id: u32,
    pub creator: Address,
    pub counterparty: Address,
    pub title: String,
    /// SHA-256 de los términos completos (el texto vive fuera de la cadena).
    pub terms_hash: BytesN<32>,
    pub status: Status,
    pub created_at: u64,
    pub accepted_at: u64,
}

#[contracttype]
pub enum DataKey {
    Count,
    Agreement(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotFound = 1,
    NotPending = 2,
    SameParty = 3,
    Unauthorized = 4,
}

#[contract]
pub struct AcuerdoClaro;

#[contractimpl]
impl AcuerdoClaro {
    /// El creador propone un acuerdo a una contraparte. Devuelve el id.
    pub fn create(
        env: Env,
        creator: Address,
        counterparty: Address,
        title: String,
        terms_hash: BytesN<32>,
    ) -> Result<u32, Error> {
        creator.require_auth();
        if creator == counterparty {
            return Err(Error::SameParty);
        }

        let id: u32 = env.storage().instance().get(&DataKey::Count).unwrap_or(0) + 1;
        env.storage().instance().set(&DataKey::Count, &id);
        env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND);

        let agreement = Agreement {
            id,
            creator: creator.clone(),
            counterparty: counterparty.clone(),
            title,
            terms_hash,
            status: Status::Pending,
            created_at: env.ledger().timestamp(),
            accepted_at: 0,
        };
        Self::save(&env, &agreement);

        env.events()
            .publish((symbol_short!("created"), id), (creator, counterparty));
        Ok(id)
    }

    /// La contraparte acepta desde su wallet; queda sellado el momento exacto.
    pub fn accept(env: Env, counterparty: Address, id: u32) -> Result<(), Error> {
        counterparty.require_auth();
        let mut a = Self::load(&env, id)?;
        if a.counterparty != counterparty {
            return Err(Error::Unauthorized);
        }
        if a.status != Status::Pending {
            return Err(Error::NotPending);
        }
        a.status = Status::Accepted;
        a.accepted_at = env.ledger().timestamp();
        Self::save(&env, &a);

        env.events().publish((symbol_short!("accepted"), id), counterparty);
        Ok(())
    }

    /// El creador puede cancelar mientras la contraparte no haya aceptado.
    pub fn cancel(env: Env, creator: Address, id: u32) -> Result<(), Error> {
        creator.require_auth();
        let mut a = Self::load(&env, id)?;
        if a.creator != creator {
            return Err(Error::Unauthorized);
        }
        if a.status != Status::Pending {
            return Err(Error::NotPending);
        }
        a.status = Status::Cancelled;
        Self::save(&env, &a);

        env.events().publish((symbol_short!("cancelled"), id), creator);
        Ok(())
    }

    pub fn get(env: Env, id: u32) -> Result<Agreement, Error> {
        Self::load(&env, id)
    }

    /// Verifica que un documento (por su hash) corresponde al acuerdo.
    pub fn verify(env: Env, id: u32, terms_hash: BytesN<32>) -> Result<bool, Error> {
        Ok(Self::load(&env, id)?.terms_hash == terms_hash)
    }

    pub fn count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Count).unwrap_or(0)
    }

    fn load(env: &Env, id: u32) -> Result<Agreement, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Agreement(id))
            .ok_or(Error::NotFound)
    }

    fn save(env: &Env, a: &Agreement) {
        let key = DataKey::Agreement(a.id);
        env.storage().persistent().set(&key, a);
        env.storage().persistent().extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND);
    }
}

#[cfg(test)]
mod test;
