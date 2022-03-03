use crossterm::style::{style, Color, StyledContent, Stylize};
use std::cell::RefCell;
use std::fmt::Display;
use std::io::Write;
use std::rc::Rc;

macro_rules! impl_color_writer_fn {
    ($($source_method:tt $method_name:tt $color:ident),* $(,)?) => {
        $(
            pub fn $method_name(&mut self) -> Writing<'a> {
                self.$source_method(Color::$color)
            }
        )*
    };
}

macro_rules! impl_writing_fn {
    ($($source_method:tt $method_name:tt $color:ident),* $(,)?) => {
        $(
            pub fn $method_name(self) -> Self {
                self.$source_method(Color::$color)
            }
        )*
    };
}

macro_rules! impl_color_writer_methods {
    () => {
        impl_color_writer_fn! {
            // $color()
            fg red Red,
            fg green Green,
            fg yellow Yellow,
            fg blue Blue,
            fg magenta Magenta,
            fg cyan Cyan,
            fg white White,
            fg black Black,
            fg gray Grey,
            fg dark_red DarkRed,
            fg dark_green DarkGreen,
            fg dark_yellow DarkYellow,
            fg dark_blue DarkBlue,
            fg dark_magenta DarkMagenta,
            fg dark_cyan DarkCyan,
            fg dark_gray DarkGrey,

            // on_$color
            bg on_red Red,
            bg on_green Green,
            bg on_yellow Yellow,
            bg on_blue Blue,
            bg on_magenta Magenta,
            bg on_cyan Cyan,
            bg on_white White,
            bg on_black Black,
            bg on_gray Grey,
            bg on_dark_red DarkRed,
            bg on_dark_green DarkGreen,
            bg on_dark_yellow DarkYellow,
            bg on_dark_blue DarkBlue,
            bg on_dark_magenta DarkMagenta,
            bg on_dark_cyan DarkCyan,
            bg on_dark_gray DarkGrey,
        }
    };
}

macro_rules! impl_writing_methods {
    () => {
        impl_writing_fn! {
            // $color()
            fg red Red,
            fg green Green,
            fg yellow Yellow,
            fg blue Blue,
            fg magenta Magenta,
            fg cyan Cyan,
            fg white White,
            fg black Black,
            fg gray Grey,
            fg dark_red DarkRed,
            fg dark_green DarkGreen,
            fg dark_yellow DarkYellow,
            fg dark_blue DarkBlue,
            fg dark_magenta DarkMagenta,
            fg dark_cyan DarkCyan,
            fg dark_gray DarkGrey,

            // on_$color()
            bg on_red Red,
            bg on_green Green,
            bg on_yellow Yellow,
            bg on_blue Blue,
            bg on_magenta Magenta,
            bg on_cyan Cyan,
            bg on_white White,
            bg on_black Black,
            bg on_gray Grey,
            bg on_dark_red DarkRed,
            bg on_dark_green DarkGreen,
            bg on_dark_yellow DarkYellow,
            bg on_dark_blue DarkBlue,
            bg on_dark_magenta DarkMagenta,
            bg on_dark_cyan DarkCyan,
            bg on_dark_gray DarkGrey,
        }
    };
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TextStyling {
    fg: Option<Color>,
    bg: Option<Color>,
}

#[allow(unused)]
impl TextStyling {
    pub fn new() -> Self {
        Self {
            fg: None,
            bg: None,
        }
    }

    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }
}

struct WriteInner<'a> {
    no_color: bool,
    hook: Option<Box<dyn FnMut(&mut ColorWriter) + 'a>>,

    // We cached the writer to use in the hook
    writer_hook: Option<ColorWriter<'a>>
}

type RcWriter<'a> = Rc<RefCell<WriteInner<'a>>>;

trait ColorWriterInner {
    fn write<D: Display>(&mut self, data: D, color: Option<Color>);
    fn writeln<D: Display>(&mut self, data: D, color: Option<Color>);
    fn write_err<D: Display>(&mut self, data: D, color: Option<Color>);
    fn writeln_err<D: Display>(&mut self, data: D, color: Option<Color>);
    fn flush(&mut self);
    fn no_color(&self) -> bool;
    fn call_hook(&mut self);
}

