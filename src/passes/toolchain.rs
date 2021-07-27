//! Several common traits that model "toolchains". These structures allow for the chaining of
//! toolchains together to produce increasingly larger toolchains.
//!
//! # Examples
//!
//! ```
//! use std::str::Chars;
//! use jodin_rs::passes::toolchain::{Toolchain, Tool};
//! fn to_chars<S : AsRef<str>>(s: S) -> Chars {
//!     s.as_ref().chars()
//! }
//!
//! fn count_chars(c: Chars<'_>) -> usize {
//!     c.count()
//! }
//!
//! let mut toolchain = Toolchain::new(to_chars, count_chars);
//! assert_eq!(toolchain.invoke("Hello"), 5);
//! ```

use crate::core::error::{JodinError, JodinResult};
use std::marker::PhantomData;

/// The most basic component of a toolchain, representing a single tool
pub trait Tool {
    /// The input type for the tool
    type Input;
    /// The output type for the tool
    type Output;

    /// What action that the tool takes
    fn invoke(&mut self, input: Self::Input) -> Self::Output;
}

#[derive(Default)]
struct DummyTool<I, O>(PhantomData<I>, PhantomData<O>);

impl<I, O> Tool for DummyTool<I, O> {
    type Input = I;
    type Output = O;

    fn invoke(&mut self, _input: Self::Input) -> Self::Output {
        panic!("Can't call a dummy tool")
    }
}

impl<I, O> Tool for fn(I) -> O {
    type Input = I;
    type Output = O;

    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        (self)(input)
    }
}

/// A tool adapter that takes in two different tools, where the first tool's output and the second tool's
/// input are the same type.
pub struct Toolchain<S, M = S, E = M> {
    start: Box<dyn Tool<Input = S, Output = M>>,
    end: Box<dyn Tool<Input = M, Output = E>>,
}

impl<S, M, E> Toolchain<S, M, E> {
    /// Creates a new toolchain from two tools
    pub fn new<T1, T2>(first: T1, second: T2) -> Self
    where
        T1: 'static + Tool<Input = S, Output = M>,
        T2: 'static + Tool<Input = M, Output = E>,
    {
        Toolchain {
            start: Box::new(first),
            end: Box::new(second),
        }
    }
}

impl<S, M, E> Tool for Toolchain<S, M, E> {
    type Input = S;
    type Output = E;

    /// Calling a toolchain's invoke will invoke the first tool, then take its output and use it
    /// as the input of the second tool.
    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        let mid = self.start.invoke(input);
        self.end.invoke(mid)
    }
}

/// Adds more functionality to tools
pub trait ToolchainUtilities: Tool {
    /// Append a tool to this tool, returning a toolchain adapter
    fn append_tool<O, T: 'static + Tool<Input = Self::Output, Output = O>>(
        self,
        other: T,
    ) -> Toolchain<Self::Input, Self::Output, O>
    where
        Self: 'static + Sized,
    {
        Toolchain {
            start: Box::new(self),
            end: Box::new(other),
        }
    }

    /// Prepend a tool to this tool, returning a toolchain adapter
    fn prepend_tool<I, T: 'static + Tool<Input = I, Output = Self::Input>>(
        self,
        other: T,
    ) -> Toolchain<I, Self::Input, Self::Output>
    where
        Self: 'static + Sized,
    {
        Toolchain {
            start: Box::new(other),
            end: Box::new(self),
        }
    }
}

impl<T> ToolchainUtilities for T where T: Tool {}

/// A tool that returns a result, and thus can "fail"
pub trait FallibleTool {
    /// The input type.
    type Input;
    /// The output type.
    type Output;
    /// The error type.
    type Error;

