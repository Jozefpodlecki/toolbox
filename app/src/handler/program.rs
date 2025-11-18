use tauri::{command, State};
use crate::{models::{GetProcessArgs, GetProgramArgs, PageArgs, Paged, Process, ProcessResult, Program}, services::InstalledProgramsService};

use super::error::*;

#[command]
pub fn get_programs_count(programs_service: State<InstalledProgramsService>) -> Result<u32> {
  
    let count = programs_service.get_count()?;

    Ok(count)
}

#[command]
pub fn get_programs(programs_service: State<InstalledProgramsService>, args: GetProgramArgs) -> Result<Paged<Program>> {
  
    let GetProgramArgs {
        name,
        page: PageArgs {
            page,
            page_size
        },
    } = args;

    let mut all = programs_service.get_all()?;

    if let Some(name) = name {
        all = all.into_iter().filter(|pr| pr.name.to_lowercase().contains(&name.to_lowercase())).collect();
    }

    let total = all.len() as u32;
    let start = (page * page_size) as usize;
    let end = (start + page_size as usize).min(all.len());

    let items = if start < all.len() {
        all[start..end].to_vec()
    } else {
        Vec::new()
    };

    let page_size = ((all.len() as f32) / (page_size as f32)).ceil() as u32;

    Ok(Paged {
        items,
        page,
        page_size,
        total
    })
}