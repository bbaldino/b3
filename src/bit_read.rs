use ux::u1;


pub trait BitRead {
    /// Pull some bytes from this source into the specified buffer, returning how many bytes were read.
    fn read(&mut self, buf: &mut [u1]) -> std::io::Result<usize>;

    /// Read the exact number of bytes required to fill buf.
    fn read_exact(&mut self, buf: &mut[u1]) -> std::io::Result<()>;
}

