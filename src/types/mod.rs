use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct County {
    pub region_name: String,
    pub state_fips: String,
    pub fips: String,
    pub county_fips: String,
    pub do_name: String,
    pub place_name: String,
    pub series: Series
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub qcew: Vec<Qcew>,
    pub laus: Vec<Laus>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Qcew {
    pub year: u32,
    pub qtr: String,
    pub annual_avg_estabs: u32,
    pub annual_avg_wkly_wage: u32,
    pub annual_avg_emplvl: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Laus {
    pub year: u32,
    pub month: u32,
    pub unemployed: u32,
    pub unemployment_rate: f32,
    pub labor_force: u32,
    pub employed: u32
}

#[derive(Deserialize)]
pub struct Info {
    pub fips: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub data: Option<County>,
}