    /// The action that the tool will take.
    fn invoke(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

/// A tool that that returns a [JodinResult](crate::core::error::JodinResult).
pub trait JodinFallibleTool {
    /// The input type.
    type Input;
    /// The output type.
    type Output;

    /// The action that the tool will take.
    fn invoke(&mut self, input: Self::Input) -> JodinResult<Self::Output>;
}

impl<T: JodinFallibleTool> FallibleTool for T {
    type Input = T::Input;
    type Output = T::Output;
    type Error = JodinError;

    fn invoke(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        T::invoke(self, input)
    }
}

impl<T: FallibleTool> Tool for T {
    type Input = T::Input;
    type Output = Result<T::Output, T::Error>;

    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        T::invoke(self, input)
    }
}

/// A toolchain of two tools that can return a result
pub struct FallibleToolchain<Error, Input, Mid = Input, Output = Mid> {
    start: Box<dyn Tool<Input = Input, Output = Result<Mid, Error>>>,
    end: Box<dyn Tool<Input = Mid, Output = Result<Output, Error>>>,
}

impl<Error, Input, Mid, Output> FallibleToolchain<Error, Input, Mid, Output> {
    /// Creates a new toolchain from two tools.
    pub fn new<T1, T2>(first: T1, second: T2) -> Self
    where
        T1: 'static + Tool<Input = Input, Output = Result<Mid, Error>>,
        T2: 'static + Tool<Input = Mid, Output = Result<Output, Error>>,
    {
        FallibleToolchain {
            start: Box::new(first),
            end: Box::new(second),
        }
    }
}

impl<Error, Input, Mid, Output> FallibleTool for FallibleToolchain<Error, Input, Mid, Output> {
    type Input = Input;
    type Output = Output;
    type Error = Error;

    /// Invokes the first tool, and only invokes the second tool if the first didn't fail.
    fn invoke(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mid = self.start.invoke(input)?;
        self.end.invoke(mid)
    }
}

/// Provides helper methods for types that implement FallibleTool
pub trait FallibleToolchainUtilities: FallibleTool {
    /// Append a fallible tool to this tool.
    fn append_tool<
        O,
        T: 'static + FallibleTool<Error = Self::Error, Input = Self::Output, Output = O>,
    >(
        self,
        other: T,
    ) -> FallibleToolchain<Self::Error, Self::Input, Self::Output, O>
    where
        Self: 'static + Sized,
    {
        FallibleToolchain {
            start: Box::new(self),
            end: Box::new(other),
        }
    }

    /// Prepend a fallible tool to this tool.
    fn prepend_tool<
        I,
        T: 'static + FallibleTool<Error = Self::Error, Input = I, Output = Self::Input>,
    >(
        self,
        other: T,
    ) -> FallibleToolchain<Self::Error, I, Self::Input, Self::Output>
    where
        Self: 'static + Sized,
    {
        FallibleToolchain {
            start: Box::new(other),
            end: Box::new(self),
        }
    }

    /// Append an infallible tool to this tool.
    fn append_infallible<O, T: 'static + Tool<Input = Self::Output, Output = O>>(
        self,
        other: T,
    ) -> FallibleToolchain<Self::Error, Self::Input, Self::Output, O>
    where
        Self: 'static + Sized,
    {
        FallibleToolchain {
            start: Box::new(self),
            end: Box::new(FallibleWrapper(other, PhantomData)),
        }
    }

    /// Prepend an infallible to this tool.
    fn prepend_infallible<I, T: 'static + Tool<Input = I, Output = Self::Input>>(
        self,
        other: T,
    ) -> FallibleToolchain<Self::Error, I, Self::Input, Self::Output>
    where
        Self: 'static + Sized,
    {
        FallibleToolchain {
            start: Box::new(FallibleWrapper(other, PhantomData)),
            end: Box::new(self),
        }
    }
}

impl<T: FallibleTool> FallibleToolchainUtilities for T {}

struct FallibleWrapper<T: Tool, E>(T, PhantomData<E>);

impl<T: Tool, E> FallibleTool for FallibleWrapper<T, E> {
    type Input = T::Input;
    type Output = T::Output;
    type Error = E;

    fn invoke(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(self.0.invoke(input))
    }
}

/// A tool that takes an iterator as an input.
pub trait CollectorTool {
    /// The item type for the iterator.
    type Input;
    /// The output type.
    type Output;

    /// Call the collector.
    fn invoke<I: IntoIterator<Item = Self::Input>>(&mut self, input_iter: I) -> Self::Output;
}

/// A tool that takes an iterator as an input that can returns a result.
pub trait FallibleCollectorTool {
    /// The item type for the iterator.
    type Input;
    /// The output type.
    type Output;
    /// The error type.
    type Error;

    /// Call the fallible collector.
    fn invoke<I: IntoIterator<Item = Self::Input>>(
        &mut self,
        input_iter: I,
    ) -> Result<Self::Output, Self::Error>;
}

impl<F: FallibleCollectorTool> CollectorTool for F {
    type Input = F::Input;
    type Output = Result<F::Output, F::Error>;

    fn invoke<I: IntoIterator<Item = Self::Input>>(&mut self, input_iter: I) -> Self::Output {
        FallibleCollectorTool::invoke(self, input_iter)
    }
}

