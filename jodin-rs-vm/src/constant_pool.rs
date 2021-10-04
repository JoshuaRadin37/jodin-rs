use crate::memory::PopFromStack;

pub struct ConstantPool;

trait ConstantPoolTrait<V: AsRef<[u8]>>: From<V> + Into<Vec<u8>> {
    fn set<P: PopFromStack>(&mut self, key: usize, value: P);
}
