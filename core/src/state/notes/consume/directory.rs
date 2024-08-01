use crate::{
    data::Directory,
    db::Db,
    state::notes::{DirectoryItem, DirectoryItemChildren, InnerState, NotesState, SelectedItem},
    types::DirectoryId,
    Error, Result, Transition,
};

pub async fn open(
    db: &mut Db,
    state: &mut NotesState,
    directory_id: DirectoryId,
) -> Result<Transition> {
    let item = state
        .root
        .find_mut(&directory_id)
        .ok_or(Error::Wip("todo: asdfasdf".to_owned()))?;

    if item.children.is_none() {
        let notes = db.fetch_notes(directory_id.clone()).await?;
        let directories = db
            .fetch_directories(directory_id.clone())
            .await?
            .into_iter()
            .map(|directory| DirectoryItem {
                directory,
                children: None,
            })
            .collect();

        item.children = Some(DirectoryItemChildren { notes, directories });
    }

    let (notes, directories) = match &mut item.children {
        Some(children) => (&children.notes, &children.directories),
        None => {
            panic!("...?");
        }
    };

    Ok(Transition::OpenDirectory {
        id: directory_id,
        notes: notes.clone(),
        directories: directories.clone(),
    })
}

pub fn close(state: &mut NotesState, directory_id: DirectoryId) -> Result<Transition> {
    state
        .root
        .find_mut(&directory_id)
        .ok_or(Error::Wip("todo: asdfasdf".to_owned()))?
        .children = None;

    Ok(Transition::CloseDirectory {
        directory_id: directory_id.clone(),
        by_note: false,
    })
}

pub fn close_by_note(state: &mut NotesState, directory: Directory) -> Result<Transition> {
    close(state, directory.id.clone())?;

    let directory_id = directory.id.clone();

    state.selected = SelectedItem::Directory(directory);
    state.inner_state = InnerState::DirectorySelected;

    Ok(Transition::CloseDirectory {
        directory_id,
        by_note: true,
    })
}

pub fn show_actions_dialog(state: &mut NotesState, directory: Directory) -> Result<Transition> {
    state.selected = SelectedItem::Directory(directory.clone());
    state.inner_state = InnerState::DirectoryMoreActions;

    Ok(Transition::ShowDirectoryActionsDialog(directory))
}

pub fn select(state: &mut NotesState, directory: Directory) -> Result<Transition> {
    state.selected = SelectedItem::Directory(directory);
    state.inner_state = InnerState::DirectorySelected;

    Ok(Transition::None)
}

pub async fn rename(
    db: &mut Db,
    state: &mut NotesState,
    mut directory: Directory,
    new_name: String,
) -> Result<Transition> {
    db.rename_directory(directory.id.clone(), new_name.clone())
        .await?;

    directory.name = new_name;
    state.inner_state = InnerState::DirectorySelected;

    Ok(Transition::RenameDirectory(directory))
}

pub async fn remove(
    db: &mut Db,
    state: &mut NotesState,
    directory: Directory,
) -> Result<Transition> {
    db.remove_directory(directory.id.clone()).await?;

    // TODO
    state.selected = SelectedItem::None;

    Ok(Transition::RemoveDirectory(directory))
}

pub async fn add(
    db: &mut Db,
    state: &mut NotesState,
    directory: Directory,
    directory_name: String,
) -> Result<Transition> {
    let parent_id = directory.id.clone();
    let directory = db.add_directory(parent_id.clone(), directory_name).await?;

    let item = state
        .root
        .find_mut(&parent_id)
        .ok_or(Error::Wip("todo: failed to find {parent_id}".to_owned()))?;

    if let DirectoryItem {
        children: Some(ref mut children),
        ..
    } = item
    {
        let directories = db
            .fetch_directories(parent_id)
            .await?
            .into_iter()
            .map(|directory| DirectoryItem {
                directory,
                children: None,
            })
            .collect();

        children.directories = directories;
    }

    state.selected = SelectedItem::Directory(directory.clone());
    state.inner_state = InnerState::DirectorySelected;

    Ok(Transition::AddDirectory(directory))
}