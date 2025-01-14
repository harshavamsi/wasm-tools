mod aliases;
mod components;
mod exports;
mod functions;
mod instances;
mod modules;
mod start;
mod types;

pub use self::aliases::*;
pub use self::components::*;
pub use self::exports::*;
pub use self::functions::*;
pub use self::instances::*;
pub use self::modules::*;
pub use self::start::*;
pub use self::types::*;

/// A WebAssembly component section.
///
/// Various builders defined in this crate already implement this trait, but you
/// can also implement it yourself for your own custom section builders, or use
/// `RawSection` to use a bunch of raw bytes as a section.
pub trait ComponentSection {
    /// Gets the section's identifier.
    fn id(&self) -> u8;

    /// Write this section's header and data into the given sink.
    fn encode<S>(&self, sink: &mut S)
    where
        S: Extend<u8>;
}

/// Known section identifiers of WebAssembly components.
///
/// These sections are supported by the component model proposal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum ComponentSectionId {
    /// The section is a custom section.
    Custom = 0,
    /// The section is a type section.
    Type = 1,
    /// The section is an import section.
    Import = 2,
    /// The section is a function section.
    Function = 3,
    /// The section is a module section.
    Module = 4,
    /// The section is a component section.
    Component = 5,
    /// The section is an instance section.
    Instance = 6,
    /// The section is an export section.
    Export = 7,
    /// The section is a start section.
    Start = 8,
    /// The section is an alias section.
    Alias = 9,
}

impl From<ComponentSectionId> for u8 {
    #[inline]
    fn from(id: ComponentSectionId) -> u8 {
        id as u8
    }
}

/// Represents a WebAssembly component that is being encoded.
///
/// Unlike core WebAssembly modules, the sections of a component
/// may appear in any order and may be repeated.
#[derive(Clone, Debug)]
pub struct Component {
    bytes: Vec<u8>,
}

impl Component {
    /// Begin writing a new `Component`.
    pub fn new() -> Self {
        Self {
            bytes: vec![
                0x00, 0x61, 0x73, 0x6D, // magic (`\0asm`)
                0x0a, 0x00, 0x01, 0x00, // version
            ],
        }
    }

    /// Finish writing this component and extract ownership of the encoded bytes.
    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }

    /// Write a section to this component.
    pub fn section(&mut self, section: &impl ComponentSection) -> &mut Self {
        self.bytes.push(section.id());
        section.encode(&mut self.bytes);
        self
    }
}

impl Default for Component {
    fn default() -> Self {
        Self::new()
    }
}
