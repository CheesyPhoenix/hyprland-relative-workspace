use hyprland::data::{Monitors, Workspaces};
use hyprland::shared::HyprData;
use std::{env::args, process::exit};

fn main() -> hyprland::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() < 2
        || !["f", "forward", "b", "backward"]
            .map(|x| x.to_string())
            .contains(&args[1])
    {
        eprintln!("Incorrect command usage! \nCorrect usage: '{} [\"f\", \"forward\", \"b\", \"backward\"] (opt. \"--with-window\")'", args[0]);
        exit(1);
    }

    change_workspace(
        args.contains(&"--with-window".into()),
        args[1] == "b" || args[1] == "backward",
    )?;

    Ok(())
}

fn change_workspace(with_window: bool, backward: bool) -> hyprland::Result<()> {
    let monitor = Monitors::get()?
        .find(|x| x.focused)
        .expect("There should always be a focused monitor");

    let active_workspace = &monitor.active_workspace;

    let workspaces = Workspaces::get()?;

    let workspaces_of_monitor = workspaces.clone().filter(|x| x.monitor == monitor.name);

    // if workspace exits
    if (!backward
        && workspaces_of_monitor
            .clone()
            .max_by_key(|x| x.id)
            .expect("There should always be at least one workspace")
            .id
            > active_workspace.id)
        || (backward
            && workspaces_of_monitor
                .clone()
                .min_by_key(|x| x.id)
                .expect("There should always be at least one workspace")
                .id
                < active_workspace.id)
    {
        exec_change_workspace(if backward { -1 } else { 1 }, true, with_window)?;
    } else {
        // workspace doesn't already exist, therefore create new

        if !backward {
            // exit if current workspace is empty
            let workpace = workspaces_of_monitor
                .clone()
                .find(|x| x.id == active_workspace.id)
                .expect("Should be able to find current workspace");
            if workpace.windows == 0 {
                println!("Did nothing; already on an empty workspace");
                return Ok(());
            }

            let highest_workspace_id = workspaces
                .max_by_key(|x| x.id)
                .expect("There should always be at least one workspace")
                .id;

            exec_change_workspace(highest_workspace_id + 1, false, with_window)?;
        } else {
            let mut target_workspace_id = active_workspace.id;

            let occupied_ids: Vec<i32> = workspaces.map(|x| x.id).collect();

            while occupied_ids.contains(&target_workspace_id) {
                target_workspace_id -= 1;
            }

            if target_workspace_id < 1 {
                println!("Did nothing; cannot go lower than workspace 1");
                return Ok(());
            }

            exec_change_workspace(target_workspace_id, false, with_window)?;
        }
    }

    Ok(())
}

fn exec_change_workspace(to: i32, relative: bool, with_window: bool) -> hyprland::Result<()> {
    if with_window {
        if relative {
            hyprland::dispatch::Dispatch::call(hyprland::dispatch::DispatchType::MoveToWorkspace(
                hyprland::dispatch::WorkspaceIdentifier::RelativeMonitor(to),
                Option::None,
            ))?;
        } else {
            hyprland::dispatch::Dispatch::call(hyprland::dispatch::DispatchType::MoveToWorkspace(
                hyprland::dispatch::WorkspaceIdentifier::Id(to),
                Option::None,
            ))?;
        }
    } else {
        if relative {
            hyprland::dispatch::Dispatch::call(hyprland::dispatch::DispatchType::Workspace(
                hyprland::dispatch::WorkspaceIdentifierWithSpecial::RelativeMonitor(to),
            ))?;
        } else {
            hyprland::dispatch::Dispatch::call(hyprland::dispatch::DispatchType::Workspace(
                hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(to),
            ))?;
        }
    }

    Ok(())
}
