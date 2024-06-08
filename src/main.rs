use std::io::Write;
use std::{collections::HashMap, error::Error};

mod person;
use csv::ReaderBuilder;
use person::*;

mod personnummer;

mod error;

mod ui;
use ui::*;

struct Csv(Vec<HashMap<String, String>>);

fn main() -> Result<(), Box<dyn Error>> {
    // Låt användaren välja en csv fil.
    let file = get_csv().ok_or("Ingen fil vald").inspect_err(|_| {
        display_dialog("Error", "Ingen fil vald");
    })?;
    let filename = file.to_str().ok_or("Ogiltigt filnamn")?;

    let csv = Csv::from_file(filename)?;

    // Se till att filen innehåller kolumnerna 'namn' och 'personnummer'.
    if !csv.contains_rows(&["namn", "personnummer"]) {
        dbg!(csv.rows().next());
        display_dialog("Error", "Filen saknar kolumnerna 'namn' och 'personnummer'");
        return Err("Filen saknar kolumnerna 'namn' och 'personnummer'".into());
    }

    // Skapa personer från csv filen.
    let people = csv
        .rows()
        .map(|line| {
            let name = line.get("namn").unwrap();
            let pin = line.get("personnummer").unwrap();
            Person::new(name, pin)
        })
        .collect::<Vec<_>>();

    // Filtrera personer över 18 år med giltiga födelsedatum.
    let over_18: Vec<_> = people
        .iter()
        .filter(|p| p.age.is_ok() && p.age.as_ref().unwrap() >= &18)
        .map(|p| p.name.clone() + " - " + &p.age.as_ref().unwrap().to_string() + " år")
        .collect::<Vec<_>>();

    // Filtrera personer med ogiltiga personnummer eller födelsedatum.
    let invalid_pins = people
        .iter()
        .filter(|p| !personnummer::validate_pin(&p.pin) || p.age.is_err())
        .map(|p| p.pin.clone() + " - " + &p.name)
        .collect::<Vec<_>>();

    let question = format!(
        "Det finns {} personer över 18 år, vill du skriva till en fil?",
        over_18.len()
    );

    // Fråga användaren om de vill skriva personer över 18 år till en fil.
    if !over_18.is_empty() && display_option("Personer över 18", &question) {
        let Some((file_path, mut file)) = save_file("personer_över_18.txt") else {
            display_dialog("Error", "Ingen fil vald");
            return Err("Ingen fil vald".into());
        };

        for person in over_18 {
            file.write_all(person.as_bytes())?;
            file.write_all(b"\n")?;
        }
        display_dialog(
            "Personer över 18",
            &format!(
                "Personer över 18 skrivna till filen '{}'",
                file_path.file_name().unwrap().to_str().unwrap()
            ),
        );
    } else if !over_18.is_empty() {
        display_dialog("Personer över 18", &over_18.join("\n"));
    } else {
        display_dialog("Personer över 18", "Inga personer över 18 år");
    }

    let question = format!(
        "Det finns {} personer med ogiltiga personnummer, vill du skriva till en fil?",
        invalid_pins.len()
    );

    // Fråga användaren om de vill skriva ogiltiga personnummer till en fil.
    if !invalid_pins.is_empty() && display_option("Personer med ogiltiga personnummer", &question) {
        let Some((file_path, mut file)) = save_file("ogiltiga_personnummer.txt") else {
            display_dialog("Error", "Ingen fil vald");
            return Err("Ingen fil vald".into());
        };

        for person in invalid_pins {
            file.write_all(person.as_bytes())?;
            file.write_all(b"\n")?;
        }
        display_dialog(
            "Personer med ogiltiga personnummer",
            &format!(
                "Personer över med ogiltiga personnummer skrivna till filen '{}'",
                file_path.file_name().unwrap().to_str().unwrap()
            ),
        );
    } else if !invalid_pins.is_empty() {
        display_dialog(
            "Personer med ogiltiga personnummer",
            &invalid_pins.join("\n"),
        );
    } else {
        display_dialog(
            "Personer med ogiltiga personnummer",
            "Inga personer med ogiltiga personnummer",
        );
    }

    Ok(())
}

impl Csv {
    /// Läs en csv fil och returnera en Csv struct.
    fn from_file(file_path: &str) -> Result<Csv, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_path(file_path)?;
        let mut records = Vec::new();

        let headers = rdr
            .headers()?
            .iter()
            .map(&str::to_string)
            .collect::<Vec<_>>();

        for result in rdr.records() {
            let record = result?;
            let hashmap: HashMap<String, String> = record
                .iter()
                .enumerate()
                .map(|(i, value)| (headers[i].to_string().to_lowercase(), value.to_string()))
                .collect();
            records.push(hashmap);
        }
        Ok(Csv(records))
    }

    /// Returnera en iterator över raderna i csv filen.
    fn rows(&self) -> impl Iterator<Item = &HashMap<String, String>> {
        self.0.iter()
    }

    /// Returnera true om csv filen innehåller en rad med nyckeln `key`.
    fn contains_row(&self, key: &str) -> bool {
        self.0.iter().any(|row| row.contains_key(key))
    }

    /// Returnera true om csv filen innehåller rader med nycklarna `keys`.
    fn contains_rows(&self, keys: &[&str]) -> bool {
        keys.iter().all(|key| self.contains_row(key))
    }
}
