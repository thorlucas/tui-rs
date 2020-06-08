use tui::backend::TestBackend;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Margin};
use tui::widgets::{Block, Borders, Row, Table};
use tui::Terminal;

#[test]
fn widgets_table_column_spacing_can_be_changed() {
    let test_case = |column_spacing, expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Length(5),
                ])
                .column_spacing(column_spacing);
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    // no space between columns
    test_case(
        0,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1Head2Head3             │",
            "│                            │",
            "│Row11Row12Row13             │",
            "│Row21Row22Row23             │",
            "│Row31Row32Row33             │",
            "│Row41Row42Row43             │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // one space between columns
    test_case(
        1,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1 Head2 Head3           │",
            "│                            │",
            "│Row11 Row12 Row13           │",
            "│Row21 Row22 Row23           │",
            "│Row31 Row32 Row33           │",
            "│Row41 Row42 Row43           │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // enough space to just not hide the third column
    test_case(
        6,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1      Head2      Head3 │",
            "│                            │",
            "│Row11      Row12      Row13 │",
            "│Row21      Row22      Row23 │",
            "│Row31      Row32      Row33 │",
            "│Row41      Row42      Row43 │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // enough space to hide part of the third column
    test_case(
        7,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1       Head2       Head│",
            "│                            │",
            "│Row11       Row12       Row1│",
            "│Row21       Row22       Row2│",
            "│Row31       Row32       Row3│",
            "│Row41       Row42       Row4│",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );
}

#[test]
fn widgets_table_columns_widths_can_use_fixed_length_constraints() {
    let test_case = |widths, expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(widths);
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    // columns of zero width show nothing
    test_case(
        &[
            Constraint::Length(0),
            Constraint::Length(0),
            Constraint::Length(0),
        ],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // columns of 1 width trim
    test_case(
        &[
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│H H H                       │",
            "│                            │",
            "│R R R                       │",
            "│R R R                       │",
            "│R R R                       │",
            "│R R R                       │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // columns of large width just before pushing a column off
    test_case(
        &[
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
        ],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1    Head2    Head3     │",
            "│                            │",
            "│Row11    Row12    Row13     │",
            "│Row21    Row22    Row23     │",
            "│Row31    Row32    Row33     │",
            "│Row41    Row42    Row43     │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );
}

#[test]
fn widgets_table_columns_widths_can_use_percentage_constraints() {
    let test_case = |widths, expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(widths)
                .column_spacing(0);
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    // columns of zero width show nothing
    test_case(
        &[
            Constraint::Percentage(0),
            Constraint::Percentage(0),
            Constraint::Percentage(0),
        ],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // columns of not enough width trims the data
    test_case(
        &[
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│HeaHeaHea                   │",
            "│                            │",
            "│RowRowRow                   │",
            "│RowRowRow                   │",
            "│RowRowRow                   │",
            "│RowRowRow                   │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // columns of large width just before pushing a column off
    test_case(
        &[
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1    Head2    Head3     │",
            "│                            │",
            "│Row11    Row12    Row13     │",
            "│Row21    Row22    Row23     │",
            "│Row31    Row32    Row33     │",
            "│Row41    Row42    Row43     │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // percentages summing to 100 should give equal widths
    test_case(
        &[Constraint::Percentage(50), Constraint::Percentage(50)],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1          Head2        │",
            "│                            │",
            "│Row11          Row12        │",
            "│Row21          Row22        │",
            "│Row31          Row32        │",
            "│Row41          Row42        │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );
}

#[test]
fn widgets_table_columns_widths_can_use_mixed_constraints() {
    let test_case = |widths, expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(widths);
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    // columns of zero width show nothing
    test_case(
        &[
            Constraint::Percentage(0),
            Constraint::Length(0),
            Constraint::Percentage(0),
        ],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // columns of not enough width trims the data
    test_case(
        &[
            Constraint::Percentage(10),
            Constraint::Length(20),
            Constraint::Percentage(10),
        ],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Hea Head2                Hea│",
            "│                            │",
            "│Row Row12                Row│",
            "│Row Row22                Row│",
            "│Row Row32                Row│",
            "│Row Row42                Row│",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // columns of large width just before pushing a column off
    test_case(
        &[
            Constraint::Percentage(30),
            Constraint::Length(10),
            Constraint::Percentage(30),
        ],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1     Head2      Head3  │",
            "│                            │",
            "│Row11     Row12      Row13  │",
            "│Row21     Row22      Row23  │",
            "│Row31     Row32      Row33  │",
            "│Row41     Row42      Row43  │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    // columns of large size (>100% total) hide the last column
    test_case(
        &[
            Constraint::Percentage(60),
            Constraint::Length(10),
            Constraint::Percentage(60),
        ],
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1            Head2      │",
            "│                            │",
            "│Row11            Row12      │",
            "│Row21            Row22      │",
            "│Row31            Row32      │",
            "│Row41            Row42      │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );
}

#[test]
pub fn widgets_table_should_respect_horizontal_margins() {
    let test_case = |margin, width, expected| {
        let backend = TestBackend::new(width, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Length(5),
                ])
                .column_spacing(1)
                .margin(Margin::default().horizontal(margin));
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    test_case(
        0,
        30,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1 Head2 Head3           │",
            "│                            │",
            "│Row11 Row12 Row13           │",
            "│Row21 Row22 Row23           │",
            "│Row31 Row32 Row33           │",
            "│Row41 Row42 Row43           │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    test_case(
        1,
        30,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│ Head1 Head2 Head3          │",
            "│                            │",
            "│ Row11 Row12 Row13          │",
            "│ Row21 Row22 Row23          │",
            "│ Row31 Row32 Row33          │",
            "│ Row41 Row42 Row43          │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    test_case(
        1,
        21,
        Buffer::with_lines(vec![
            "┌───────────────────┐",
            "│ Head1 Head2 Head3 │",
            "│                   │",
            "│ Row11 Row12 Row13 │",
            "│ Row21 Row22 Row23 │",
            "│ Row31 Row32 Row33 │",
            "│ Row41 Row42 Row43 │",
            "│                   │",
            "│                   │",
            "└───────────────────┘",
        ]),
    );

    test_case(
        1,
        19,
        Buffer::with_lines(vec![
            "┌─────────────────┐",
            "│ Head1 Head2 Hea │",
            "│                 │",
            "│ Row11 Row12 Row │",
            "│ Row21 Row22 Row │",
            "│ Row31 Row32 Row │",
            "│ Row41 Row42 Row │",
            "│                 │",
            "│                 │",
            "└─────────────────┘",
        ]),
    );

    test_case(
        1,
        12,
        Buffer::with_lines(vec![
            "┌──────────┐",
            "│ Head1 He │",
            "│          │",
            "│ Row11 Ro │",
            "│ Row21 Ro │",
            "│ Row31 Ro │",
            "│ Row41 Ro │",
            "│          │",
            "│          │",
            "└──────────┘",
        ]),
    );

    test_case(
        2,
        21,
        Buffer::with_lines(vec![
            "┌───────────────────┐",
            "│  Head1 Head2 Hea  │",
            "│                   │",
            "│  Row11 Row12 Row  │",
            "│  Row21 Row22 Row  │",
            "│  Row31 Row32 Row  │",
            "│  Row41 Row42 Row  │",
            "│                   │",
            "│                   │",
            "└───────────────────┘",
        ]),
    );
}

#[test]
pub fn widgets_table_should_respect_vertical_margins() {
    let test_case = |margin, height, expected| {
        let backend = TestBackend::new(30, height);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Length(5),
                ])
                .column_spacing(1)
                .margin(Margin::default().vertical(margin));
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    test_case(
        0,
        10,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1 Head2 Head3           │",
            "│                            │",
            "│Row11 Row12 Row13           │",
            "│Row21 Row22 Row23           │",
            "│Row31 Row32 Row33           │",
            "│Row41 Row42 Row43           │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    test_case(
        1,
        10,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│                            │",
            "│Head1 Head2 Head3           │",
            "│                            │",
            "│Row11 Row12 Row13           │",
            "│Row21 Row22 Row23           │",
            "│Row31 Row32 Row33           │",
            "│Row41 Row42 Row43           │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    test_case(
        1,
        9,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│                            │",
            "│Head1 Head2 Head3           │",
            "│                            │",
            "│Row11 Row12 Row13           │",
            "│Row21 Row22 Row23           │",
            "│Row31 Row32 Row33           │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    test_case(
        2,
        10,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│                            │",
            "│                            │",
            "│Head1 Head2 Head3           │",
            "│                            │",
            "│Row11 Row12 Row13           │",
            "│Row21 Row22 Row23           │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );
}

#[test]
pub fn widgets_table_should_max_margin_and_highlighter() {
    let test_case = |margin, highlight_width, expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        let highlight_symbol = ">".repeat(highlight_width);

        terminal
            .draw(|mut f| {
                let size = f.size();
                let table = Table::new(
                    ["Head1", "Head2", "Head3"].iter(),
                    vec![
                        Row::Data(["Row11", "Row12", "Row13"].iter()),
                        Row::Data(["Row21", "Row22", "Row23"].iter()),
                        Row::Data(["Row31", "Row32", "Row33"].iter()),
                        Row::Data(["Row41", "Row42", "Row43"].iter()),
                    ]
                    .into_iter(),
                )
                .block(Block::default().borders(Borders::ALL))
                .widths(&[
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Length(5),
                ])
                .column_spacing(1)
                .margin(Margin::default().horizontal(margin))
                .highlight_symbol(&highlight_symbol);
                f.render_widget(table, size);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    test_case(
        0,
        0,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Head1 Head2 Head3           │",
            "│                            │",
            "│Row11 Row12 Row13           │",
            "│Row21 Row22 Row23           │",
            "│Row31 Row32 Row33           │",
            "│Row41 Row42 Row43           │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    test_case(
        2,
        1,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│  Head1 Head2 Head3         │",
            "│                            │",
            "│  Row11 Row12 Row13         │",
            "│  Row21 Row22 Row23         │",
            "│  Row31 Row32 Row33         │",
            "│  Row41 Row42 Row43         │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );

    test_case(
        2,
        3,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│   Head1 Head2 Head3        │",
            "│                            │",
            "│   Row11 Row12 Row13        │",
            "│   Row21 Row22 Row23        │",
            "│   Row31 Row32 Row33        │",
            "│   Row41 Row42 Row43        │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );
}
