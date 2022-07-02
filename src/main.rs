use std::fmt::Alignment;
use druid::widget::{Button, Flex, Label, Padding, Align, MainAxisAlignment, CrossAxisAlignment};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Data,Color};
use druid::keyboard_types::Key::ColorF0Red;


fn main()-> Result<(),PlatformError> {

    let main_window= WindowDesc::new(full_sudoku_grid);
    let data = 0_u32;
    let launch_res=AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data);

    return launch_res;

}

fn full_sudoku_grid () -> impl Widget<u32> {
    Padding::new(10.0,Flex::column()
        .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
        .must_fill_main_axis(true)
        .with_flex_child(small_square_row(),1.0)
        .with_flex_child(small_square_row(),1.0)
        .with_flex_child(small_square_row(),1.0)
    )
}

fn small_square_row () -> impl Widget<u32> {
    Flex::row()
        .with_flex_child(small_square(),1.0)
        .with_flex_child(small_square(),1.0)
        .with_flex_child(small_square(),1.0)
}

fn small_square() -> impl Widget<u32> {
    Flex::column().main_axis_alignment(MainAxisAlignment::Center)
        .with_flex_child(
            custom_flex_row(Label::new("1"),Label::new("2"),Label::new("3")),
            1.0
        )
        .with_flex_child(
        custom_flex_row(Label::new("4"),Label::new("5"),Label::new("6")),
        1.0
        )
        .with_flex_child(
            custom_flex_row(Label::new("7"),Label::new("8"),Label::new("9")),
            1.0
        )
        .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
        .must_fill_main_axis(true)
        .border(Color::rgb(1.0,1.0,1.0),1.0)

}

fn custom_flex_row<T: Data>(box1: impl Widget<T> + 'static,box2: impl Widget<T>+ 'static, box3: impl Widget<T> + 'static) -> impl Widget<T>
{
    Flex::row()
        .with_flex_child(box1,1.0)
        .with_flex_child(box2,1.0)
        .with_flex_child(box3,1.0)
        .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
        .must_fill_main_axis(true)
        .border(Color::rgb(1.0,0.0,0.0),1.0)

}