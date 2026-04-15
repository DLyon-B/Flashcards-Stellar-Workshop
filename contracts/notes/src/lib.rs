#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur Flashcard
#[contracttype]
#[derive(Clone, Debug)]
pub struct Flashcard {
    id: u64,
    question: String,
    answer: String,
}

// Storage key
const FLASHCARD_DATA: Symbol = symbol_short!("FLASHCARD");

#[contract]
pub struct FlashcardContract;

#[contractimpl]
impl FlashcardContract {

    // 🔍 Ambil semua flashcard
    pub fn get_flashcards(env: Env) -> Vec<Flashcard> {
        env.storage().instance().get(&FLASHCARD_DATA).unwrap_or(Vec::new(&env))
    }

    // ➕ Tambah flashcard
    pub fn create_flashcard(env: Env, question: String, answer: String) -> String {
        let mut cards: Vec<Flashcard> = env.storage().instance()
            .get(&FLASHCARD_DATA)
            .unwrap_or(Vec::new(&env));

        let card = Flashcard {
            id: env.prng().gen::<u64>(),
            question,
            answer,
        };

        cards.push_back(card);
        env.storage().instance().set(&FLASHCARD_DATA, &cards);

        String::from_str(&env, "Flashcard created successfully")
    }

    // ❌ Hapus flashcard
    pub fn delete_flashcard(env: Env, id: u64) -> String {
        let mut cards: Vec<Flashcard> = env.storage().instance()
            .get(&FLASHCARD_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..cards.len() {
            if cards.get(i).unwrap().id == id {
                cards.remove(i);
                env.storage().instance().set(&FLASHCARD_DATA, &cards);
                return String::from_str(&env, "Flashcard deleted");
            }
        }

        String::from_str(&env, "Flashcard not found")
    }

    // 🔎 Search sederhana (exact match)
    pub fn search_flashcard(env: Env, keyword: String) -> Vec<Flashcard> {
        let cards: Vec<Flashcard> = env.storage().instance()
            .get(&FLASHCARD_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for i in 0..cards.len() {
            let card = cards.get(i).unwrap();
            if card.question == keyword {
                result.push_back(card);
            }
        }

        result
    }

    // 🎲 Ambil flashcard random
    pub fn get_random_flashcard(env: Env) -> Option<Flashcard> {
        let cards: Vec<Flashcard> = env.storage().instance()
            .get(&FLASHCARD_DATA)
            .unwrap_or(Vec::new(&env));

        if cards.len() == 0 {
            return None;
        }

        let random_index = (env.prng().gen::<u64>() % cards.len() as u64) as u32;
        Some(cards.get(random_index).unwrap())
    }

    // 💡 Hint sederhana (tidak manipulasi string kompleks)
    pub fn get_hint(env: Env, id: u64) -> String {
        let cards: Vec<Flashcard> = env.storage().instance()
            .get(&FLASHCARD_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..cards.len() {
            let card = cards.get(i).unwrap();
            if card.id == id {
                return String::from_str(&env, "Hint: Think about the main concept!");
            }
        }

        String::from_str(&env, "Flashcard not found")
    }
}