use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
//use std::io;
use csv::ReaderBuilder;
use serde::Deserialize;
//use plotters::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("CID_Jan.csv")?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::Headers)
        .from_reader(file);

    type Record = HashMap<String, String>;

    //this is a hashmap with the gl/month tuple as a key,
    //the value is the number of hours
    let mut total_time_per_gl: HashMap<(String, i32), f32> = HashMap::new();

    //for result in reader.deserialize::<TimeSheetEntry>() {
    for result in reader.deserialize() {
        let record: Record = result?;
        // for (key, value) in &record {
        //     println!("{}: {}", key, value);
        // }

        // Extract the month from the date
        let month: i32 = record
            .get("Date")
            .expect("Date not found in record")
            .split('-')
            .nth(1)
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();

        let mut work_order = record.get("Work Order Title").unwrap().to_string();

        //some lines do not have a proper code as they are "special".
        //in these cases, we need to take the name from the paycode
        // TODO: remove when we have proper GL codes
        if work_order == "" {
            work_order = record
                .get("Paycode Description")
                .expect("Could not find Paycode Description in record")
                .as_str()
                .trim()
                .to_string();

            //remove all whitespace
            work_order = work_order
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("");

            println!("stripped work order = {}", work_order)
        }

        let mut total_time = *total_time_per_gl
            .entry((work_order.clone(), month.clone()).clone())
            .or_insert(0.0);
        total_time = total_time + record.get("Hours").unwrap().parse::<f32>().unwrap();
        total_time_per_gl.insert((work_order, month), total_time);
    }

    for (key, value) in &total_time_per_gl {
        println!("GL={} Month={}: Hours={}", key.0, key.1, value);
    }

    // // Generate the plot
    // let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    // root.fill(&WHITE)?;

    // let work_orders: Vec<&String> = total_time_per_gl.keys().collect();
    // let times: Vec<f32> = total_time_per_gl.values().cloned().collect();

    // let max_time = times.iter().cloned().fold(0.0 / 0.0, f32::max);

    // let mut chart = ChartBuilder::on(&root)
    //     .caption("Time Worked per Month", ("sans-serif", 30).into_font())
    //     .margin(10)
    //     .x_label_area_size(30)
    //     .y_label_area_size(30)
    //     .build_cartesian_2d(0..work_orders.len() as u32, 0.0..max_time)?;

    // chart.configure_mesh().draw()?;

    // chart.draw_series(
    //         work_orders
    //             .iter()
    //             .zip(times.iter())
    //             .enumerate()
    //             .map(|(i, (work_order, time))| {
    //                 let x = i as f32;
    //                 let y = *time;
    //                 let text = Text::new(format!("{:.2}", y), (5 ,5), ("sans-serif", 15));
    //                     //.set_font(("sans-serif", 15))
    //                     //.set_pos((x, y), (5, 5))
    //                     //.into_owned();
    //                 (Rectangle::new([(x - 0.4, 0.0), (x + 0.4, y)], HSLColor(0.3, 0.8, 0.5).filled()), text);
    //             }),
    //     )
    //     .unwrap();

    Ok(())
}
