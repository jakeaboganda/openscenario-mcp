use opendrive::core::OpenDrive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Failed to load OpenDRIVE file: {0}")]
    LoadError(String),

    #[error("Road '{0}' not found in OpenDRIVE file")]
    RoadNotFound(String),

    #[error("Lane {1} not found in road '{0}'")]
    LaneNotFound(String, i32),

    #[error("Position {1} is out of bounds for road '{0}' (length: {2})")]
    PositionOutOfBounds(String, f64, f64),

    #[error("Poor quality OpenDRIVE data: {0}")]
    QualityError(String),
}

/// Summary information about a road
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadSummary {
    pub id: String,
    pub name: Option<String>,
    pub length: f64,
    pub lane_count: usize,
}

/// Detailed metadata about a road
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadMetadata {
    pub id: String,
    pub name: Option<String>,
    pub length: f64,
    pub lanes: Vec<LaneInfo>,
}

/// Information about a specific lane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneInfo {
    pub id: i32,
    pub lane_type: String,
}

/// A suggested spawn point for vehicle placement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPoint {
    pub road_id: String,
    pub lane_id: i32,
    pub s: f64,
    pub description: String,
}

/// Quality assessment of OpenDRIVE data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScore {
    pub score: u8, // 0-100
    pub has_lanes: bool,
    pub has_geometry: bool,
    pub has_valid_length: bool,
    pub issues: Vec<String>,
}

pub struct OpenDriveValidator {
    #[allow(dead_code)]
    opendrive: OpenDrive,
    road_cache: HashMap<String, RoadInfo>,
}

struct RoadInfo {
    length: f64,
    lane_ids: Vec<i32>,
}

impl OpenDriveValidator {
    /// Load an OpenDRIVE file and create a validator
    pub fn load(path: &Path) -> Result<Self, ValidationError> {
        let file = std::fs::File::open(path)
            .map_err(|e| ValidationError::LoadError(format!("Failed to open file: {}", e)))?;

        let opendrive = OpenDrive::from_xml_read(file).map_err(|e| {
            ValidationError::LoadError(format!("Failed to parse OpenDRIVE XML: {}", e))
        })?;

        let mut road_cache = HashMap::new();
        for road in &opendrive.road {
            let mut lane_ids = Vec::new();

            // Collect all lane IDs from all lane sections
            for lane_section in road.lanes.lane_section.iter() {
                // Add left lanes
                if let Some(left) = &lane_section.left {
                    for lane in &left.lane {
                        // Cast i64 to i32 (OpenDRIVE spec keeps lane IDs in i32 range)
                        let lane_id = lane.id as i32;
                        if !lane_ids.contains(&lane_id) {
                            lane_ids.push(lane_id);
                        }
                    }
                }

                // Add center lanes
                for lane in &lane_section.center.lane {
                    let lane_id = lane.id as i32;
                    if !lane_ids.contains(&lane_id) {
                        lane_ids.push(lane_id);
                    }
                }

                // Add right lanes
                if let Some(right) = &lane_section.right {
                    for lane in &right.lane {
                        let lane_id = lane.id as i32;
                        if !lane_ids.contains(&lane_id) {
                            lane_ids.push(lane_id);
                        }
                    }
                }
            }

            road_cache.insert(
                road.id.clone(),
                RoadInfo {
                    length: road.length.value,
                    lane_ids,
                },
            );
        }

        Ok(Self {
            opendrive,
            road_cache,
        })
    }

    /// Check if a road exists in the OpenDRIVE file
    pub fn road_exists(&self, road_id: &str) -> bool {
        self.road_cache.contains_key(road_id)
    }

    /// Validate a position against the OpenDRIVE road network
    pub fn validate_position(&self, road_id: &str, s: f64) -> Result<(), ValidationError> {
        self.validate_road_position(road_id, s)
    }

    /// Validate a lane position (road + lane ID)
    pub fn validate_lane_position(
        &self,
        road_id: &str,
        lane_id: i32,
    ) -> Result<(), ValidationError> {
        let road_info = self
            .road_cache
            .get(road_id)
            .ok_or_else(|| ValidationError::RoadNotFound(road_id.to_string()))?;

        if !road_info.lane_ids.contains(&lane_id) {
            return Err(ValidationError::LaneNotFound(road_id.to_string(), lane_id));
        }

        Ok(())
    }

    /// Validate a road position (road + s-coordinate)
    pub fn validate_road_position(&self, road_id: &str, s: f64) -> Result<(), ValidationError> {
        // Check for NaN and infinity
        if !s.is_finite() {
            return Err(ValidationError::PositionOutOfBounds(
                road_id.to_string(),
                s,
                0.0, // length is irrelevant for NaN/infinity
            ));
        }

        let road_info = self
            .road_cache
            .get(road_id)
            .ok_or_else(|| ValidationError::RoadNotFound(road_id.to_string()))?;

        if s < 0.0 || s > road_info.length {
            return Err(ValidationError::PositionOutOfBounds(
                road_id.to_string(),
                s,
                road_info.length,
            ));
        }

        Ok(())
    }

