use crate::core::error::{JodinError, JodinResult};
use std::marker::PhantomData;

pub trait Tool {
    type Input;
    type Output;
    fn invoke(&mut self, input: Self::Input) -> Self::Output;
}

pub struct Toolchain<S, M = S, E = M> {
    start: Box<dyn Tool<Input = S, Output = M>>,
    end: Box<dyn Tool<Input = M, Output = E>>,
}

impl<S, M, E> Toolchain<S, M, E> {
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

    fn invoke(&mut self, input: Self::Input) -> Self::Output {
        let mid = self.start.invoke(input);
        self.end.invoke(mid)
    }
}

pub trait ToolchainUtilities: Tool {
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

pub trait FallibleTool {
    type Input;
    type Output;
    type Error;

    fn invoke(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

pub trait JodinFallibleTool {
    type Input;
    type Output;
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

pub struct FallibleToolchain<Error, Input, Mid = Input, Output = Mid> {
    start: Box<dyn Tool<Input = Input, Output = Result<Mid, Error>>>,
    end: Box<dyn Tool<Input = Mid, Output = Result<Output, Error>>>,
}

impl<Error, Input, Mid, Output> FallibleToolchain<Error, Input, Mid, Output> {
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

    fn invoke(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mid = self.start.invoke(input)?;
        self.end.invoke(mid)
    }
}

pub trait FallibleToolchainUtilities: FallibleTool {
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
