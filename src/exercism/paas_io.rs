use std::io::{Read, Result, Write};
pub fn call() {}

pub struct IoState<T> {
    wrapped: T,
    bytes_through: usize,
    reads: usize,
    writes: usize,
}

type ReadStats<T> = IoState<T>;
type WriteStats<T> = IoState<T>;

impl<T> IoState<T> {
    fn new(wrapped: T) -> Self {
        Self {
            wrapped,
            bytes_through: Default::default(),
            writes: Default::default(),
            reads: Default::default(),
        }
    }

    fn get_ref(&self) -> &T {
        &self.wrapped
    }

    fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    fn reads(&self) -> usize {
        self.reads
    }

    fn writes(&self) -> usize {
        self.writes
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.wrapped.read(buf).inspect(|len| {
            self.bytes_through += len;
        })
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        self.wrapped.write(buf).inspect(|len| {
            self.bytes_through += len;
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}

#[cfg(test)]
mod read_string {

    use super::*;
    use std::io::{BufReader, Read};

    const CHUNK_SIZE: usize = 2;
    static INPUT: &[u8] = b"Twas brillig, and the slithy toves/Did gyre and gimble in the wabe:/All mimsy were the borogoves,/And the mome raths outgrabe.";

    #[test]
    fn read_passthrough() {
        let data = INPUT;
        let size = data.len();
        let mut reader = ReadStats::new(data);
        let mut buffer = Vec::with_capacity(size);
        let qty_read = reader.read_to_end(&mut buffer);

        assert!(qty_read.is_ok());
        assert_eq!(size, qty_read.unwrap());
        assert_eq!(size, buffer.len());

        // 2: first to read all the data, second to check that
        // there wasn't any more pending data which simply didn't
        // fit into the existing buffer
        assert_eq!(2, reader.reads());
        assert_eq!(size, reader.bytes_through());
    }

    #[test]
    fn read_chunks() {
        let data = INPUT;
        let size = data.len();
        let mut reader = ReadStats::new(data);
        let mut buffer = [0_u8; CHUNK_SIZE];
        let mut chunks_read = 0;

        while reader.read(&mut buffer[..]).unwrap() > 0 {
            chunks_read += 1;
        }

        assert_eq!(size / CHUNK_SIZE + 1.min(size % CHUNK_SIZE), chunks_read);
        assert_eq!(1 + chunks_read, reader.reads());
        assert_eq!(size, reader.bytes_through());
    }

    #[test]
    fn read_buffered_chunks() {
        let data = INPUT;
        let size = data.len();
        let mut reader = BufReader::new(ReadStats::new(data));
        let mut buffer = [0_u8; CHUNK_SIZE];
        let mut chunks_read = 0;

        while reader.read(&mut buffer[..]).unwrap() > 0 {
            chunks_read += 1;
        }

        assert_eq!(size / CHUNK_SIZE + 1.min(size % CHUNK_SIZE), chunks_read);
        // the BufReader should smooth out the reads, collecting into
        // a buffer and performing only two read operations:
        // the first collects everything into the buffer,
        // and the second ensures that no data remains
        assert_eq!(2, reader.get_ref().reads());
        assert_eq!(size, reader.get_ref().bytes_through());
    }
}

#[cfg(test)]
mod write_string {
    use super::*;
    use std::io::{self, BufWriter, Write};

    const CHUNK_SIZE: usize = 2;
    static INPUT: &[u8] = b"Beware the Jabberwock, my son!/The jaws that bite, the claws that catch!/Beware the Jubjub bird, and shun/The frumious Bandersnatch!";

    #[test]
    fn write_passthrough() {
        let data = INPUT;
        let size = data.len();
        let mut writer = WriteStats::new(Vec::with_capacity(size));
        let written = writer.write(data);

        assert!(written.is_ok());
        assert_eq!(size, written.unwrap());
        assert_eq!(size, writer.bytes_through());
        assert_eq!(1, writer.writes());
        assert_eq!(data, writer.get_ref().as_slice());
    }

    #[test]
    fn sink_oneshot() {
        let data = INPUT;
        let size = data.len();
        let mut writer = WriteStats::new(io::sink());
        let written = writer.write(data);

        assert!(written.is_ok());
        assert_eq!(size, written.unwrap());
        assert_eq!(size, writer.bytes_through());
        assert_eq!(1, writer.writes());
    }

    #[test]
    fn sink_windowed() {
        let data = INPUT;
        let size = data.len();
        let mut writer = WriteStats::new(io::sink());
        let mut chunk_count = 0;

        for chunk in data.chunks(CHUNK_SIZE) {
            chunk_count += 1;
            let written = writer.write(chunk);
            assert!(written.is_ok());
            assert_eq!(CHUNK_SIZE, written.unwrap());
        }

        assert_eq!(size, writer.bytes_through());
        assert_eq!(chunk_count, writer.writes());
    }

    #[test]
    fn sink_buffered_windowed() {
        let data = INPUT;
        let size = data.len();
        let mut writer = BufWriter::new(WriteStats::new(io::sink()));
        for chunk in data.chunks(CHUNK_SIZE) {
            let written = writer.write(chunk);
            assert!(written.is_ok());
            assert_eq!(CHUNK_SIZE, written.unwrap());
        }

        // at this point, nothing should have yet been passed through to
        // our writer
        assert_eq!(0, writer.get_ref().bytes_through());
        assert_eq!(0, writer.get_ref().writes());
        // after flushing, everything should pass through in one go
        assert!(writer.flush().is_ok());
        assert_eq!(size, writer.get_ref().bytes_through());
        assert_eq!(1, writer.get_ref().writes());
    }
}
