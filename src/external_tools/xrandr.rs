use anyhow::Result;
use std::collections::BTreeMap;

use crate::models::{Monitor, MonitorGrid};

use super::utils::get_command_output;

type MonitorConfig = String;
type ParsedMonitorConfig = (String, i32, i32); // (dimensions, x_offset, y_offset)

pub fn get_monitor_grid() -> Result<MonitorGrid> {
    let raw_monitors = get_raw_monitors_config();
    let parsed_monitors_grid = parse_raw_monitors_config(&raw_monitors)?;

    Ok(MonitorGrid::new(parsed_monitors_grid))
}

/// Sample output:
///
/// [
///     "DisplayPort-0 connected 3440x1440+1920+540 (normal left inverted right x axis y axis) 800mm x 337mm",
///     "DisplayPort-1 connected 1440x2560+5360+0 right (normal left inverted right x axis y axis) 597mm x 336mm",
///     "DisplayPort-2 connected 1920x1080+0+0 (normal left inverted right x axis y axis) 527mm x 296mm",
///     "HDMI-A-0 connected primary 1920x1080+0+1080 (normal left inverted right x axis y axis) 527mm x 296mm"
/// ]
fn get_raw_monitors_config() -> Vec<MonitorConfig> {
    let output = get_command_output(&["xrandr"]).trim().to_owned();

    output
        .split("\n")
        .filter(|line| line.contains(" connected "))
        .map(|line| line.to_owned())
        .collect()
}

fn parse_raw_monitors_config(raw_monitors: &[MonitorConfig]) -> Result<Vec<Vec<Monitor>>> {
    // Parse the xrandr output.
    let mut monitor_configs: Vec<ParsedMonitorConfig> = raw_monitors
        .iter()
        .map(parse_monitor_config)
        .collect::<Result<Vec<ParsedMonitorConfig>>>()?;

    // Sort monitors by x_offset and then by y_offset.
    monitor_configs.sort_by_key(|&(_, x_offset, y_offset)| (x_offset, y_offset));

    // Create a BTreeMap to hold columns.
    let mut columns: BTreeMap<i32, Vec<(String, i32)>> = BTreeMap::new();

    for (dimensions, x_offset, y_offset) in monitor_configs {
        columns
            .entry(x_offset)
            .or_default()
            .push((dimensions, y_offset));
    }

    // Sort each column by y_offset.
    for column in columns.values_mut() {
        column.sort_by_key(|&(_, y_offset)| y_offset);
    }

    // Convert the BTreeMap to a 2D array.
    let grid: Vec<Vec<Monitor>> = columns
        .into_values()
        .map(|column| {
            column
                .into_iter()
                .map(|(dimensions, _)| Monitor::from_string_dimensions(&dimensions))
                .collect::<Result<Vec<Monitor>>>()
        })
        .collect::<Result<Vec<Vec<Monitor>>>>()?;

    Ok(grid)
}

fn parse_monitor_config(monitor_config: &MonitorConfig) -> Result<ParsedMonitorConfig> {
    let config_parts: Vec<&str> = monitor_config.split_whitespace().collect();

    if config_parts.len() < 3 {
        return Err(anyhow::anyhow!("Invalid monitor config: {monitor_config}"));
    }

    let position_index = if config_parts[2] == "primary" { 3 } else { 2 };

    if let Some(position) = config_parts.get(position_index) {
        let offsets: Vec<&str> = position.split('+').collect();

        if offsets.len() != 3 {
            return Err(anyhow::anyhow!("Invalid monitor config: {monitor_config}"));
        }

        let dimensions = offsets[0].to_owned();
        let x_offset = offsets[1].parse::<i32>()?;
        let y_offset = offsets[2].parse::<i32>()?;

        Ok((dimensions, x_offset, y_offset))
    } else {
        Err(anyhow::anyhow!("Invalid monitor config: {monitor_config}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_raw_monitors_config {
        use super::*;

        #[test]
        fn test_can_parse_quad_monitor_config() {
            let mock_config = vec![
                "DisplayPort-0 connected 3440x1440+1920+540 (normal left inverted right x axis y axis) 800mm x 337mm".to_owned(),
                "DisplayPort-1 connected 1440x2560+5360+0 right (normal left inverted right x axis y axis) 597mm x 336mm".to_owned(),
                "DisplayPort-2 connected 1920x1080+0+0 (normal left inverted right x axis y axis) 527mm x 296mm".to_owned(),
                "HDMI-A-0 connected primary 1920x1080+0+1080 (normal left inverted right x axis y axis) 527mm x 296mm".to_owned()
            ];

            let monitor_grid = parse_raw_monitors_config(&mock_config).unwrap();

            assert_eq!(
                monitor_grid,
                vec![
                    vec![Monitor::new(1920, 1080), Monitor::new(1920, 1080)],
                    vec![Monitor::new(3440, 1440)],
                    vec![Monitor::new(1440, 2560)],
                ]
            );
        }
    }
}
