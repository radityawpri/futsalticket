#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec
};

// Storage key untuk data tiket (max 9 karakter)
const TKT_DATA: Symbol = symbol_short!("TKT_DATA");

// Struktur data Tiket
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ticket {
    pub id: u64,
    pub home_team: String,
    pub away_team: String,
    pub match_date: String,
    pub price: u64, // harga dalam satuan terkecil
}

#[contract]
pub struct FutsalTicketContract;

#[contractimpl]
impl FutsalTicketContract {

    // Ambil semua tiket
    pub fn get_tickets(env: Env) -> Vec<Ticket> {
        env.storage()
            .instance()
            .get(&TKT_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah tiket baru
    pub fn create_ticket(
        env: Env,
        home_team: String,
        away_team: String,
        match_date: String,
        price: u64
    ) -> String {
        let mut tickets: Vec<Ticket> = env.storage()
            .instance()
            .get(&TKT_DATA)
            .unwrap_or(Vec::new(&env));

        let ticket = Ticket {
            id: env.prng().gen::<u64>(),
            home_team,
            away_team,
            match_date,
            price,
        };

        tickets.push_back(ticket);

        env.storage().instance().set(&TKT_DATA, &tickets);

        String::from_str(&env, "Tiket berhasil ditambahkan")
    }

    // Hapus tiket berdasarkan ID
    pub fn delete_ticket(env: Env, id: u64) -> String {
        let tickets: Vec<Ticket> = env.storage()
            .instance()
            .get(&TKT_DATA)
            .unwrap_or(Vec::new(&env));

        let mut new_tickets = Vec::new(&env);
        let mut found = false;

        for ticket in tickets.iter() {
            if ticket.id != id {
                new_tickets.push_back(ticket);
            } else {
                found = true;
            }
        }

        if found {
            env.storage().instance().set(&TKT_DATA, &new_tickets);
            String::from_str(&env, "Tiket berhasil dihapus")
        } else {
            String::from_str(&env, "Tiket tidak ditemukan")
        }
    }
}