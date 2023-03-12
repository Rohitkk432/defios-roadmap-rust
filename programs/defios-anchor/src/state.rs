use anchor_lang::prelude::*;

//to do :make of same type root and leaves
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ObjectiveState {
    Locked, 
    InProgress,
    Closed,
    Deprecated,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ObjectiveDeliverable {
    Infrastructure, 
    Tooling,
    Publication,
    Product,
    Other
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum RoadmapOutlook {
    Next2, 
    Next5,
    Plus5,
    LongTerm
}


#[account]
pub struct RoadMapMetaDataStore {
    pub bump : u8,
    pub roadmap_title: String,
    pub roadmap_creation_unix: u64,
    pub roadmap_creator_id: Pubkey,
    pub roadmap_description_link: String,
    pub number_of_objectives: u64,
    pub root_objective_ids: Vec<Pubkey>,
    pub roadmap_creator: Pubkey,
    pub roadmap_outlook: RoadmapOutlook
}

impl RoadMapMetaDataStore {
    pub fn size() -> usize {
        8 + // discriminator
        1 + //bump
        50 + // roadmap_title
        16 + // roadmap_creation_unix
        32 + //roadmap_creator_id
        8 + // number_of_objectives
        640 + // root_objective_ids
        32 + // roadmap_description_link    
        32 +//roadmap_creator
        1 //roadmap_outlook
    }
}

#[account]
pub struct Objective {
    pub bump :u8,
    pub objective_title : String,
    pub objective_creation_unix: u64,
    pub objective_creator_gh_id: Pubkey,
    pub objective_start_unix : u64,
    pub objective_end_unix : u64,
    pub objective_description_link: String,
    pub objective_state:ObjectiveState,
    pub children_objective_id: Vec<Pubkey>,
    pub objective_deliverable: ObjectiveDeliverable,
    pub objective_staker_ids: Vec<Pubkey>,
    pub objective_staker_amts: Vec<u64>
}

impl Objective {
    pub fn size() -> usize {
        8 + // discriminator
        1 + //bump
        50 + // objective_title
        16 + // objective_creation_unix
        16 + // objective_end_unix
        16 + // objective_start_unix
        640 + // children_objective_id
        32 +  //objective_creator_gh_id
        32 + // objective_description_link 
        1 + //objective_state
        1 + //objective deliverable
        640 + //objective_staker_ids
        160 //objective_staker_amts
    }
}