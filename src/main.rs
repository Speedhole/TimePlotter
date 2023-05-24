
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
//use std::io;
use csv::ReaderBuilder;
use serde::Deserialize;
//use plotters::prelude::*;

// #[derive(Debug, Deserialize)]
// struct TimeSheetEntry {
//     Supervisor: String,
//     Date: String,
//     Hours: f32,
//     Name: String,
//     JobTitle: String,
//     Paycode: String,
//     PaycodeDescription: String,
//     WorkOrder: String,
//     Transaction: String,
//     Department: String,
//     Division: String,
// }

#[derive(Debug, Clone, Copy)]
struct MonthData {
    month: i32,
    hours: f32,
}



fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("CID_Jan.csv")?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::Headers)
        .from_reader(file);

   

    type Record = HashMap<String, String>;

    let mut year_data = [
        MonthData{month: 1, hours: 0.0}, 
        MonthData{month: 2, hours: 0.0},
        MonthData{month: 3, hours: 0.0},
        MonthData{month: 4, hours: 0.0},
        MonthData{month: 5, hours: 0.0}, 
        MonthData{month: 6, hours: 0.0},
        MonthData{month: 7, hours: 0.0},
        MonthData{month: 8, hours: 0.0}, 
        MonthData{month: 9, hours: 0.0},
        MonthData{month: 10, hours: 0.0},
        MonthData{month: 11, hours: 0.0}, 
        MonthData{month: 12, hours: 0.0},];

    let mut total_time_per_gl = HashMap::new();

    //for result in reader.deserialize::<TimeSheetEntry>() {
    for result in  reader.deserialize() {
        let record: Record = result?;
        // for (key, value) in &record {
        //     println!("{}: {}", key, value);
        // }

        // Extract the month from the date
        let month :i32 = record.get("Date")
            .expect("Date not found in record")
            .split('-')
            .nth(1)
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        println!("Month= {}", month);

        // Calculate the total time per month for each WorkOrder
        let total_time = total_time_per_gl.entry(record.get("Work Order Title").expect("WorkOrder not found").clone()).or_insert(year_data.clone());
        let mut month_data: MonthData = *total_time.get(usize::try_from(month-1).unwrap()).expect("Could not find MonthData");
        month_data.hours += record.get("Hours").unwrap().parse::<f32>().unwrap();
        // total_time_per_gl.entry(record.get("Work Order Title").expect("WorkOrder not found").clone()).or_insert(year_data.clone())
        //     .get(usize::try_from(month-1).unwrap()).expect("Could not find MonthData").hours += record.get("Hours").unwrap().parse::<f32>().unwrap();
        
        
        
        
        for (key, value) in &total_time_per_gl {
            println!("{}: {}", key, value[0].hours);
            println!("{}: {}", key, value[1].hours);
            println!("{}: {}", key, value[2].hours);
            println!("{}: {}", key, value[3].hours);
            println!("{}: {}", key, value[4].hours);
            println!("{}: {}", key, value[5].hours);
            println!("{}: {}", key, value[6].hours);
            println!("{}: {}", key, value[7].hours);
            println!("{}: {}", key, value[8].hours);
            println!("{}: {}", key, value[9].hours);
            println!("{}: {}", key, value[10].hours);
            println!("{}: {}", key, value[11].hours);
        }
    }

    // for (key, value) in &total_time_per_gl {
    //         println!("{}: {}", key, value[0].hours);
    //         println!("{}: {}", key, value[1].hours);
    //         println!("{}: {}", key, value[2].hours);
    //         println!("{}: {}", key, value[3].hours);
    //         println!("{}: {}", key, value[4].hours);
    //         println!("{}: {}", key, value[5].hours);
    //         println!("{}: {}", key, value[6].hours);
    //         println!("{}: {}", key, value[7].hours);
    //         println!("{}: {}", key, value[8].hours);
    //         println!("{}: {}", key, value[9].hours);
    //         println!("{}: {}", key, value[10].hours);
    //         println!("{}: {}", key, value[11].hours);
    //     }

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
