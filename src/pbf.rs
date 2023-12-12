use anyhow::Result;
use osmpbf::ElementReader;

use crate::*;

pub fn parse_pbf<F: FnMut(Element)>(input_bytes: &[u8], mut callback: F) -> Result<()> {
    let reader = ElementReader::new(input_bytes);
    reader.for_each(|element| {
        match element {
            osmpbf::Element::Node(node) => {
                let id = NodeID(node.id());
                let mut tags = HashMap::new();
                for (k, v) in node.tags() {
                    tags.insert(k.to_string(), v.to_string());
                }

                let lon = node.lon();
                let lat = node.lat();
                callback(Element::Node { id, lon, lat, tags });
            }
            osmpbf::Element::DenseNode(node) => {
                let id = NodeID(node.id());
                let mut tags = HashMap::new();
                for (k, v) in node.tags() {
                    tags.insert(k.to_string(), v.to_string());
                }

                let lon = node.lon();
                let lat = node.lat();
                callback(Element::Node { id, lon, lat, tags });
            }
            osmpbf::Element::Way(way) => {
                let id = WayID(way.id());
                let mut tags = HashMap::new();
                for (k, v) in way.tags() {
                    tags.insert(k.to_string(), v.to_string());
                }

                let mut node_ids = Vec::new();
                for id in way.refs() {
                    node_ids.push(NodeID(id));
                }
                callback(Element::Way { id, node_ids, tags });
            }
            osmpbf::Element::Relation(relation) => {
                let id = RelationID(relation.id());
                let mut tags = HashMap::new();
                for (k, v) in relation.tags() {
                    tags.insert(k.to_string(), v.to_string());
                }

                let mut members = Vec::new();
                for member in relation.members() {
                    // TODO plumb error
                    let role = member.role().unwrap();
                    let id = match member.member_type {
                        osmpbf::elements::RelMemberType::Node => {
                            OsmID::Node(NodeID(member.member_id))
                        }
                        osmpbf::elements::RelMemberType::Way => OsmID::Way(WayID(member.member_id)),
                        osmpbf::elements::RelMemberType::Relation => {
                            OsmID::Relation(RelationID(member.member_id))
                        }
                    };
                    members.push((role.to_string(), id));
                }
                callback(Element::Relation { id, tags, members });
            }
        }
    })?;
    Ok(())
}
