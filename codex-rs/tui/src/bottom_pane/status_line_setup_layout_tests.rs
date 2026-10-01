use super::*;
use crossterm::event::KeyCode;
use pretty_assertions::assert_eq;
use tokio::sync::mpsc::unbounded_channel;

#[test]
fn repeated_layout_entries_survive_open_and_save_in_order() {
    let expected = vec![
        StatusLineItem::ModelName,
        StatusLineItem::Space,
        StatusLineItem::Space,
        StatusLineItem::Reasoning,
        StatusLineItem::LineBreak,
        StatusLineItem::CurrentDir,
        StatusLineItem::LineBreak,
        StatusLineItem::GitBranch,
    ];
    let configured = expected.iter().map(ToString::to_string).collect::<Vec<_>>();
    let (tx, mut rx) = unbounded_channel();
    let mut view = StatusLineSetupView::new(
        Some(&configured),
        /*use_theme_colors*/ false,
        StatusSurfacePreviewData::default(),
        AppEventSender::new(tx),
        crate::keymap::RuntimeKeymap::defaults().list,
    );

    view.handle_key_event(KeyCode::Enter.into());

    let AppEvent::StatusLineSetup {
        items,
        use_theme_colors,
    } = rx.try_recv().unwrap()
    else {
        panic!("expected status line settings");
    };
    assert_eq!((items, use_theme_colors), (expected, false));
}

#[test]
fn additional_layout_entries_can_be_selected_without_reopening() {
    let (tx, mut rx) = unbounded_channel();
    let mut view = StatusLineSetupView::new(
        Some(&["model".to_string()]),
        /*use_theme_colors*/ true,
        StatusSurfacePreviewData::default(),
        AppEventSender::new(tx),
        crate::keymap::RuntimeKeymap::defaults().list,
    );
    for query in ["line-break", "space"] {
        for ch in query.chars() {
            view.handle_key_event(KeyCode::Char(ch).into());
        }
        for index in 0..2 {
            view.handle_key_event(KeyCode::Home.into());
            for _ in 0..index {
                view.handle_key_event(KeyCode::Down.into());
            }
            view.handle_key_event(KeyCode::Char(' ').into());
        }
        for _ in query.chars() {
            view.handle_key_event(KeyCode::Backspace.into());
        }
    }
    view.handle_key_event(KeyCode::Enter.into());

    let AppEvent::StatusLineSetup {
        items,
        use_theme_colors,
    } = rx.try_recv().unwrap()
    else {
        panic!("expected status line settings");
    };
    assert_eq!(
        (items, use_theme_colors),
        (
            vec![
                StatusLineItem::ModelName,
                StatusLineItem::LineBreak,
                StatusLineItem::Space,
                StatusLineItem::LineBreak,
                StatusLineItem::Space,
            ],
            true,
        )
    );
}
