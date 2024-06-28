use {
    super::render_directory,
    crate::{traits::*, views::note_tree::note::render_note, Node},
    cursive::{
        view::Nameable,
        views::{LinearLayout, PaddedView},
        Cursive, View,
    },
    glues_core::{state::note_tree::DirectoryItem, types::DirectoryId},
};

pub fn render_item_list(siv: &mut Cursive, directory_id: DirectoryId) -> impl View {
    let directories = siv
        .glues()
        .db
        .fetch_directories(directory_id.clone())
        .log_unwrap();
    let notes = siv
        .glues()
        .db
        .fetch_notes(directory_id.clone())
        .log_unwrap();
    let mut layout = LinearLayout::vertical();

    for child in directories {
        let directory_item = DirectoryItem {
            directory: child,
            children: None,
        };

        layout.add_child(render_directory(siv, directory_item));
    }

    for child in notes {
        layout.add_child(render_note(child));
    }

    let layout = layout.with_name(
        Node::note_tree()
            .directory(&directory_id)
            .note_list()
            .name(),
    );

    PaddedView::lrtb(1, 0, 0, 0, layout)
}
