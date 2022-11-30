use ux::u1;


pub trait BitWrite {
    /// Write a buffer into this writer, returning how many bytes were written.
    fn write(&mut self, buf: &[u1]) -> std::io::Result<usize>;
}
