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
