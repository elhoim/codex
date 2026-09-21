use super::*;
use crate::app_event::AppEvent;
use pretty_assertions::assert_eq;
use tokio::sync::mpsc::unbounded_channel;

#[test]
fn multiline_preview_reserves_rows_and_clips_without_hiding_hints() {
    let (tx, _rx) = unbounded_channel::<AppEvent>();
    let mut picker = MultiSelectPicker::builder(
        "Status line".to_string(),
        /*subtitle*/ None,
        AppEventSender::new(tx),
    )
    .items(vec![MultiSelectItem {
        id: "model".to_string(),
        name: "Model".to_string(),
        enabled: true,
        ..Default::default()
    }])
    .on_preview(|_| {
        Some(Text::from(
            "gpt-5  medium\n~/project/a/very/long/directory/path/that/overflows\nmain",
        ))
    })
    .build();
    let width = 48;
    let height = picker.desired_height(width);
    let preview = picker.preview_line.take();
    assert_eq!(height, picker.desired_height(width) + 3);
    let mut snapshots = Vec::new();
    let tall_preview = Text::from(
        (0..30)
            .map(|row| Line::from(format!("preview row {row}")))
            .collect::<Vec<_>>(),
    );
    for (height, preview) in [
        (height, preview.clone()),
        (2, preview.clone()),
        (1, preview),
        (12, Some(tall_preview)),
    ] {
        picker.preview_line = preview;
        let area = Rect::new(/*x*/ 0, /*y*/ 0, width, height);
        let mut buf = Buffer::empty(area);
        picker.render(area, &mut buf);
        let text = buf
            .content
            .chunks(usize::from(width))
            .map(|row| {
                row.iter()
                    .map(ratatui::buffer::Cell::symbol)
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            text.ends_with("space toggle · enter save · esc cancel"),
            "{text}"
        );
        if height >= 5 {
            assert!(text.contains("Status line\n"), "{text}");
            assert!(text.contains("Type to search"), "{text}");
            assert!(text.contains("› [x] Model"), "{text}");
        }
        snapshots.push(format!("height={height}\n{text}"));
    }
    insta::assert_snapshot!(snapshots.join("\n\n"));
}
