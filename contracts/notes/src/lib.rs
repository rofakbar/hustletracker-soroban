#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// 1. Struktur Data Task (Kerjaan Lu)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    id: u64,
    client_name: String,
    task_desc: String,
    fee: u64, // Bayaran
}

// 2. Kunci untuk nyimpen data di blockchain
const TASK_DATA: Symbol = symbol_short!("TASK_DATA");

#[contract]
pub struct TaskContract;

#[contractimpl]
impl TaskContract {
    // Fungsi 1: Nambahin kerjaan baru
    pub fn add_task(env: Env, id: u64, client_name: String, task_desc: String, fee: u64) {
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));
        
        let new_task = Task {
            id,
            client_name,
            task_desc,
            fee,
        };
        
        tasks.push_back(new_task);
        env.storage().instance().set(&TASK_DATA, &tasks);
    }

    // Fungsi 2: Ngeliat semua daftar kerjaan
    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env))
    }

    // Fungsi 3: Hapus kerjaan yang udah beres
    pub fn delete_task(env: Env, id: u64) {
        let tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));
        let mut updated_tasks = Vec::new(&env);
        
        for task in tasks.iter() {
            if task.id != id {
                updated_tasks.push_back(task);
            }
        }
        
        env.storage().instance().set(&TASK_DATA, &updated_tasks);
    }
}