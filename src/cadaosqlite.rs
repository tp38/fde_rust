//! # cadaosqlite :
//! Ce module définit un CaDaoSqlite contenant les données communes utilisées pour les requetes
//! SQL d'accès à la table CA (cf. fichier ca.rs pour ce qui concerne la définition de cette table)
//! - author : Thierry Probst <thierry.probst@free.fr>
//! - version : 1.0.0
//! - date : 23/04/2023

use chrono::prelude::*;
use rusqlite::{Connection, Result, named_params };
use crate::ca::Ca;

#[derive(Debug)]
pub struct CaDaoSqlite {
    /// le chemin du fichier sqlite3 qui contient la table CA
    pub url: String,
}

impl CaDaoSqlite {
    /// exist : vérifie si un date est présente dans la table CA.
    /// - params :
    ///     - &self -> une référence sur la structure contenant l'url et la date
    ///     - d -> une référence sur une date à chercher
    /// - return : un booleen
    ///     - true -> la date existe
    ///     - false -> sinon
    pub fn exist(&self, d: &NaiveDate ) -> Result<bool> {
        let mut result = false;
        let conn = Connection::open(&self.url)?;

        let sum: i32 = conn.query_row( "SELECT COUNT(*) FROM CA WHERE date = :day",
            &[ (":day", d.format("%Y-%m-%d").to_string().as_str() )], |row| row.get(0) )?;
        if sum > 0 { result = true; }
        Ok(result)
    }

    /// get : récupère les données correspondant au jour en cours.
    /// - params :
    ///    - &self -> une référence sur la structure contenant l'url et la date
    ///    - d -> une référence sur une date du Ca à récuperer
    /// - return : un objet Ca en cas de succès sinon l'erreur Sqlite3
    pub fn get(&self, d: &NaiveDate ) -> Result<Ca> {
        let conn = Connection::open(&self.url)?;

        let ca: Ca = conn.query_row( "SELECT * FROM CA WHERE date = :day",
            &[ (":day", d.format("%Y-%m-%d").to_string().as_str() )],
            |row| Ok(Ca {
                date: row.get("date")?,
                ca: row.get("ca")?,
                hours: row.get(2)?,
                hsup: row.get(3)?,
                comment: row.get("comment")?,
            })
        )?;
        Ok(ca)
    }

    /// del : supprime le Ca dans la DB.
    /// - params :
    ///     - &self -> une référence sur la structure contenant l'url et la date
    ///     - c -> une structure Ca contenant les données a supprimer
    /// - return : un objet () en cas de succés sinon l'erreur Sqlite3
    pub fn del(&self, c: Ca) -> Result<()> {
        let conn = Connection::open(&self.url)?;

        let mut stmt = conn.prepare( "DELETE FROM CA WHERE date = :date" )?;
        stmt.execute( named_params!{ ":date": c.date } )?;
        Ok(())
    }