impl ColorWriterInner for RcWriter<'_> {
    fn write<D: Display>(&mut self, data: D, color: Option<Color>) {
        self.call_hook();

        let no_color = self.borrow().no_color;
        write_color(no_color, &mut std::io::stdout(), data, color);
    }

    fn writeln<D: Display>(&mut self, data: D, color: Option<Color>) {
        self.write(data, color);
        std::io::stdout().write_all(b"\n").unwrap();
    }

    fn write_err<D: Display>(&mut self, data: D, color: Option<Color>) {
        self.call_hook();

        let no_color = self.borrow().no_color;
        write_color(no_color, &mut std::io::stderr(), data, color);
    }

    fn writeln_err<D: Display>(&mut self, data: D, color: Option<Color>) {
        self.write_err(data, color);
        std::io::stderr().write_all(b"\n").unwrap();
    }

    fn flush(&mut self) {
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();
    }

    fn no_color(&self) -> bool {
        self.borrow().no_color
    }

    fn call_hook(&mut self) {
        if let Some(hook) = &mut self.borrow_mut().hook {
            let writer = &mut self.borrow_mut().writer_hook;
            hook(writer.get_or_insert_with(|| ColorWriter {
                inner: self.clone()
            }));
        }
    }
}

pub struct ColorWriter<'a> {
    inner: RcWriter<'a>,
}

#[allow(unused)]
impl<'a> ColorWriter<'a> {
    pub fn new(no_color: bool) -> Self {
        ColorWriter {
            inner: Rc::new(RefCell::new(WriteInner {
                hook: None,
                writer_hook: None,
                no_color,
            })),
        }
    }

    pub fn colorized() -> Self {
        ColorWriter::new(false)
    }

    pub fn with_hook<F>(no_color: bool, f: F) -> Self
    where
        F: FnMut(&mut ColorWriter) + 'a,
    {
        ColorWriter {
            inner: Rc::new(RefCell::new(WriteInner {
                hook: Some(Box::new(f)),
                writer_hook: None,
                no_color,
            })),
        }
    }

    pub fn flush(&mut self) {
        self.inner.flush();
    }

    pub fn write<D: Display>(&mut self, data: D, color: Option<Color>) {
        self.inner.write(data, color);
    }

    pub fn writeln<D: Display>(&mut self, data: D, color: Option<Color>) {
        self.inner.writeln(data, color);
    }

    pub fn write_err<D: Display>(&mut self, data: D, color: Option<Color>) {
        self.inner.write_err(data, color);
    }

    pub fn writeln_err<D: Display>(&mut self, data: D, color: Option<Color>) {
        self.inner.writeln_err(data, color);
    }

    pub fn styled(&mut self, style: TextStyling) -> Writing<'a> {
        Writing {
            inner: self.inner.clone(),
            fg: style.fg,
            bg: style.bg,
        }
    }

    pub fn fg(&mut self, color: Color) -> Writing<'a> {
        Writing {
            inner: self.inner.clone(),
            fg: Some(color),
            bg: None,
        }
    }

    pub fn bg(&mut self, color: Color) -> Writing<'a> {
        Writing {
            inner: self.inner.clone(),
            fg: None,
            bg: Some(color),
        }
    }

    impl_color_writer_methods!();
}

pub struct Writing<'a> {
    inner: RcWriter<'a>,
    fg: Option<Color>,
    bg: Option<Color>,
}

#[allow(unused)]
impl<'a> Writing<'a> {
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    fn styled<D: Display>(&self, data: D) -> StyledContent<D> {
        let mut stylized = StyledContent::new(Default::default(), data);

        if let Some(fg) = &self.fg {
            stylized = stylized.with(fg.clone());
        }

        if let Some(bg) = &self.bg {
            stylized = stylized.with(bg.clone());
        }

        stylized
    }

    pub fn write<D: Display>(&mut self, data: D) {
        let no_color = self.inner.no_color();
        if no_color {
            self.inner.write(data, None);
        } else {
            let styled = self.styled(data);
            self.inner.write(styled, None);
        }
    }

    pub fn writeln<D: Display>(&mut self, data: D) {
        self.write(data);
        self.write("\n");
    }

    pub fn write_err<D: Display>(&mut self, data: D) {
        let no_color = self.inner.no_color();
        if no_color {
            self.inner.write_err(data, None);
        } else {
            let styled = self.styled(data);
            self.inner.write_err(styled, None);
        }
    }

    pub fn writeln_err<D: Display>(&mut self, data: D) {
        self.write_err(data);
        self.write_err("\n");
    }

    impl_writing_methods!();
}

fn write_color<D, W>(no_color: bool, writer: &mut W, data: D, color: Option<Color>)
where
    D: Display,
    W: Write,
{
    if no_color {
        write!(writer, "{}", data).unwrap();
    } else {
        match color {
            Some(color) => write!(writer, "{}", style(data).with(color)).unwrap(),
            None => write!(writer, "{}", data).unwrap(),
        }
    }
}
