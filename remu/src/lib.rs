#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtAddr(pub usize);

pub struct Mmu {
    memory: Vec<u8>,
}

impl Mmu {
    fn new(size: usize) -> Self {
        Mmu {
            memory: vec![0; size],
        }
    }

    fn read(&self, addr: VirtAddr, size: usize) -> Vec<u8> {
        let mut buf = vec![0; size];
        self.read_into(addr, &mut buf);
        buf
    }

    fn read_into(&self, addr: VirtAddr, buf: &mut [u8]) {
        buf.copy_from_slice(&self.memory[addr.0..addr.0 + buf.len()]);
    }

    fn write_from(&mut self, addr: VirtAddr, buf: &[u8]) {
        self.memory[addr.0..addr.0 + buf.len()].copy_from_slice(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let dat = vec![1, 2, 3, 4];
        let addr = VirtAddr(0);
        let mut mmu = Mmu::new(128);

        mmu.write_from(addr, &dat);
        assert_eq!(dat, mmu.read(addr, dat.len()));
    }
}
