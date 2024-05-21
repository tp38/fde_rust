//! # fde :
//! un programe pour suivre l'activité de Nat
//! - author : Thierry Probst <thierry.probst@free.fr>
//! - version : 1.0.0
//! - date : 23/04/2023

use std::env;
use chrono::{Local, NaiveDate};
use dialog::Dialog;
use camonth::CaMonth;
use cadaosqlite::CaDaoSqlite;
use ca::Ca;

mod ca;
mod cadaosqlite;
mod dialog;
mod camonth;

const AUTHOR: &str = "Thierry Probst <thierry.probst@free.fr>";
const VERSION: &str = "1.0.0";
const VDATE: &str = "23/04/2023";
const DB_URL: &str = "./data/fildeclair.sq3";


/// main : la fonction principale qui lit le contenu de la ligne de commande et redirige l'exécution
/// vers la bonne fonction. Actuellement les options supportées sont :
/// 1. l'affichage d'un écran d'aide
/// 2. l'affichage des données d'un mois particulier (indiqué en parametre)

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let today = Local::now().date_naive();

    for arg in args.iter() {
        match arg.as_str() {
            x if x.contains("fde") => Dialog::show_welcome(),
            "--help" | "-h" => Dialog::show_help(),
            x if x.contains("--month") | x.contains("-m") => {
                match NaiveDate::parse_from_str( extract_date(&x), "%d/%m/%Y" ) {
                    Ok(x) => get_month( x ),
                    Err(_) => get_month( today ),
                }
            }
            x if x.contains("--day") | x.contains("-d") => {
                match NaiveDate::parse_from_str( extract_date(&x), "%d/%m/%Y" ) {
                    Ok(x) => process_day( x ),
                    Err(_) => println!("erreur dans NaiveDate::parse_from_str ..."),
                }
            }
            _ => println!("commande inconnue! fin du programme ..."),
        }
    }
}

/// process_day : récupère et affiche les informations liées à un jour particulier (représenté par
/// une date)
/// - params : day -> un jour qui servira de base pour déterminer le mois choisi
/// - return : un Result vide sinon les Erreurs sqlite
fn process_day(day: NaiveDate) -> () {
    let mut c = Ca::new( &day );
    loop {
        match Dialog::menu_ca( &c ).as_str() { // on affiche le menu
            "d" => { c.delete(); break; }, // delete
            "m" => { c = Dialog::dialog_ca( c ); c.save(); break; }, // modify
            "s" => { c.save(); break; }, // save
            "q" => break,
            _ => println!( "option inconnue" ),
        }

    }
}

/// get_month : récupère et affiche les informations liées à un mois particulier (représenté par
/// une date)
/// - params : day -> un jour qui servira de base pour déterminer le mois choisi
/// - return : valeur de retour vide
fn get_month(day: NaiveDate) -> () {
    let mut cmonth = CaMonth{ day: day, ca: 0.0, hours:0.0, hsup: 0.0, datas: Vec::new() };
    match cmonth.retrieve_datas( DB_URL.to_string()) {
        Ok(()) => Dialog::show_month( cmonth ),
        Err(e) => Dialog::something_goes_wrong( "CaMonth::retrieve_datas", e),
    }
}

/// extract_date : sert à trouver la chaine (dd/mm/yyyy) située dérière le signe '=' dans
/// l'argument
/// - params : arg -> chaine représentant l'argument ( --month=dd/mm/yyyy ou -m=dd/mm/yyyy )
/// - return : une sous chaine de arg
fn extract_date( arg: &str ) -> &str {
    let octets = arg.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b'=' { let i = i + 1; return &arg[i..]; }
    }
    return &arg[..];
}
