use std::collections::HashSet;

use plotters::{
    backend::SVGBackend,
    chart::ChartBuilder,
    coord::ranged1d::IntoSegmentedCoord,
    drawing::IntoDrawingArea,
    element::Rectangle,
    series::Histogram,
    style::{Color, GREEN, RED, WHITE},
};
use venx_core::{glam::UVec3, plat::layer::layer::Lr};

use super::VenxPlat;

impl VenxPlat {
    pub(super) fn chart_node_destibution(&self, path: &str, plat_name: &str) -> anyhow::Result<()> {
        let root = SVGBackend::new(path, (640, 480)).into_drawing_area();

        let mut visited_nodes = HashSet::new();

        let rplat = self.get_normal_unchecked().borrow_raw_plat();

        let mut branches = vec![0; rplat.depth()];
        let mut compact_forks = 0;
        let mut forks = 0;
        let mut free = 0;
        let mut merged_on_level_3 = 0;

        let lr = &rplat[Lr::BASE];

        let mut non_zero_flags = vec![];
        let mut g_counter = 0;

        lr.traverse(UVec3::ZERO, 0..=(rplat.depth()), |p| {
            if p.level > 2 {
                let node = &lr[p.node_idx];
                if node.flag != 0 {
                    non_zero_flags.push(node.flag);
                }
                if !visited_nodes.contains(&p.node_idx) {
                    g_counter += 1;
                    visited_nodes.insert(p.node_idx);

                    branches[p.level as usize] += 1;
                } else if p.level == 3 {
                    merged_on_level_3 += 1;
                }
            }
        });

        for node in lr.nodes.iter() {
            if node.flag == -3 {
                assert!(node.is_fork());
                compact_forks += 1;
            } else if node.flag > 0 {
                assert!(node.is_fork());
                forks += 1;
            } else if node.flag == -1 {
                free += 1;
            }
        }

        // Validation
        {
            assert_eq!(free, lr.free() + 1);
            assert_ne!(forks, 0);
            assert_ne!(compact_forks, 0);

            let mut total = 0;

            for level in &branches {
                total += level;
            }

            total += forks;
            total += compact_forks;
            total += free;
            // FIXME
            // assert_eq!(total, lr.nodes.len());
        }

        dbg!(
            merged_on_level_3,
            compact_forks,
            forks,
            lr.level_2.len(),
            lr.free_l2(),
            lr.nodes.len(),
            lr.free(),
            g_counter,
            &non_zero_flags[0..50],
        );

        let max = 1_500_000;

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(35)
            .caption(
                format!("Node amounts in merged DAG ({plat_name}) on Base"),
                ("sans-serif", 30.0),
            )
            .build_cartesian_2d(
                (0u32..((self.depth() as u32 + 1) / 2)).into_segmented(),
                0u32..(max + 10000),
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(WHITE.mix(0.3))
            .y_desc("Node Amount")
            .x_desc("Level")
            //.y_label_offset(100)
            .y_max_light_lines(2)
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        // Draw Branches
        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .style(RED.mix(0.5).filled())
                    // .baseline_func(|v| 1)
                    .data(
                        branches
                            .iter()
                            .skip(0)
                            .enumerate()
                            .map(|(level, amount)| (level as u32, *amount as u32)),
                    ),
            )?
            .label("Unmerged (Unique) Nodes")
            .legend(|(x, y)| {
                Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], RED.mix(0.5).filled())
            });

        //  chart.draw_series()
        //   cc.draw_series(LineSeries::new(
        //     (0..).zip(data.iter()).map(|(a, b)| (a, *b)),
        //     &Palette99::pick(idx),
        // ))?
        // .label(format!("CPU {}", idx))
        // .legend(move |(x, y)| {
        //     Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(idx))
        //     });
        // chart
        //     .draw_secondary_series(actual)?
        //     .label("Observed")
        //     .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], GREEN.filled()));

        // chart
        //     .draw_series(
        //         Histogram::vertical(&chart)
        //             .style(GREEN.mix(0.5).filled())
        //             // .baseline_func(|v| 1)
        //             .data(
        //                 merged_amounts
        //                     .iter()
        //                     .enumerate()
        //                     .map(|(level, amount)| (level as u32, *amount as u32)),
        //             ),
        //     )?
        //     .label("Merged Nodes")
        //     .legend(|(x, y)| {
        //         Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], GREEN.mix(0.5).filled())
        //     });
        chart.configure_series_labels().draw()?;
        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

        Ok(())
    }
}
