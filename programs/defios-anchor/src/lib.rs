use anchor_lang::prelude::*;
use instructions::*;
use crate::state::{ObjectiveState, ObjectiveDeliverable, RoadmapOutlook};

pub mod error;
pub mod instructions;
pub mod state;

declare_id!("2jBpaAC79KdXVL6TaYkRRGzADBrc8nVPN1K2pfbqgfV9");

#[program]
pub mod roadmaps {
    use super::*;

    pub fn add_roadmap_data(
        ctx: Context<AddMetadata>,
        roadmap_title: String,
        roadmap_description_link:String,
        roadmap_outlook:RoadmapOutlook
    ) -> Result<()> {
        add_roadmap_data::handler(ctx, roadmap_title, roadmap_description_link, roadmap_outlook)
    }

    pub fn send_funds(
        ctx: Context<StakeObjective>,
        transfer_amount: u64
    ) -> Result<()> {
        send_funds::handler(ctx, transfer_amount)
    }

    pub fn add_objective_data(
        ctx: Context<AddObjective>, 
        objective_title: String,
        objective_start_unix:u64,
        objective_end_unix:u64,
        objective_description_link:String,
        objective_state:ObjectiveState,
        objective_deliverable:ObjectiveDeliverable,
    ) -> Result<()> {
        add_objective_data::handler(ctx, objective_title,objective_start_unix,objective_end_unix,objective_description_link,objective_state,objective_deliverable)
    }
    pub fn add_child_objective(
        ctx: Context<AddChildObjective>, 
        from_root:bool
    ) -> Result<()> {
        add_child_objective::handler(ctx,from_root)
    }

}