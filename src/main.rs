use rusqlite::{Connection, Result};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let connection = Connection::open("database.db")?;
    
    connection.execute(
        "create table if not exists lantus (
            id integer primary key,
            date default (datetime('now','localtime')),
            units integer
        )",
        [],
    )?;

    let mut buffer = String::new();
    
    println!("Cuantas unidades de lantus te vas a poner?");
    
    io::stdin().read_line(&mut buffer)?;
    
    // if theres a record for today, and the user is adding a record on the same day, throw an error
    // if theres a record for today, and the user is adding a record on a different day, update the record
    // else insert a new record
 
    let units = &buffer.trim().parse::<i32>().unwrap();
    let today_query = "select * from lantus where date = datetime('now','localtime')";
    let today_result = connection.query_row(today_query, [], |row| {
        let date: String = row.get(1)?;
        let units: i32 = row.get(2)?;
        Ok((date, units))
    });
    println!("{:?}", today_result);
    match today_result {
        Ok((date, units)) => {
            if date == format!("{}", chrono::Local::today().format("%Y-%m-%d %H:%M:%S")) {
                println!("Ya hay una entrada para hoy");
                println!("{}", units);
            } else {
                let insert_query = "insert into lantus (date, units) values (datetime('now','localtime'), ?)";
                connection.execute(insert_query, [units])?;
                println!("{}", units);
            }
        }
        Err(_) => {
            let insert_query = "insert into lantus (date, units) values (datetime('now','localtime'), ?)";
            connection.execute(insert_query, [units])?;
            println!("{}", "Still getting into this query :(");
        }
    }
    Ok(())
}    