    /// update : sauvegarde le Ca dans la DB.
    /// - params :
    ///     - &self -> une référence sur la structure contenant l'url et la date
    ///     - c -> une structure Ca contenant les données a supprimer
    /// - return : un objet () en cas de succés sinon l'erreur Sqlite3
    pub fn update(&self, c: &Ca) -> Result<()> {
        let conn = Connection::open(&self.url)?;

        let mut stmt = conn.prepare( "UPDATE CA
                            SET ca = :ca, hours = :hours, hsup = :hsup, comment = :comment
                            WHERE date = :date" )?;
        stmt.execute( named_params!{ ":ca": c.ca, ":hours": c.hours,
                ":hsup": c.hsup, ":comment": c.comment, ":date": c.date } )?;
        Ok(())
    }

    /// add : enregistre le Ca dans la DB.
    /// - params :
    ///     - &self -> une référence sur la structure contenant l'url et la date
    ///     - c -> une structure Ca contenant les données a enregistrer
    /// - return : un objet () en cas de succés sinon l'erreur Sqlite3
    pub fn add(&self, c: &Ca) -> Result<()> {
        let conn = Connection::open(&self.url)?;

        let mut stmt = conn.prepare( "INSERT INTO CA (date, ca, hours, hsup, comment) VALUES
        (:date, :ca, :hours, :hsup, :comment)" )?;
        stmt.execute(
            named_params!{ ":date": c.date, ":ca": c.ca, ":hours": c.hours, ":hsup": c.hsup, ":comment": c.comment }
        )?;
        Ok(())
    }


    /// datas_for_month : récupère les données du mois choisi.
    /// - params :
    ///     - &self -> une référence sur la structure contenant l'url et la date
    ///     - d -> une référence sur une date qui servira dans la requete d'interrogation de la DB
    /// - return : un Vecteur contenant des objet Ca en cas de succès sinon l'erreur Sqlite3
    pub fn datas_for_month(&self, d: &NaiveDate) -> Result<Vec<Ca>> {
        let conn = Connection::open(&self.url)?;

        let mut stmt = conn.prepare( "SELECT * FROM CA WHERE date LIKE :month ORDER BY date" )?;
        let rows = stmt.query_map(&[(":month", d.format("%Y-%m%%").to_string().as_str() )], |row| {
            Ok(Ca {
                date: row.get("date")?,
                ca: row.get("ca")?,
                hours: row.get(2)?,
                hsup: row.get(3)?,
                comment: row.get("comment")?,
            })
        })?;
        let mut ca_tab = Vec::new();
        for ca in rows {
            ca_tab.push( ca? );
        }
        Ok(ca_tab)
    }

    /// ca_for_mount : récupère le chiffre d'affaire du mois choisi.
    /// - params :
    ///     - &self -> une référence sur la structure contenant l'url et la date
    ///     - d -> une référence sur une date qui servira dans la requete d'interrogation de la DB
    /// - return : un float contenant la valeur du chiffre d'affaire en cas de succès
    /// sinon l'erreur Sqlite3
    pub fn ca_for_month(&self, d : &NaiveDate ) -> Result<f64> {
        let conn = Connection::open(&self.url)?;

        let ca: f64 = conn.query_row( "SELECT SUM(ca) FROM CA WHERE date LIKE :month",
            &[ (":month", d.format("%Y-%m%%").to_string().as_str() )],
            |r| r.get(0) )?;
        Ok(ca)
    }

    /// hours_for_month : récupère le nombre d'heures du mois choisi.
    /// - params :
    ///     - &self -> une référence sur la structure contenant l'url et la date
    ///     - d -> une référence sur une date qui servira dans la requete d'interrogation de la DB
    /// - return : un float contenant la valeur du nombre d'heures en cas de succès
    /// sinon l'erreur Sqlite3
    pub fn hours_for_month(&self, d: &NaiveDate) -> Result<f64> {
        let conn = Connection::open(&self.url)?;

        let hours: f64 = conn.query_row( "SELECT SUM(hours) FROM CA WHERE date LIKE :month",
            &[ (":month", d.format("%Y-%m%%").to_string().as_str() )],
            |r| r.get(0) )?;
        Ok(hours)
    }

    /// hsup_for_month : récupère le nombre d'heures supplémentaires du mois choisi.
    /// - params :
    ///     - &self -> une référence sur la structure contenant l'url et la date
    ///     - d -> une référence sur une date qui servira dans la requete d'interrogation de la DB
    /// - return : un float contenant la valeur du nombre d'heures supplémentaires en cas de succès
    /// sinon l'erreur Sqlite3
    pub fn hsup_for_month(&self, d: &NaiveDate ) -> Result<f64> {
        let conn = Connection::open(&self.url)?;

        let hsup: f64 = conn.query_row( "SELECT SUM(hsup) FROM CA WHERE date LIKE :month",
            &[ (":month", d.format("%Y-%m%%").to_string().as_str() )],
            |r| r.get(0) )?;
        Ok(hsup)
    }
}
