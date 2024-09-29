use ratatui::prelude::*;

pub trait TuiView {
    #[allow(unused_variables)]
    fn view(&mut self, frame: &mut Frame, area: Rect) {}
}

pub fn sub_views_with_layouts<const N: usize>(
    frame: &mut Frame,
    area: Rect,
    direction: Direction,
    list: [(&mut dyn TuiView, Constraint); N],
) {
    let mut constraints: [Constraint; N] = [list[0].1; N]; // work around uninitialized
    for i in 0..N {
        constraints[i] = list[i].1;
    }
    let sub_areas = Layout::new(direction, constraints).areas::<N>(area);
    for i in 0..N {
        list[i].0.view(frame, sub_areas[i]);
    }
}
