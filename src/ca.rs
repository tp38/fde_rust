//! # ca :
//! Ce module définit une structure Ca représentant une journée de travail.
//! Cela correspondant à un enregistrement de la table CA du fichier Sqlite3
//! - author : Thierry Probst <thierry.probst@free.fr>
//! - version : 1.0.0
//! - date : 23/04/2023
use std::fmt;
use chrono::NaiveDate;
use crate::CaDaoSqlite;
use crate::Dialog;
use crate::DB_URL;


#[derive(Debug)]
pub struct Ca {
    /// la date du jour concerné
    pub date: String,
    /// le chiffre d'affaire réalisé
    pub ca: f32,
    /// le nombre d'heure travaillé
    pub hours: f32,
    /// le nombre d'heure suppl"supplémentaires réalisées
    pub hsup: f32,
    /// un éventuel commentaire lié à la journée
    pub comment: Option<String>,
}

impl Ca {
    /// new : crée un objet chiffre d'affaire 'Ca' à partir date. L'objet est recherché dans la
    /// base sqlite3. Si il est trouvé, il est rapatrié en vue de son utilisation si non ce sont
    /// des valeurs génériques (day, 0.0 et None) qui sont utilisées pour initialiser les différents
    /// membres
    /// - params : day -> la date du chiffre d'affaire
    /// - return : un objet Ca
    pub fn new( day: &NaiveDate) -> Ca {
        let sqlite = CaDaoSqlite{ url: DB_URL.to_string() };
        let mut c = Ca{ date: day.to_string(), ca: 0.0, hours: 0.0, hsup: 0.0, comment: None };
        match sqlite.exist( day ) {
            Ok(exist) =>
                if exist { // on récupère les données
                    match sqlite.get( day ) {
                        Ok(ca) => c = ca,
                        Err(e) => Dialog::something_goes_wrong( "cadaoslite:get", e ),
                    }
                } else { // on sauvegarde le template
                    match sqlite.add( &c ) {
                        Ok(()) => println!( "{} a été créé dans la DB", c ),
                        Err(e) => Dialog::something_goes_wrong( "cadaosqlite::add", e),
                    }
                },
            Err(e) => Dialog::something_goes_wrong( "cadaosqlite:exist", e ),
        }
        return c;
    }

    /// delete : suppresion d'un objet ca dans la base de données sqlite3
    /// - params : l'objet courant qui appelle la suppression
    /// - return : aucun
    pub fn delete(self) -> () {
        let sqlite = CaDaoSqlite{ url: DB_URL.to_string() };
        match sqlite.del( self ) {
            Ok(()) => (),
            Err(e) => Dialog::something_goes_wrong("cadaosqlite::del", e),
        }
    }

    /// save : enregistrement de l'objet Ca dans la base de données sqlite3
    /// - params : l'objet courant qui appelle l'enregistrement
    /// - return : aucun
    pub fn save( &self) -> () {
        let sqlite = CaDaoSqlite{ url: DB_URL.to_string() };
        match sqlite.update( &self ) {
            Ok(()) => (),
            Err(e) => Dialog::something_goes_wrong("cadaosqlite::update", e),
        }
    }
}

impl fmt::Display for Ca {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({} : {} [{}/{} '{:?}'])", self.date, self.ca, self.hours, self.hsup, self.comment)
    }

}
