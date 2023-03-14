# Roadmap Programs

Every open source community, needs to make sure members are aware about the long-term and short-term prospects of the company. Traditionally, the project owners used to __create an openly accessible trello board__. But this approach has 2 major limitations:
* The community contributors do not feel a sense of ownership in the project as they are unable to contribute to the direction of the project in a permissionless manner.
* There is no way for the community to signal economic value of dependencies to new contributors. So it becomes hard for newcomers to focus their energy in sync with the rest of the contributors.

## Design Choices

### DAG Style Task Representation

As opposed to the standard **Kanban style** task boards or roadmaps, DAGs are more suited to dynamic and responsive community collaboration. This is because of a few reasons:

* Each objective is modular and its internal metadata can be implmented with arbitrary complexity. 

* It is very easy to visualize dependencies and blockers for a particular task since the directed graph representation makes parent-child relationships super clear.

* An objective can be shared across multiple roadmaps making it easy for different communities to align interests behind a single checkpoint.

* It is very easy to view change in direction within the roadmap by simply querying all the objectives marked as deprecated and looking at them along with their child objectives.

### Staking On Roadmap Objectives

In order to build a robust and collaborative community, it is important the people can signal the economic benefits of each improvement and update amongst each other. To do so in a seamless manner, the roadmap needs to be implemented on-chain. Having the roadmap on-chain enables:

* Staking tokens as rewards behind the completion of each objective in the roadmap to incentivize development in that direction.

* Represent parent-child relationships in an immutable manner to force longer deliberations before pursuing a course of action or moving away from a course of action, because the cascading effects of these decisions are immediately visible.


## Architecture

The main tech stack for the creation of this roadmap builder was Anchor, Solana, and Rust.

The contract is quite simple and has only 4 entry points:

* add_roadmap_data: Used for creating a new roadmap PDA
* add_objective_data: Used for creating a new objective PDA
* add_child_objective: Used to link an objective to either a root roadmap or another objective as its parent
* send_funds: Used to stake on an objective

The two type of PDA's defined within the smart contract are:
1. RoadMapMetaDataStore(For Roadmap Data)
```
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

```
2. Objective (For objective Data)
```
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
```


    

## Product SS
Here are a few images illustrating the working of the Roadmap on https://defi-os.com

![Browse Different Roadmaps](https://raw.githubusercontent.com/tanmaymunjal/priyesh_image_repo/main/priyesh.jpg)
![Go through the nodes and their relationships](https://raw.githubusercontent.com/tanmaymunjal/priyesh_image_repo/main/priyesh2.png)
![Analyse Node Details](https://raw.githubusercontent.com/tanmaymunjal/priyesh_image_repo/main/priyesh3.png)
---
#### How to Contribute?

The work in the project falls into 4 major areas:
1. Working on bug reports and security
3. Further improvements and feature addition in the contracts with respect to the roadmap for the project
4. Working on connectors and tests for the smart contract
5. Working on the documentation of the code

Newcomers may feel free to look into and contribute to any part of the project.
---
#### Guide for newcomers

1) Fork the repo
2) Set up the repo as a folder on your local machine or a remote server to experiment with it
3) Setup all major dependencies related to it
(Anchor installation: https://www.anchor-lang.com/docs/installation,
Anchor-spl: https://crates.io/crates/anchor-spl)
4) Build the project on your local using anchor build or test it using anchor test
6) Interact with the contract, get a goot idea of how it works, and try to contribute in whatever you see fit to solve open issues and/or add features
7) Add tests for any changes you made
8) Commit any changes you made in your git repo and start a pull request to the main branch
---
#### Code Formatting

The project uses the default rustfmt cli command with no additional customisations to format the code, please format any pull requests you make in the same formatting style.

---
#### Issue Reporting
 
Please mention all steps to reproduce the issue, link any custom code that led to the issue, and mention any CLI errors you got within the issue itself.
