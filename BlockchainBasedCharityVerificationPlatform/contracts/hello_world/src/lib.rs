#![allow(non_snake_case)]
#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Structure to track the status of charity projects
#[contracttype]
#[derive(Clone)]
pub struct CharityStatus {
    pub registered: u64,  // Count of registered charities
    pub verified: u64,    // Count of verified charities
    pub total_donations: u64, // Total donation amount
    pub total_projects: u64  // Total charity projects on platform
}

// Reference for the CharityStatus struct
const ALL_PROJECTS: Symbol = symbol_short!("ALL_PROJ");

// Mapping project_id to respective Charity struct
#[contracttype]
pub enum CharityBook {
    Project(u64)
}

// Charity structure to track the project details
#[contracttype]
#[derive(Clone)]
pub struct Charity {
    pub project_id: u64,      // Unique project ID
    pub title: String,        // Project title
    pub description: String,  // Project description
    pub registered_time: u64, // Time of project registration
    pub verified: bool,       // Whether the project is verified by platform
    pub funds_received: u64,  // Total funds received
}

// For generating unique project IDs
const COUNT_PROJECTS: Symbol = symbol_short!("C_PROJ");

#[contract]
pub struct CharityVerificationContract;

#[contractimpl]
impl CharityVerificationContract {

    // Function to register a new charity project
    pub fn register_project(env: Env, title: String, description: String) -> u64 {
        // Generate a new unique project ID
        let mut project_count: u64 = env.storage().instance().get(&COUNT_PROJECTS).unwrap_or(0);
        project_count += 1;

        let mut charity_status = Self::view_all_projects(env.clone());

        let time = env.ledger().timestamp(); // Get the current ledger timestamp

        let new_project = Charity {
            project_id: project_count.clone(),
            title: title.clone(),
            description: description.clone(),
            registered_time: time,
            verified: false,
            funds_received: 0,
        };

        // Update the total project count
        charity_status.total_projects += 1;
        charity_status.registered += 1;

        // Store the project information
        env.storage().instance().set(&CharityBook::Project(project_count), &new_project);
        env.storage().instance().set(&ALL_PROJECTS, &charity_status);
        env.storage().instance().set(&COUNT_PROJECTS, &project_count);

        log!(&env, "Project Registered with ID: {}", project_count);

        project_count
    }

    // Function for an admin to verify a project
    pub fn verify_project(env: Env, project_id: u64) {
        let mut project = Self::view_project(env.clone(), project_id.clone());

        if project.verified == false {
            project.verified = true;

            let mut charity_status = Self::view_all_projects(env.clone());
            charity_status.verified += 1; // Increment the verified count

            // Store the updated project
            env.storage().instance().set(&CharityBook::Project(project_id), &project);
            env.storage().instance().set(&ALL_PROJECTS, &charity_status);

            log!(&env, "Project ID: {} has been verified", project_id);
        } else {
            log!(&env, "Project ID: {} is already verified", project_id);
            panic!("Project is already verified");
        }
    }

    // Function to donate to a verified project
    pub fn donate(env: Env, project_id: u64, amount: u64) {
        let mut project = Self::view_project(env.clone(), project_id.clone());

        // Ensure that the project is verified before allowing donations
        if project.verified == true {
            project.funds_received += amount;

            let mut charity_status = Self::view_all_projects(env.clone());
            charity_status.total_donations += amount;

            // Update the project with the new donation amount
            env.storage().instance().set(&CharityBook::Project(project_id), &project);
            env.storage().instance().set(&ALL_PROJECTS, &charity_status);

            log!(&env, "Donated {} to Project ID: {}", amount, project_id);
        } else {
            log!(&env, "Cannot donate! Project ID: {} is not verified.", project_id);
            panic!("Cannot donate to unverified project");
        }
    }

    // View function for an admin to see the overall charity status
    pub fn view_all_projects(env: Env) -> CharityStatus {
        env.storage().instance().get(&ALL_PROJECTS).unwrap_or(CharityStatus {
            registered: 0,
            verified: 0,
            total_donations: 0,
            total_projects: 0,
        })
    }

    // View function to retrieve a project’s details
    pub fn view_project(env: Env, project_id: u64) -> Charity {
        let key = CharityBook::Project(project_id.clone());
        env.storage().instance().get(&key).unwrap_or(Charity {
            project_id: 0,
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
            registered_time: 0,
            verified: false,
            funds_received: 0,
        })
    }
}