    /// List all roads in the network
    pub fn list_roads(&self) -> Vec<RoadSummary> {
        self.opendrive
            .road
            .iter()
            .map(|road| RoadSummary {
                id: road.id.clone(),
                name: road.name.clone(),
                length: road.length.value,
                lane_count: self
                    .road_cache
                    .get(&road.id)
                    .map(|info| info.lane_ids.len())
                    .unwrap_or(0),
            })
            .collect()
    }

    /// Get detailed information about a specific road
    pub fn get_road_info(&self, road_id: &str) -> Option<RoadMetadata> {
        let road = self.opendrive.road.iter().find(|r| r.id == road_id)?;

        let road_info = self.road_cache.get(road_id)?;

        let lanes = road_info
            .lane_ids
            .iter()
            .map(|&id| LaneInfo {
                id,
                lane_type: self.get_lane_type_name(road_id, id),
            })
            .collect();

        Some(RoadMetadata {
            id: road.id.clone(),
            name: road.name.clone(),
            length: road.length.value,
            lanes,
        })
    }

    /// Suggest valid spawn points for vehicles
    pub fn suggest_spawn_points(
        &self,
        road_id: &str,
        count: usize,
    ) -> Result<Vec<SpawnPoint>, ValidationError> {
        let road_info = self
            .road_cache
            .get(road_id)
            .ok_or_else(|| ValidationError::RoadNotFound(road_id.to_string()))?;

        // Filter to driving lanes only (negative IDs are typically driving lanes)
        let driving_lanes: Vec<i32> = road_info
            .lane_ids
            .iter()
            .copied()
            .filter(|&id| id < 0) // Negative = driving lanes in OpenDRIVE
            .collect();

        if driving_lanes.is_empty() {
            return Ok(Vec::new());
        }

        let mut points = Vec::new();

        // Distribute vehicles evenly along the road
        let spacing = road_info.length / (count as f64 + 1.0);

        for i in 0..count {
            let s = spacing * (i as f64 + 1.0);
            let lane_id = driving_lanes[i % driving_lanes.len()];

            points.push(SpawnPoint {
                road_id: road_id.to_string(),
                lane_id,
                s,
                description: format!(
                    "Lane {} at {:.1}m ({}%)",
                    lane_id,
                    s,
                    (s / road_info.length * 100.0) as u32
                ),
            });
        }

        Ok(points)
    }

    /// Assess quality of OpenDRIVE data
    pub fn assess_quality(&self) -> QualityScore {
        let mut score = 100u8;
        let mut issues = Vec::new();

        // Check if there are any roads
        let has_roads = !self.opendrive.road.is_empty();
        if !has_roads {
            score = 0;
            issues.push("No roads found".to_string());
            return QualityScore {
                score,
                has_lanes: false,
                has_geometry: false,
                has_valid_length: false,
                issues,
            };
        }

        let has_lanes = self.opendrive.road.iter().any(|r| {
            r.lanes
                .lane_section
                .iter()
                .any(|ls| ls.left.is_some() || ls.right.is_some() || !ls.center.lane.is_empty())
        });

        let has_geometry = self
            .opendrive
            .road
            .iter()
            .all(|r| !r.plan_view.geometry.is_empty());

        let has_valid_length = self
            .opendrive
            .road
            .iter()
            .all(|r| r.length.value > 0.0 && r.length.value < 100000.0);

        if !has_lanes {
            score -= 40;
            issues.push("Missing lane information".to_string());
        }

        if !has_geometry {
            score -= 30;
            issues.push("Missing geometry data".to_string());
        }

        if !has_valid_length {
            score -= 20;
            issues.push("Invalid road lengths detected".to_string());
        }

        // Check for very short roads (likely errors)
        let short_roads = self
            .opendrive
            .road
            .iter()
            .filter(|r| r.length.value < 10.0)
            .count();
        if short_roads > 0 {
            score = score.saturating_sub(10);
            issues.push(format!("{} roads are very short (<10m)", short_roads));
        }

        QualityScore {
            score,
            has_lanes,
            has_geometry,
            has_valid_length,
            issues,
        }
    }

    /// Helper: Get lane type name
    fn get_lane_type_name(&self, _road_id: &str, lane_id: i32) -> String {
        // Simplified lane type detection based on ID
        // In OpenDRIVE: negative IDs = driving lanes, 0 = center, positive = opposite direction
        if lane_id < 0 {
            "driving".to_string()
        } else if lane_id == 0 {
            "center".to_string()
        } else {
            "driving_opposite".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_validator_creation() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/test_road.xodr");

        let result = OpenDriveValidator::load(&path);
        assert!(result.is_ok());
    }
}
