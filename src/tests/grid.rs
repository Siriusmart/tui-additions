use ratatui::layout::{Constraint, Rect};

use crate::widgets::{Grid, GridError};

#[test]
fn length() {
    {
        let constraints = vec![Constraint::Percentage(60), Constraint::Percentage(40)];
        let length = 103;

        let lengths = Grid::lengths(&constraints, length).unwrap();

        assert_eq!(vec![60, 40], lengths)
    }

    {
        let constraints = vec![Constraint::Percentage(60), Constraint::Percentage(40)];
        let length = 3;

        let lengths = Grid::lengths(&constraints, length).unwrap();

        assert_eq!(vec![0, 0], lengths)
    }

    {
        let constraints = vec![Constraint::Percentage(60), Constraint::Percentage(40)];
        let length = 2;

        let err = Grid::lengths(&constraints, length).unwrap_err();

        assert_eq!(GridError::NotEnoughLength, err)
    }

    {
        let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        let length = 7;

        let lengths = Grid::lengths(&constraints, length).unwrap();

        assert_eq!(vec![2, 2], lengths)
    }
}

#[test]
fn chunks() {
    {
        let widths = vec![Constraint::Percentage(40), Constraint::Percentage(60)];
        let heights = vec![Constraint::Percentage(100)];
        let grid = Grid::new(widths, heights).unwrap();
        let area = Rect::new(10, 20, 103, 52);

        let chunks = grid.chunks(area).unwrap();

        assert_eq!(
            vec![vec![Rect::new(11, 21, 40, 50), Rect::new(52, 21, 60, 50)]],
            chunks
        );
    }

    {
        let widths = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        let heights = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
        let grid = Grid::new(widths, heights).unwrap();
        let area = Rect::new(10, 10, 103, 103);

        let chunks = grid.chunks(area).unwrap();

        assert_eq!(
            vec![
                vec![Rect::new(11, 11, 50, 50), Rect::new(62, 11, 50, 50)],
                vec![Rect::new(11, 62, 50, 50), Rect::new(62, 62, 50, 50)]
            ],
            chunks
        );
    }
}
