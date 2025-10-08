use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct TitanicPassenger {
    #[serde(rename = "PassengerId")]
    pub passenger_id: u32,
    #[serde(rename = "Survived")]
    pub survived: u32,
    #[serde(rename = "Pclass")]
    pub pclass: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Sex")]
    pub sex: String,
    #[serde(rename = "Age")]
    pub age: Option<f32>,
    #[serde(rename = "SibSp")]
    pub sib_sp: u32,
    #[serde(rename = "Parch")]
    pub parch: u32,
    #[serde(rename = "Ticket")]
    pub ticket: String,
    #[serde(rename = "Fare")]
    pub fare: f32,
    #[serde(rename = "Cabin")]
    pub cabin: Option<String>,
    #[serde(rename = "Embarked")]
    pub embarked: Option<String>,
}

pub fn process_titanic_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    let mut total_passengers = 0;
    let mut survived_passengers = 0;
    let mut male_passengers = 0;
    let mut female_passengers = 0;

    for result in rdr.deserialize() {
        let passenger: TitanicPassenger = result?;
        total_passengers += 1;

        if passenger.survived == 1 {
            survived_passengers += 1;
        }

        if passenger.sex == "male" {
            male_passengers += 1;
        } else if passenger.sex == "female" {
            female_passengers += 1;
        }
    }

    println!("\n--- Análise de Dados do Titanic ---");
    println!("Total de passageiros processados: {}", total_passengers);
    println!("Passageiros que sobreviveram: {}", survived_passengers);
    println!("Taxa de sobrevivência: {:.2}%", (survived_passengers as f32 / total_passengers as f32) * 100.0);
    println!("Passageiros masculinos: {}", male_passengers);
    println!("Passageiros femininos: {}", female_passengers);
    println!("------------------------------------\n");

    Ok(())
}

