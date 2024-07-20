pub mod more_actions;

use {
    crate::{traits::*, Node},
    cursive::{
        event::EventResult,
        view::Nameable,
        views::{Button, FocusTracker, LinearLayout, TextView},
        View, With,
    },
    glues_core::{data::Note, Event},
    std::rc::Rc,
};

pub fn render_note(note: Note) -> impl View {
    let note_node = Node::note_tree().note(&note.id);
    let button = Button::new_raw(note.name.clone(), |siv| {
        siv.dispatch(Event::OpenNote);
    })
    .with_name(note_node.name_button().name());

    LinearLayout::horizontal()
        .child(TextView::new("◦ "))
        .child(button)
        .wrap_with(FocusTracker::new)
        .on_focus(on_item_focus(note.clone()))
        .with_name(note_node.name())
}

fn on_item_focus(note: Note) -> impl for<'a> Fn(&'a mut LinearLayout) -> EventResult {
    let note = Rc::new(note);

    move |_| {
        let note = Rc::clone(&note);

        EventResult::with_cb(move |siv| {
            let note = note.as_ref().clone();

            siv.dispatch(Event::SelectNote(note));
        })
    }
}