/// A collector, fallible tool that that returns a [JodinResult](crate::core::error::JodinResult).
pub trait JodinFallibleCollectorTool {
    /// The input type.
    type Input;
    /// The output type.
    type Output;

    /// Call the fallible collector.
    fn invoke<I: IntoIterator<Item = Self::Input>>(
        &mut self,
        input_iter: I,
    ) -> JodinResult<Self::Output>;
}

/// A toolchain that takes in a tool and ends with a collector tool.
pub struct CollectorToolchain<T, CT, Input, Mid, Output>
where
    T: Tool<Input = Input, Output = Mid>,
    CT: CollectorTool<Input = Mid, Output = Output>,
{
    input: T,
    collector: CT,
}

impl<T, CT, Input, Mid, Output> CollectorTool for CollectorToolchain<T, CT, Input, Mid, Output>
where
    T: Tool<Input = Input, Output = Mid>,
    CT: CollectorTool<Input = Mid, Output = Output>,
{
    type Input = Input;
    type Output = Output;

    fn invoke<I: IntoIterator<Item = Self::Input>>(&mut self, input_iter: I) -> Self::Output {
        let vec: Vec<_> = input_iter
            .into_iter()
            .map(|item| self.input.invoke(item))
            .collect();
        self.collector.invoke(vec)
    }
}

impl<T, CT, Input, Mid, Output> CollectorToolchain<T, CT, Input, Mid, Output>
where
    T: Tool<Input = Input, Output = Mid>,
    CT: CollectorTool<Input = Mid, Output = Output>,
{
    /// Creates a new collector toolchain from a tool and a collector tool.
    pub fn new(input: T, collector: CT) -> Self {
        CollectorToolchain { input, collector }
    }
}

/// A fallible collector that takes in a tool and ends with a collector tool.
pub struct FallibleCollectorToolchain<T, CT, Input, Mid, Output, Error1, Error2>
where
    T: Tool<Input = Input, Output = Result<Mid, Error1>>,
    CT: CollectorTool<Input = Mid, Output = Result<Output, Error2>>,
    Error1: Into<Error2>,
{
    input: T,
    collector: CT,
}

impl<T, CT, Input, Mid, Output, Error1, Error2>
    FallibleCollectorToolchain<T, CT, Input, Mid, Output, Error1, Error2>
where
    T: Tool<Input = Input, Output = Result<Mid, Error1>>,
    CT: CollectorTool<Input = Mid, Output = Result<Output, Error2>>,
    Error1: Into<Error2>,
{
    /// Creates a new fallible collector toolchain using a tool and collector that both results in
    /// an error.
    pub fn new(input: T, collector: CT) -> Self {
        FallibleCollectorToolchain { input, collector }
    }
}

impl<T, CT, Input, Mid, Output, Error1, Error2> FallibleCollectorTool
    for FallibleCollectorToolchain<T, CT, Input, Mid, Output, Error1, Error2>
where
    T: Tool<Input = Input, Output = Result<Mid, Error1>>,
    CT: CollectorTool<Input = Mid, Output = Result<Output, Error2>>,
    Error2: From<Error1>,
{
    type Input = Input;
    type Output = Output;
    type Error = Error2;

    fn invoke<I: IntoIterator<Item = Self::Input>>(
        &mut self,
        input_iter: I,
    ) -> Result<Self::Output, Self::Error> {
        let mut vec = vec![];
        for item in input_iter {
            vec.push(self.input.invoke(item)?)
        }
        self.collector.invoke(vec)
    }
}

/// Simple tool that maps values to other values
pub struct MapTool<I, O, F: Fn(I) -> O> {
    callback: F,
    _phantom1: PhantomData<I>,
    _phantom2: PhantomData<O>,
}

/// creates a simple tool that maps values to other values
pub fn map_tool<I, O, F: Fn(I) -> O>(callback: F) -> MapTool<I, O, F> {
    MapTool {
        callback,
        _phantom1: Default::default(),
        _phantom2: Default::default(),
    }
}

impl<I, O, F: Fn(I) -> O> Tool for MapTool<I, O, F> {
    type Input = I;
    type Output = O;

    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        (self.callback)(input)
    }
}

macro_rules! chain_tools {
    ($first:expr $(,$tool:expr)*) => {
        {
            $first
            $(.append_tool($tool))*
        }
    };
}
