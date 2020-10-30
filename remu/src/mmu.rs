use std::path::PathBuf;
use std::convert::TryInto;


#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtAddr(pub usize);

pub struct Mmu {
    memory: Vec<u8>,
    cur_alloc: VirtAddr,
    pub entry_point : Option<VirtAddr>,
}

#[allow(dead_code)]
impl Mmu {
    pub fn new(size: usize) -> Self {
        Mmu {
            memory: vec![0; size],
            cur_alloc: VirtAddr(0),
            entry_point: None,
        }
    }

    pub fn alloc(&mut self, size: usize) -> VirtAddr {
        let r = self.cur_alloc;
        self.cur_alloc = VirtAddr(self.cur_alloc.0 + size);
        r
    }

    pub fn read(&self, addr: VirtAddr, size: usize) -> &[u8] {
        &self.memory[addr.0..addr.0 + size]
    }

    pub fn read_u32(&self, addr: VirtAddr) -> u32 {
        let bts : [u8; 4] = self.memory[addr.0..addr.0 + 4].try_into().unwrap();
        u32::from_le_bytes(bts)
    }

    pub fn read_into(&self, addr: VirtAddr, buf: &mut [u8]) {
        buf.copy_from_slice(&self.memory[addr.0..addr.0 + buf.len()]);
    }

    pub fn write_from(&mut self, addr: VirtAddr, buf: &[u8]) {
        self.memory[addr.0..addr.0 + buf.len()].copy_from_slice(buf);
    }

    pub fn load_section(&mut self, section: &elf::Section) {
        let header = &section.shdr;
        let addr = header.addr as usize;
        let size = header.size as usize;

        self.write_from(VirtAddr(addr), &section.data);
        self.cur_alloc = VirtAddr(std::cmp::max(self.cur_alloc.0, addr + size));
    }

    pub fn load_elf(&mut self, path : &PathBuf) {
        let file = elf::File::open_path(&path).unwrap();
        self.entry_point = Some(VirtAddr(file.ehdr.entry as usize));

        for section in &file.sections {
            if section.shdr.addr > 0 {
                self.load_section(&section);
            }
        }
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

        let mut dat2 = vec![0; 4];
        mmu.read_into(addr, &mut dat2);
        assert_eq!(dat, dat2);

    }
}
