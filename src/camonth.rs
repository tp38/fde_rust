//! # camonth :
//! Ce module définit la gestion d'un CaMonth représentant l'acttivité sur un mois donné.
//! - author : Thierry Probst <thierry.probst@free.fr>
//! - version : 1.0.0
//! - date : 23/04/2023
use crate::ca::Ca;
use crate::cadaosqlite::CaDaoSqlite;
use rusqlite::{Result};
use chrono::NaiveDate;



#[derive(Debug)]
pub struct CaMonth {
    ///le jour permettant de définir le mois
    pub day: NaiveDate,
    /// le chiffre d'affaire du mois
    pub ca: f64,
    /// le nombre d'heures
    pub hours: f64,
    /// le nombre d'heures supplémentaires
    pub hsup: f64,
    /// le détail de chaque jour du mois
    pub datas: Vec<Ca>,
}


impl CaMonth {
    /// récupère les données du mois et complète la structure CaMonth
    /// - params :
    ///     - self -> la strucure concernée
    ///     - url -> un objet String représentant l'url du fichier sqlite3
    /// - return : uniquement les erreurs sqlite3 éventuelles via une structure Result
    pub fn retrieve_datas(&mut self, url: String) -> Result<()> {
        let sqlite = CaDaoSqlite{ url: url.to_string() };

        self.ca = sqlite.ca_for_month( &self.day )?;
        self.hours = sqlite.hours_for_month( &self.day )?;
        self.hsup = sqlite.hsup_for_month( &self.day )?;
        self.datas = sqlite.datas_for_month( &self.day )?;
        Ok(())
    }

}
