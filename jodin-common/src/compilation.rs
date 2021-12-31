use crate::ast::JodinNode;
use crate::compilation_settings::CompilationSettings;
use crate::error::JodinResult;
use std::io;

/// A target for compilation.
pub trait Target {}

/// Compile code for a target.
pub trait Compiler<T: Target> {
    /// Attempt to compile the AST into something usable.
    fn compile(&mut self, tree: &JodinNode, settings: &CompilationSettings) -> JodinResult<()>;
}

/// Compile a part of the of program, such as a function.
pub trait MicroCompiler<T: Target, Output: Compilable<T>> {
    /// Compile a part of the program.
    ///
    /// # Arguments
    ///
    /// * `tree`: The AST that is being compiled
    /// * `context`: the context to compile the project in
    ///
    /// returns: Result<Output, JodinError>
    fn create_compilable(&mut self, tree: &JodinNode) -> JodinResult<Output>;
}

/// The context that the project is being built in.
pub struct Context {}

impl Context {
    /// Create a new context instance
    pub fn new() -> Self {
        Context {}
    }
}

/// Execute an arbitrary compiler
pub fn execute_compiler<T, C>(
    tree: JodinNode,
    mut compiler: C,
    compiler_settings: &CompilationSettings,
) -> JodinResult<()>
where
    T: Target,
    C: Compiler<T>,
{
    compiler.compile(&tree, compiler_settings)
}

/// A component of a program that can be compiled
pub trait Compilable<T: Target> {
    /// Compile the instance into a target writer
    fn compile<W: io::Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()>;
}

impl<T: Target, C: Compilable<T>> Compilable<T> for Vec<C> {
    fn compile<W: io::Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        for node in self {
            node.compile(context, w)?
        }
        Ok(())
    }
}

/// A writer that pads after every new line. The default padding string is "    ".
///
/// # Example
/// ```
/// use jodinc::ompilation::PaddedWriter;
/// use std::fmt::Write;
/// let mut to_pad = PaddedWriter::new(String::new());
/// writeln!(to_pad, "Hello");
/// to_pad.increase_pad();
/// writeln!(to_pad, "Hello");
/// to_pad.increase_pad();
/// writeln!(to_pad, "Hello");
/// writeln!(to_pad, "Hello");
/// to_pad.decrease_pad();
/// writeln!(to_pad, "Goodbye");
/// let string = to_pad.finish();
/// let hopeful =
/// r"Hello
///     Hello
///         Hello
///         Hello
///     Goodbye
/// ";
/// assert_eq!(string, hopeful)
/// ```
pub struct PaddedWriter<W: io::Write> {
    writer: W,
    pad_string: String,
    count: usize,
    pad_next: bool,
}

impl<W: io::Write> PaddedWriter<W> {
    /// Create a new padded writer from some other writer
    pub fn new(writer: W) -> Self {
        PaddedWriter {
            writer,
            pad_string: "    ".to_string(),
            count: 0,
            pad_next: true,
        }
    }

    /// Sets the pad string to use
    pub fn with_pad<S: AsRef<str>>(mut self, padding: S) -> Self {
        self.pad_string = padding.as_ref().to_string();
        self
    }

    /// Sets the initial pad count
    pub fn with_initial_pad(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    /// Increases the padding by one
    pub fn increase_pad(&mut self) {
        self.count += 1;
    }

    /// Decreases the padding by one
    pub fn decrease_pad(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }

    /// Changes the padding
    pub fn change_pad(&mut self, change: isize) {
        if change > 0 {
            self.count += change as usize;
        } else {
            match self.count.checked_sub(change as usize) {
                None => {
                    self.count = 0;
                }
                Some(new_val) => {
                    self.count = new_val;
                }
            }
        }
    }

    /// Finishes the padding, returning the inner writer
    pub fn finish(self) -> W {
        self.writer
    }
}

impl<W: io::Write> io::Write for PaddedWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for &c in buf {
            if self.pad_next {
                if self.count > 0 {
                    write!(self.writer, "{}", self.pad_string.repeat(self.count))?;
                }
                self.pad_next = false;
            }

            match c as char {
                '\n' => {
                    writeln!(self.writer)?;
                    self.pad_next = true;
                }
                _ => {
                    self.writer.write(&[c])?;
                }
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
