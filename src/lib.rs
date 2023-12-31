mod pbf;
mod xml;

use std::collections::HashMap;
use std::fmt;

use anyhow::Result;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use self::pbf::parse_pbf;
pub use self::xml::parse_xml;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct NodeID(pub i64);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WayID(pub i64);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct RelationID(pub i64);

impl fmt::Display for NodeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "https://www.openstreetmap.org/node/{}", self.0)
    }
}
impl fmt::Display for WayID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "https://www.openstreetmap.org/way/{}", self.0)
    }
}
impl fmt::Display for RelationID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "https://www.openstreetmap.org/relation/{}", self.0)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum OsmID {
    Node(NodeID),
    Way(WayID),
    Relation(RelationID),
}

impl OsmID {
    pub fn inner_id(self) -> i64 {
        match self {
            OsmID::Node(id) => id.0,
            OsmID::Way(id) => id.0,
            OsmID::Relation(id) => id.0,
        }
    }
}

impl fmt::Display for OsmID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OsmID::Node(n) => write!(f, "{}", n),
            OsmID::Way(w) => write!(f, "{}", w),
            OsmID::Relation(r) => write!(f, "{}", r),
        }
    }
}

// TODO Into for both directions

pub enum Element {
    Bounds {
        min_lon: f64,
        min_lat: f64,
        max_lon: f64,
        max_lat: f64,
    },
    Node {
        id: NodeID,
        lon: f64,
        lat: f64,
        tags: HashMap<String, String>,
    },
    Way {
        id: WayID,
        node_ids: Vec<NodeID>,
        tags: HashMap<String, String>,
    },
    Relation {
        id: RelationID,
        tags: HashMap<String, String>,
        // Role, member ID
        members: Vec<(String, OsmID)>,
    },
}

/// Parses bytes representing an OSM XML or PBF file and invokes the callback for every element.
/// The order should be all nodes, then ways, then relations, per
/// <https://wiki.openstreetmap.org/wiki/OSM_XML#Certainties_and_Uncertainties>
pub fn parse<F: FnMut(Element)>(input_bytes: &[u8], callback: F) -> Result<()> {
    if is_xml(input_bytes) {
        parse_xml(input_bytes, callback)
    } else {
        parse_pbf(input_bytes, callback)
    }
}

fn is_xml(input_bytes: &[u8]) -> bool {
    let check_header = "<?xml";
    if input_bytes.len() < check_header.len() {
        return false;
    }
    if let Ok(x) = std::str::from_utf8(&input_bytes[..check_header.len()]) {
        return x.to_lowercase() == check_header;
    }
    false
}
