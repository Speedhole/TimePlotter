use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
//use std::io;
use csv::ReaderBuilder;
use serde::Deserialize;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("CID_Jan.csv")?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::Headers)
        .from_reader(file);

    type Record = HashMap<String, String>;

    //this is a hashmap with the gl/month tuple as a key,
    //the value is the number of hours
    //let mut total_time_per_gl: HashMap<(String, i32), f32> = HashMap::new();
    let mut total_time_per_gl: HashMap<String, HashMap<i32, f32>> = HashMap::new();

    //for result in reader.deserialize::<TimeSheetEntry>() {
    for result in reader.deserialize() {
        let record: Record = result?;

        if !record.contains_key("Date") {
            //we reached the end of the file or an empty line
            println!("we reached an empty line");
            continue;
        }
        else {
            println!("record value = {}", record.contains_key("Date"))
        }
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

        // println!("month = {}", month);

        // TODO: remove everything from .split() once moved to GL codes
        let mut work_order = record.get("Work Order Title").unwrap().to_string().split(' ').nth(0).unwrap().to_string();

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

            //println!("stripped work order = {}", work_order)
        }

        // let mut total_time = *total_time_per_gl
        //     .entry((work_order.clone(), month.clone()).clone())
        //     .or_insert(0.0);
        // total_time = total_time + record.get("Hours").unwrap().parse::<f32>().unwrap();
        // total_time_per_gl.insert((work_order, month), total_time);


        //test code
        let mut data: HashMap<i32,f32> = HashMap::new();
        let _d= data.entry(month.clone()).or_insert(0.0);

        if month == 2 {
            println!("month = 2");
        }

        let mut map = total_time_per_gl.entry(work_order.clone()).or_insert(data);
        //println!("map.keys = {:?}", map.get(&month));
        let mut total_time:f32 = 0.0;
        if map.contains_key(&month){
            total_time = *map.get(&month).unwrap();
        }
        
        total_time = total_time + record.get("Hours").unwrap().parse::<f32>().unwrap();
        map.insert(month, total_time);
    }

    // for (key, value) in &total_time_per_gl {
    //     println!("GL={} Month={}: Hours={}", key.0, key.1, value);
    // }

    //get max hour value
    let mut max_hours: f32 = 0.0;
    for (outer_key, map) in &total_time_per_gl {
        for(inner_key, val) in map {
            if max_hours < *val {
                max_hours = *val;
            }
            println!("GL={} Month={}: Hours={}", outer_key, inner_key, val);
        }
    }


    //get total number of GL codes used
    let gl_count = total_time_per_gl.len();
    println!("Total number of GLs = {} with max value of {}", gl_count, max_hours);

    // // // Generate the plot
    // let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    // root.fill(&WHITE)?;
    // let root = root.margin(10,10,10,10);

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
