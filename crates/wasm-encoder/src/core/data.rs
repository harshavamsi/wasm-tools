use crate::{encoders, Instruction, Section, SectionId};

/// An encoder for the data section.
///
/// Data sections are only supported for modules.
///
/// # Example
///
/// ```
/// use wasm_encoder::{
///     DataSection, Instruction, MemorySection, MemoryType,
///     Module,
/// };
///
/// let mut memory = MemorySection::new();
/// memory.memory(MemoryType {
///     minimum: 1,
///     maximum: None,
///     memory64: false,
/// });
///
/// let mut data = DataSection::new();
/// let memory_index = 0;
/// let offset = Instruction::I32Const(42);
/// let segment_data = b"hello";
/// data.active(memory_index, &offset, segment_data.iter().copied());
///
/// let mut module = Module::new();
/// module
///     .section(&memory)
///     .section(&data);
///
/// let wasm_bytes = module.finish();
/// ```
#[derive(Clone, Default, Debug)]
pub struct DataSection {
    bytes: Vec<u8>,
    num_added: u32,
}

/// A segment in the data section.
#[derive(Clone, Debug)]
pub struct DataSegment<'a, D> {
    /// This data segment's mode.
    pub mode: DataSegmentMode<'a>,
    /// This data segment's data.
    pub data: D,
}

/// A data segment's mode.
#[derive(Clone, Debug)]
pub enum DataSegmentMode<'a> {
    /// An active data segment.
    Active {
        /// The memory this segment applies to.
        memory_index: u32,
        /// The offset where this segment's data is initialized at.
        offset: &'a Instruction<'a>,
    },
    /// A passive data segment.
    ///
    /// Passive data segments are part of the bulk memory proposal.
    Passive,
}

impl DataSection {
    /// Create a new data section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of data segments in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Define a data segment.
    pub fn segment<D>(&mut self, segment: DataSegment<D>) -> &mut Self
    where
        D: IntoIterator<Item = u8>,
        D::IntoIter: ExactSizeIterator,
    {
        match segment.mode {
            DataSegmentMode::Passive => {
                self.bytes.push(0x01);
            }
            DataSegmentMode::Active {
                memory_index: 0,
                offset,
            } => {
                self.bytes.push(0x00);
                offset.encode(&mut self.bytes);
                Instruction::End.encode(&mut self.bytes);
            }
            DataSegmentMode::Active {
                memory_index,
                offset,
            } => {
                self.bytes.push(0x02);
                self.bytes.extend(encoders::u32(memory_index));
                offset.encode(&mut self.bytes);
                Instruction::End.encode(&mut self.bytes);
            }
        }

        let data = segment.data.into_iter();
        self.bytes
            .extend(encoders::u32(u32::try_from(data.len()).unwrap()));
        self.bytes.extend(data);

        self.num_added += 1;
        self
    }

    /// Define an active data segment.
    pub fn active<D>(&mut self, memory_index: u32, offset: &Instruction, data: D) -> &mut Self
    where
        D: IntoIterator<Item = u8>,
        D::IntoIter: ExactSizeIterator,
    {
        self.segment(DataSegment {
            mode: DataSegmentMode::Active {
                memory_index,
                offset,
            },
            data,
        })
    }

    /// Define a passive data segment.
    ///
    /// Passive data segments are part of the bulk memory proposal.
    pub fn passive<D>(&mut self, data: D) -> &mut Self
    where
        D: IntoIterator<Item = u8>,
        D::IntoIter: ExactSizeIterator,
    {
        self.segment(DataSegment {
            mode: DataSegmentMode::Passive,
            data,
        })
    }

    /// Copy an already-encoded data segment into this data section.
    pub fn raw(&mut self, already_encoded_data_segment: &[u8]) -> &mut Self {
        self.bytes.extend_from_slice(already_encoded_data_segment);
        self.num_added += 1;
        self
    }
}

impl Section for DataSection {
    fn id(&self) -> u8 {
        SectionId::Data.into()
    }

    fn encode<S>(&self, sink: &mut S)
    where
        S: Extend<u8>,
    {
        let num_added = encoders::u32(self.num_added);
        let n = num_added.len();
        sink.extend(
            encoders::u32(u32::try_from(n + self.bytes.len()).unwrap())
                .chain(num_added)
                .chain(self.bytes.iter().copied()),
        );
    }
}

/// An encoder for the data count section.
#[derive(Clone, Copy, Debug)]
pub struct DataCountSection {
    /// The number of segments in the data section.
    pub count: u32,
}

impl Section for DataCountSection {
    fn id(&self) -> u8 {
        SectionId::DataCount.into()
    }

    fn encode<S>(&self, sink: &mut S)
    where
        S: Extend<u8>,
    {
        let count = encoders::u32(self.count);
        let n = count.len();
        sink.extend(encoders::u32(u32::try_from(n).unwrap()).chain(count));
    }
}
